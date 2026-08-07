"""Check and refresh penta's optional local Magic reference cache."""

from __future__ import annotations

import argparse
import fcntl
import gzip
import hashlib
import html
import json
import math
import os
import re
import socket
import sqlite3
import stat
import subprocess
import sys
import tempfile
import time
import unicodedata
import uuid
from collections.abc import Iterable, Iterator
from contextlib import contextmanager
from dataclasses import dataclass, field
from datetime import datetime, timedelta, timezone
from html.parser import HTMLParser
from pathlib import Path
from typing import Any, ClassVar
from urllib.error import HTTPError, URLError
from urllib.parse import quote, urljoin, urlsplit, urlunsplit
from urllib.request import HTTPRedirectHandler, Request, build_opener

USER_AGENT = "penta-reference-fetcher/1.0 (repository-local development tool)"
WIZARDS_RULES_HUB = "https://magic.wizards.com/en/rules"
ETERNAL_CENTRAL_PAGE = "https://www.eternalcentral.com/9394rules/"
ETERNAL_CENTRAL_API = (
    "https://www.eternalcentral.com/wp-json/wp/v2/pages?slug=9394rules"
)
SCRYFALL_BULK_API = "https://api.scryfall.com/bulk-data"
CACHE_LAYOUT_VERSION = 1
CACHE_DIR_NAME = "penta-reference-cache"
CACHE_ENV_VAR = "PENTA_REFERENCE_CACHE_DIR"
CACHE_GIT_CONFIG = "penta.referenceDir"
LEGACY_REFERENCE_DIR = Path("docs/reference")
LEGACY_LOCK_NAME = ".refresh-magic-references.lock"
MANIFEST_NAME = "manifest.json"
LOCK_NAME = "refresh.lock"
LOCK_METADATA_VERSION = 1
LOCK_TOOL_VERSION = 2
SCHEMA_VERSION = 1
DATABASE_NAME = "scryfall.sqlite"
DATABASE_RESOURCE_NAME = "scryfall-index"
# Bump this whenever tables, columns, indexes, or indexed semantics change.
DATABASE_SCHEMA_VERSION = 1
DATABASE_MANIFEST_KEY = f"{DATABASE_RESOURCE_NAME}-schema-v{DATABASE_SCHEMA_VERSION}"
DATABASE_RELATIVE_PATH = f"indexes/schema-v{DATABASE_SCHEMA_VERSION}/{DATABASE_NAME}"
DATABASE_INPUTS = ("oracle-cards", "rulings")
DATABASE_TABLES = (
    "cards",
    "card_faces",
    "card_names",
    "card_keywords",
    "card_colors",
    "card_parts",
    "rulings",
)
DATABASE_FTS_TABLES = ("card_search", "ruling_search")
MAX_SMALL_RESPONSE_BYTES = 8 * 1024 * 1024
MAX_BULK_BYTES = 2 * 1024 * 1024 * 1024
MAX_JSONL_LINE_BYTES = 4 * 1024 * 1024
MAX_JSONL_RECORDS = 10_000_000
MAX_GZIP_EXPANSION_RATIO = 50

DEFAULT_RESOURCES = (
    "comprehensive-rules",
    "eternal-central-rules",
    "oracle-cards",
    "rulings",
)

RESOURCE_FILES = {
    "comprehensive-rules": "sources/magic-comprehensive-rules.txt",
    "eternal-central-rules": "sources/eternal-central-93-94-rules.txt",
    "oracle-cards": "sources/scryfall-oracle-cards.jsonl.gz",
    "rulings": "sources/scryfall-rulings.jsonl.gz",
    "default-cards": "sources/scryfall-default-cards.jsonl.gz",
    "all-cards": "sources/scryfall-all-cards.jsonl.gz",
}

SCRYFALL_TYPES = {
    "oracle-cards": "oracle_cards",
    "rulings": "rulings",
    "default-cards": "default_cards",
    "all-cards": "all_cards",
}

BAD_STATUSES = {"missing", "untracked", "corrupt", "stale"}
USABLE_STATUSES = {"current", "fresh"}


class ReferenceError(RuntimeError):
    """Raised when a source or local cache does not satisfy its contract."""


@dataclass(frozen=True)
class RemoteResource:
    name: str
    source_url: str
    source_updated_at: str | None
    expected_size: int | None
    content: bytes | None = None
    metadata: dict[str, Any] = field(default_factory=dict)


@dataclass(frozen=True)
class ResourceStatus:
    name: str
    status: str
    path: str
    detail: str
    local_source_updated_at: str | None = None
    remote_source_updated_at: str | None = None

    def as_dict(self) -> dict[str, Any]:
        return {
            "name": self.name,
            "status": self.status,
            "path": self.path,
            "detail": self.detail,
            "local_source_updated_at": self.local_source_updated_at,
            "remote_source_updated_at": self.remote_source_updated_at,
        }


class LinkCollector(HTMLParser):
    def __init__(self) -> None:
        super().__init__(convert_charrefs=True)
        self.links: list[str] = []

    def handle_starttag(self, tag: str, attrs: list[tuple[str, str | None]]) -> None:
        if tag.lower() != "a":
            return
        href = dict(attrs).get("href")
        if href:
            self.links.append(href)


class SearchableText(HTMLParser):
    BLOCK_TAGS: ClassVar[frozenset[str]] = frozenset(
        {
            "article",
            "blockquote",
            "br",
            "div",
            "h1",
            "h2",
            "h3",
            "h4",
            "h5",
            "h6",
            "li",
            "ol",
            "p",
            "pre",
            "section",
            "table",
            "td",
            "th",
            "tr",
            "ul",
        }
    )

    def __init__(self) -> None:
        super().__init__(convert_charrefs=True)
        self.parts: list[str] = []
        self.skip_depth = 0

    def handle_starttag(self, tag: str, attrs: list[tuple[str, str | None]]) -> None:
        del attrs
        tag = tag.lower()
        if tag in {"script", "style"}:
            self.skip_depth += 1
        elif not self.skip_depth and tag in self.BLOCK_TAGS:
            self.parts.append("\n")

    def handle_endtag(self, tag: str) -> None:
        tag = tag.lower()
        if tag in {"script", "style"} and self.skip_depth:
            self.skip_depth -= 1
        elif not self.skip_depth and tag in self.BLOCK_TAGS:
            self.parts.append("\n")

    def handle_data(self, data: str) -> None:
        if not self.skip_depth:
            self.parts.append(data)

    def normalized(self) -> str:
        text = "".join(self.parts).replace("\xa0", " ")
        text = re.sub(r"\[/?vc_[^\]]*\]", "", text)
        lines: list[str] = []
        for raw_line in text.splitlines():
            line = re.sub(r"[ \t\r\f\v]+", " ", raw_line).strip()
            if line and (not lines or line != lines[-1]):
                lines.append(line)
        return "\n".join(lines)


def utc_now() -> datetime:
    return datetime.now(timezone.utc)


def iso_now() -> str:
    return utc_now().replace(microsecond=0).isoformat().replace("+00:00", "Z")


def parse_timestamp(value: str | None) -> datetime | None:
    if not isinstance(value, str) or not value:
        return None
    normalized = value.strip()
    if normalized.endswith("Z"):
        normalized = normalized[:-1] + "+00:00"
    try:
        parsed = datetime.fromisoformat(normalized)
    except ValueError:
        return None
    if parsed.tzinfo is None:
        parsed = parsed.replace(tzinfo=timezone.utc)
    return parsed.astimezone(timezone.utc)


def normalize_url(url: str) -> str:
    try:
        parts = urlsplit(url)
    except (TypeError, ValueError) as error:
        raise ReferenceError(f"invalid source URL: {url!r}") from error
    return urlunsplit(
        (
            parts.scheme,
            parts.netloc,
            quote(parts.path, safe="/%"),
            parts.query,
            parts.fragment,
        )
    )


def require_https_host(url: str, *allowed_hosts: str) -> None:
    if not isinstance(url, str):
        raise ReferenceError(f"source URL is not a string: {url!r}")
    try:
        parts = urlsplit(url)
    except ValueError as error:
        raise ReferenceError(f"invalid source URL: {url!r}") from error
    hostname = (parts.hostname or "").lower()
    allowed = any(
        hostname == host or hostname.endswith(f".{host}") for host in allowed_hosts
    )
    if parts.scheme != "https" or not allowed:
        raise ReferenceError(f"refusing unexpected source URL: {url}")


class SafeRedirectHandler(HTTPRedirectHandler):
    def __init__(self, allowed_hosts: tuple[str, ...]) -> None:
        super().__init__()
        self.allowed_hosts = allowed_hosts

    def redirect_request(
        self,
        request: Request,
        file_pointer: Any,
        code: int,
        message: str,
        headers: Any,
        new_url: str,
    ) -> Request | None:
        resolved_url = normalize_url(urljoin(request.full_url, new_url))
        require_https_host(resolved_url, *self.allowed_hosts)
        return super().redirect_request(
            request, file_pointer, code, message, headers, resolved_url
        )


def request(
    url: str,
    *,
    accept: str,
    timeout: float,
    allowed_hosts: tuple[str, ...],
) -> Any:
    require_https_host(url, *allowed_hosts)
    headers = {"User-Agent": USER_AGENT, "Accept": accept}
    opener = build_opener(SafeRedirectHandler(allowed_hosts))
    response = opener.open(
        Request(normalize_url(url), headers=headers), timeout=timeout
    )
    try:
        require_https_host(response.geturl(), *allowed_hosts)
    except Exception:
        response.close()
        raise
    return response


def request_bytes(
    url: str,
    *,
    accept: str,
    timeout: float,
    allowed_hosts: tuple[str, ...],
    max_bytes: int = MAX_SMALL_RESPONSE_BYTES,
) -> tuple[bytes, str]:
    with request(
        url, accept=accept, timeout=timeout, allowed_hosts=allowed_hosts
    ) as response:
        content_length = response.headers.get("Content-Length")
        if content_length:
            try:
                if int(content_length) > max_bytes:
                    raise ReferenceError(
                        f"response from {url} exceeds the {max_bytes}-byte limit"
                    )
            except ValueError:
                pass
        content = bytearray()
        while True:
            remaining = max_bytes - len(content)
            chunk = response.read(min(64 * 1024, remaining + 1))
            if not chunk:
                break
            content.extend(chunk)
            if len(content) > max_bytes:
                raise ReferenceError(
                    f"response from {url} exceeds the {max_bytes}-byte limit"
                )
        return bytes(content), response.geturl()


def sha256_bytes(content: bytes) -> str:
    return hashlib.sha256(content).hexdigest()


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as source:
        for chunk in iter(lambda: source.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def find_repo_root(explicit: str | None) -> Path:
    if explicit:
        candidate = Path(explicit).expanduser().resolve()
        if not (candidate / ".git").exists():
            raise ReferenceError(f"not a Git repository root: {candidate}")
        return candidate

    current = Path.cwd().resolve()
    for candidate in (current, *current.parents):
        if (candidate / ".git").exists():
            return candidate

    skill_repo = Path(__file__).resolve().parents[4]
    if (skill_repo / ".git").exists():
        return skill_repo
    raise ReferenceError("could not locate the repository root; pass --repo-root")


def git_output(
    repo_root: Path, *arguments: str, allow_missing: bool = False
) -> str | None:
    try:
        result = subprocess.run(
            ("git", "-C", str(repo_root), *arguments),
            check=False,
            capture_output=True,
            text=True,
            timeout=10,
        )
    except (OSError, subprocess.TimeoutExpired) as error:
        raise ReferenceError(
            f"cannot run Git to locate the shared cache: {error}"
        ) from error
    if allow_missing and result.returncode == 1 and not result.stdout.strip():
        return None
    if result.returncode != 0:
        detail = result.stderr.strip() or f"Git exited {result.returncode}"
        raise ReferenceError(f"cannot locate the shared cache: {detail}")
    value = result.stdout.strip()
    if not value:
        if allow_missing:
            return None
        raise ReferenceError("Git returned an empty path for the shared cache")
    return value


def git_common_dir(repo_root: Path) -> Path:
    value = git_output(
        repo_root,
        "rev-parse",
        "--path-format=absolute",
        "--git-common-dir",
    )
    assert value is not None
    return Path(value).resolve()


def resolve_configured_path(value: str, *, base: Path) -> Path:
    path = Path(value).expanduser()
    if not path.is_absolute():
        path = base / path
    return path.resolve()


def resolve_reference_dir(repo_root: Path, explicit: str | None) -> Path:
    if explicit:
        return resolve_configured_path(explicit, base=repo_root)
    common_dir = git_common_dir(repo_root)
    environment_value = os.environ.get(CACHE_ENV_VAR)
    if environment_value:
        return resolve_configured_path(environment_value, base=common_dir)
    configured = git_output(
        repo_root,
        "config",
        "--path",
        "--get",
        CACHE_GIT_CONFIG,
        allow_missing=True,
    )
    if configured:
        return resolve_configured_path(configured, base=common_dir)
    return common_dir / CACHE_DIR_NAME / f"layout-v{CACHE_LAYOUT_VERSION}"


def load_manifest(reference_dir: Path) -> dict[str, Any]:
    path = reference_dir / MANIFEST_NAME
    if not path.exists():
        return {"schema_version": SCHEMA_VERSION, "resources": {}, "derived": {}}
    try:
        manifest = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, UnicodeError, json.JSONDecodeError) as error:
        raise ReferenceError(f"cannot read {path}: {error}") from error
    if not isinstance(manifest, dict):
        raise ReferenceError(f"manifest must contain a JSON object: {path}")
    if manifest.get("schema_version") != SCHEMA_VERSION:
        raise ReferenceError(
            f"unsupported manifest schema {manifest.get('schema_version')!r} in {path}"
        )
    if not isinstance(manifest.get("resources"), dict):
        raise ReferenceError(f"manifest has no resources object: {path}")
    if "derived" not in manifest:
        manifest["derived"] = {}
    if not isinstance(manifest["derived"], dict):
        raise ReferenceError(f"manifest has no derived object: {path}")
    return manifest


def write_manifest(reference_dir: Path, manifest: dict[str, Any]) -> None:
    manifest["schema_version"] = SCHEMA_VERSION
    manifest["generated_at"] = iso_now()
    payload = (json.dumps(manifest, indent=2, sort_keys=True) + "\n").encode()
    atomic_write(reference_dir / MANIFEST_NAME, payload)


def atomic_write(path: Path, content: bytes) -> tuple[int, str]:
    path.parent.mkdir(parents=True, exist_ok=True)
    temporary: Path | None = None
    try:
        with tempfile.NamedTemporaryFile(
            mode="wb",
            dir=path.parent,
            prefix=f".{path.name}.",
            suffix=".tmp",
            delete=False,
        ) as target:
            temporary = Path(target.name)
            target.write(content)
            target.flush()
            os.fsync(target.fileno())
        os.replace(temporary, path)
        return len(content), sha256_bytes(content)
    except Exception:
        if temporary is not None:
            temporary.unlink(missing_ok=True)
        raise


def ensure_safe_cache_parent(reference_dir: Path, relative_path: str) -> Path:
    relative = Path(relative_path)
    if relative.is_absolute() or ".." in relative.parts:
        raise ReferenceError(f"invalid cache-relative path: {relative_path!r}")
    current = reference_dir
    for part in relative.parent.parts:
        current = current / part
        try:
            info = os.lstat(current)
        except FileNotFoundError:
            os.mkdir(current, 0o700)
            info = os.lstat(current)
        if stat.S_ISLNK(info.st_mode) or not stat.S_ISDIR(info.st_mode):
            raise ReferenceError(
                f"cache path component is not a real directory: {current}"
            )
    return reference_dir / relative


def validate_safe_cache_parent(reference_dir: Path, relative_path: str) -> Path:
    relative = Path(relative_path)
    if relative.is_absolute() or ".." in relative.parts:
        raise ReferenceError(f"invalid cache-relative path: {relative_path!r}")
    current = reference_dir
    for part in relative.parent.parts:
        current = current / part
        try:
            info = os.lstat(current)
        except FileNotFoundError:
            break
        if stat.S_ISLNK(info.st_mode) or not stat.S_ISDIR(info.st_mode):
            raise ReferenceError(
                f"cache path component is not a real directory: {current}"
            )
    return reference_dir / relative


MAX_LOCK_HISTORY_BYTES = 4 * 1024 * 1024


def open_lock_file(
    lock_path: Path, *, exclusive: bool, create: bool, binary: bool = False
) -> Any:
    flags = os.O_RDWR | os.O_APPEND if exclusive else os.O_RDONLY
    if create:
        flags |= os.O_CREAT
    flags |= getattr(os, "O_CLOEXEC", 0)
    flags |= getattr(os, "O_NOFOLLOW", 0)
    descriptor = os.open(lock_path, flags, 0o600)
    try:
        info = os.fstat(descriptor)
        if not stat.S_ISREG(info.st_mode):
            raise ReferenceError(f"cache lock is not a regular file: {lock_path}")
        if exclusive and hasattr(os, "getuid") and info.st_uid != os.getuid():
            raise ReferenceError(
                f"refusing to write cache lock owned by uid {info.st_uid}: {lock_path}"
            )
        mode = "a+" if exclusive else "r"
        if binary:
            mode += "b"
            return os.fdopen(descriptor, mode)
        return os.fdopen(descriptor, mode, encoding="utf-8")
    except BaseException:
        os.close(descriptor)
        raise


def read_lock_events(lock_path: Path) -> tuple[list[dict[str, Any]], list[str]]:
    """Read complete recent NDJSON events without modifying the lock file."""

    try:
        with open_lock_file(
            lock_path, exclusive=False, create=False, binary=True
        ) as lock_file:
            size = lock_file.seek(0, os.SEEK_END)
            start = max(0, size - MAX_LOCK_HISTORY_BYTES)
            lock_file.seek(start)
            payload = lock_file.read(MAX_LOCK_HISTORY_BYTES)
    except FileNotFoundError:
        return [], []
    except (OSError, UnicodeError, ReferenceError) as error:
        return [], [f"lock history is unreadable: {error}"]

    if start:
        first_newline = payload.find(b"\n")
        payload = b"" if first_newline < 0 else payload[first_newline + 1 :]
    if payload and not payload.endswith(b"\n"):
        last_newline = payload.rfind(b"\n")
        payload = b"" if last_newline < 0 else payload[: last_newline + 1]

    events: list[dict[str, Any]] = []
    problems: list[str] = []
    for line_number, raw_line in enumerate(payload.splitlines(), start=1):
        if not raw_line.strip():
            continue
        try:
            event = json.loads(raw_line)
        except (UnicodeDecodeError, json.JSONDecodeError) as error:
            if len(problems) < 5:
                problems.append(
                    f"invalid event near retained line {line_number}: {error}"
                )
            continue
        if not isinstance(event, dict):
            if len(problems) < 5:
                problems.append(
                    f"invalid event near retained line {line_number}: not an object"
                )
            continue
        if event.get("protocol_version") != LOCK_METADATA_VERSION:
            if len(problems) < 5:
                problems.append(
                    f"unsupported lock protocol near retained line {line_number}"
                )
            continue
        events.append(event)
    return events, problems


def latest_lock_owner(events: Iterable[dict[str, Any]]) -> dict[str, Any] | None:
    owner: dict[str, Any] | None = None
    for event in events:
        if event.get("event") == "acquired":
            owner = dict(event)
            owner["released"] = False
        elif owner is not None and event.get("owner_id") == owner.get("owner_id"):
            if event.get("event") == "progress":
                owner["phase"] = event.get("phase", owner.get("phase"))
                owner["updated_at"] = event.get("at")
                if "detail" in event:
                    owner["detail"] = event["detail"]
            elif event.get("event") == "released":
                owner["released"] = True
                owner["outcome"] = event.get("outcome")
                owner["released_at"] = event.get("at")
                owner["held_seconds"] = event.get("held_seconds")
                if "error" in event:
                    owner["error"] = event["error"]
    return owner


def append_lock_event(lock_file: Any, event: dict[str, Any]) -> None:
    payload = (
        json.dumps(event, ensure_ascii=False, separators=(",", ":"), sort_keys=True)
        + "\n"
    ).encode("utf-8")
    written = 0
    while written < len(payload):
        written += os.write(lock_file.fileno(), payload[written:])
    os.fsync(lock_file.fileno())


def ensure_lock_event_boundary(lock_file: Any) -> None:
    size = os.fstat(lock_file.fileno()).st_size
    if size and os.pread(lock_file.fileno(), 1, size - 1) != b"\n":
        os.write(lock_file.fileno(), b"\n")
        os.fsync(lock_file.fileno())


def lock_owner_summary(owner: dict[str, Any] | None) -> str:
    if not owner:
        return "no owner metadata is available"
    fields = (
        ("operation", owner.get("operation")),
        ("phase", owner.get("phase")),
        ("pid", owner.get("pid")),
        ("host", owner.get("hostname")),
        ("acquired", owner.get("at")),
        ("updated", owner.get("updated_at")),
        ("worktree", owner.get("worktree")),
    )
    return ", ".join(f"{name}={value}" for name, value in fields if value is not None)


def process_start_identity(pid: int) -> str | None:
    try:
        result = subprocess.run(
            ("ps", "-o", "lstart=", "-p", str(pid)),
            check=False,
            capture_output=True,
            text=True,
            timeout=2,
        )
    except (OSError, subprocess.TimeoutExpired):
        return None
    value = " ".join(result.stdout.split())
    return value or None


def same_host_process_state(owner: dict[str, Any] | None) -> str | None:
    if not owner or owner.get("hostname") != socket.gethostname():
        return None
    pid = owner.get("pid")
    if type(pid) is not int or pid <= 0:
        return "unverifiable"
    try:
        os.kill(pid, 0)
    except ProcessLookupError:
        return "absent"
    except PermissionError:
        return "alive-unverifiable"
    recorded_start = owner.get("process_started_at")
    current_start = process_start_identity(pid)
    if isinstance(recorded_start, str) and current_start:
        return "alive-matching" if recorded_start == current_start else "pid-reused"
    return "alive-unverifiable"


def current_git_head(repo_root: Path) -> str | None:
    try:
        return git_output(repo_root, "rev-parse", "--verify", "HEAD")
    except ReferenceError:
        return None


@dataclass
class CacheLock:
    lock_file: Any | None
    owner_id: str | None
    exclusive: bool

    def update(self, phase: str, **detail: Any) -> None:
        if not self.exclusive or self.lock_file is None or self.owner_id is None:
            return
        event: dict[str, Any] = {
            "protocol_version": LOCK_METADATA_VERSION,
            "tool_version": LOCK_TOOL_VERSION,
            "event": "progress",
            "owner_id": self.owner_id,
            "at": iso_now(),
            "phase": phase,
        }
        if detail:
            event["detail"] = detail
        append_lock_event(self.lock_file, event)


def acquire_flock(
    lock_file: Any,
    *,
    exclusive: bool,
    timeout: float,
    lock_path: Path,
) -> None:
    mode = fcntl.LOCK_EX if exclusive else fcntl.LOCK_SH
    deadline = time.monotonic() + timeout
    while True:
        try:
            fcntl.flock(lock_file.fileno(), mode | fcntl.LOCK_NB)
            return
        except BlockingIOError:
            if time.monotonic() >= deadline:
                events, problems = read_lock_events(lock_path)
                owner = latest_lock_owner(events)
                if owner is not None and not owner.get("released"):
                    diagnostic = lock_owner_summary(owner)
                elif owner is not None:
                    diagnostic = (
                        "the active lock has no unreleased owner metadata; last "
                        f"completed writer: {lock_owner_summary(owner)}"
                    )
                else:
                    diagnostic = "the active lock has no owner metadata"
                if problems:
                    diagnostic += f"; metadata warning: {problems[-1]}"
                raise ReferenceError(
                    f"timed out after {timeout:g}s waiting for the shared cache lock; "
                    f"{diagnostic}. Run 'lock-status' for diagnostics; never delete "
                    "refresh.lock based only on its metadata"
                ) from None
            time.sleep(min(0.2, max(0.01, deadline - time.monotonic())))


@contextmanager
def cache_lock(
    reference_dir: Path,
    *,
    repo_root: Path,
    operation: str,
    exclusive: bool,
    timeout: float,
    details: dict[str, Any] | None = None,
) -> Iterator[CacheLock]:
    """Coordinate shared-cache access with append-only diagnostic events."""

    lock_path = reference_dir / LOCK_NAME
    if not exclusive and not lock_path.exists():
        if any(
            (reference_dir / name).exists()
            for name in (MANIFEST_NAME, "sources", "indexes")
        ):
            raise ReferenceError(
                f"shared cache contains data but has no persistent lock: {lock_path}"
            )
        yield CacheLock(None, None, False)
        if lock_path.exists():
            raise ReferenceError(
                "shared cache initialization overlapped this read; retry the command"
            )
        return
    if exclusive:
        reference_dir.mkdir(parents=True, exist_ok=True)
        directory_info = os.lstat(reference_dir)
        if not stat.S_ISDIR(directory_info.st_mode) or stat.S_ISLNK(
            directory_info.st_mode
        ):
            raise ReferenceError(
                f"shared cache root is not a real directory: {reference_dir}"
            )
    try:
        lock_file_context = open_lock_file(
            lock_path, exclusive=exclusive, create=exclusive
        )
    except FileNotFoundError:
        yield CacheLock(None, None, False)
        return

    with lock_file_context as lock_file:
        acquire_flock(
            lock_file,
            exclusive=exclusive,
            timeout=timeout,
            lock_path=lock_path,
        )
        if exclusive:
            os.fchmod(lock_file.fileno(), 0o600)
            ensure_lock_event_boundary(lock_file)
        owner_id: str | None = None
        started = time.monotonic()
        handle = CacheLock(lock_file, None, exclusive)
        if exclusive:
            owner_id = uuid.uuid4().hex
            event: dict[str, Any] = {
                "protocol_version": LOCK_METADATA_VERSION,
                "tool_version": LOCK_TOOL_VERSION,
                "event": "acquired",
                "owner_id": owner_id,
                "at": iso_now(),
                "operation": operation,
                "phase": "starting",
                "pid": os.getpid(),
                "ppid": os.getppid(),
                "hostname": socket.gethostname(),
                "worktree": str(repo_root),
                "cache_dir": str(reference_dir),
                "script": str(Path(__file__).resolve()),
                "python": sys.executable,
                "process_started_at": process_start_identity(os.getpid()),
                "git_head": current_git_head(repo_root),
            }
            if hasattr(os, "getuid"):
                event["uid"] = os.getuid()
            if details:
                event["details"] = details
            append_lock_event(lock_file, event)
            handle.owner_id = owner_id
        try:
            yield handle
        except BaseException as error:
            if owner_id is not None:
                append_lock_event(
                    lock_file,
                    {
                        "protocol_version": LOCK_METADATA_VERSION,
                        "tool_version": LOCK_TOOL_VERSION,
                        "event": "released",
                        "owner_id": owner_id,
                        "at": iso_now(),
                        "outcome": "error",
                        "held_seconds": round(time.monotonic() - started, 3),
                        "error": {
                            "type": type(error).__name__,
                            "message": str(error)[:500],
                        },
                    },
                )
            raise
        else:
            if owner_id is not None:
                append_lock_event(
                    lock_file,
                    {
                        "protocol_version": LOCK_METADATA_VERSION,
                        "tool_version": LOCK_TOOL_VERSION,
                        "event": "released",
                        "owner_id": owner_id,
                        "at": iso_now(),
                        "outcome": "success",
                        "held_seconds": round(time.monotonic() - started, 3),
                    },
                )
        finally:
            fcntl.flock(lock_file.fileno(), fcntl.LOCK_UN)


@contextmanager
def legacy_cache_lock(legacy_dir: Path, *, timeout: float) -> Iterator[None]:
    """Exclude old-version refreshers while a legacy cache is migrated."""

    legacy_dir.mkdir(parents=True, exist_ok=True)
    lock_path = legacy_dir / LEGACY_LOCK_NAME
    with open_lock_file(lock_path, exclusive=True, create=True) as lock_file:
        acquire_flock(
            lock_file,
            exclusive=True,
            timeout=timeout,
            lock_path=lock_path,
        )
        os.fchmod(lock_file.fileno(), 0o600)
        try:
            yield
        finally:
            fcntl.flock(lock_file.fileno(), fcntl.LOCK_UN)


def atomic_download(
    remote: RemoteResource, destination: Path, *, timeout: float
) -> tuple[int, str, dict[str, Any]]:
    if remote.content is not None:
        size, digest = atomic_write(destination, remote.content)
        return size, digest, {}

    destination.parent.mkdir(parents=True, exist_ok=True)
    temporary: Path | None = None
    digest = hashlib.sha256()
    size = 0
    try:
        with tempfile.NamedTemporaryFile(
            mode="wb",
            dir=destination.parent,
            prefix=f".{destination.name}.",
            suffix=".tmp",
            delete=False,
        ) as target:
            temporary = Path(target.name)
            with request(
                remote.source_url,
                accept="application/gzip,application/octet-stream;q=0.9,*/*;q=0.8",
                timeout=timeout,
                allowed_hosts=("scryfall.io",),
            ) as response:
                while True:
                    chunk = response.read(1024 * 1024)
                    if not chunk:
                        break
                    target.write(chunk)
                    digest.update(chunk)
                    size += len(chunk)
                    if remote.expected_size is not None and size > remote.expected_size:
                        raise ReferenceError(
                            f"download for {remote.name} exceeded its advertised "
                            f"{remote.expected_size}-byte size"
                        )
            target.flush()
            os.fsync(target.fileno())

        if remote.expected_size is not None and size != remote.expected_size:
            raise ReferenceError(
                f"downloaded {size} bytes for {remote.name}; expected "
                f"{remote.expected_size}"
            )
        validation = validate_download(
            temporary, remote.name, compressed_size=remote.expected_size
        )
        os.replace(temporary, destination)
        return size, digest.hexdigest(), validation
    except Exception:
        if temporary is not None:
            temporary.unlink(missing_ok=True)
        raise


def validate_download(
    path: Path, name: str, *, compressed_size: int | None
) -> dict[str, Any]:
    record_count = sum(
        1 for _ in iter_scryfall_records(path, name, compressed_size=compressed_size)
    )
    return {"record_count": record_count}


def iter_scryfall_records(
    path: Path, name: str, *, compressed_size: int | None
) -> Iterator[dict[str, Any]]:
    if name not in SCRYFALL_TYPES:
        raise ReferenceError(f"{name} is not a Scryfall JSONL resource")
    expected_object = "ruling" if name == "rulings" else "card"
    record_count = 0
    uncompressed_size = 0
    max_uncompressed_size = (
        compressed_size * MAX_GZIP_EXPANSION_RATIO
        if compressed_size is not None
        else MAX_BULK_BYTES
    )
    try:
        with gzip.open(path, "rb") as source:
            line_number = 0
            while True:
                line = source.readline(MAX_JSONL_LINE_BYTES + 1)
                if not line:
                    break
                line_number += 1
                if len(line) > MAX_JSONL_LINE_BYTES:
                    raise ReferenceError(
                        f"oversized JSONL record in {name} at line {line_number}"
                    )
                uncompressed_size += len(line)
                if uncompressed_size > max_uncompressed_size:
                    raise ReferenceError(
                        f"decompressed {name} exceeds the safety limit"
                    )
                if not line.strip():
                    raise ReferenceError(
                        f"blank JSONL record in {name} at line {line_number}"
                    )
                item = json.loads(line)
                if not isinstance(item, dict) or item.get("object") != expected_object:
                    raise ReferenceError(
                        f"unexpected Scryfall object in {name} at line {line_number}"
                    )
                record_count += 1
                if record_count > MAX_JSONL_RECORDS:
                    raise ReferenceError(
                        f"{name} exceeds the record-count safety limit"
                    )
                yield item
    except (OSError, EOFError, UnicodeError, json.JSONDecodeError) as error:
        raise ReferenceError(
            f"invalid Scryfall JSONL gzip for {name}: {error}"
        ) from error
    if not record_count:
        raise ReferenceError(f"empty Scryfall JSONL gzip for {name}")


def parse_effective_date(content: bytes) -> str | None:
    text = content[:4096].decode("utf-8-sig", errors="replace")
    match = re.search(r"These rules are effective as of ([^.]+)\.", text)
    if not match:
        return None
    value = match.group(1).strip()
    try:
        return (
            datetime.strptime(value, "%B %d, %Y")
            .replace(tzinfo=timezone.utc)
            .date()
            .isoformat()
        )
    except ValueError:
        return value


def load_comprehensive_rules(timeout: float) -> RemoteResource:
    hub_content, hub_url = request_bytes(
        WIZARDS_RULES_HUB,
        accept="text/html,application/xhtml+xml",
        timeout=timeout,
        allowed_hosts=("wizards.com",),
    )
    parser = LinkCollector()
    parser.feed(hub_content.decode("utf-8", errors="replace"))
    candidates = []
    for link in parser.links:
        path = urlsplit(link).path.lower()
        if path.endswith(".txt") and "magiccomprules" in path.replace("%20", ""):
            candidates.append(normalize_url(urljoin(hub_url, link)))
    if not candidates:
        raise ReferenceError(f"no Comprehensive Rules TXT link found at {hub_url}")
    rules_url = candidates[-1]
    require_https_host(rules_url, "media.wizards.com")
    content, final_url = request_bytes(
        rules_url,
        accept="text/plain,*/*;q=0.8",
        timeout=timeout,
        allowed_hosts=("media.wizards.com",),
    )
    if b"Magic: The Gathering Comprehensive Rules" not in content[:4096]:
        raise ReferenceError(f"unexpected Comprehensive Rules content at {final_url}")
    effective_date = parse_effective_date(content)
    if not effective_date:
        raise ReferenceError(f"could not read the effective date from {final_url}")
    filename_date = re.search(
        r"MagicCompRules(?:%20| )?(\d{8})\.txt$", final_url, flags=re.IGNORECASE
    )
    if filename_date and filename_date.group(1) != effective_date.replace("-", ""):
        raise ReferenceError(
            f"Comprehensive Rules filename date does not match {effective_date}"
        )
    return RemoteResource(
        name="comprehensive-rules",
        source_url=final_url,
        source_updated_at=effective_date,
        expected_size=len(content),
        content=content,
        metadata={"source_index_url": WIZARDS_RULES_HUB, "format": "text/plain"},
    )


def load_eternal_central_rules(timeout: float) -> RemoteResource:
    payload, _ = request_bytes(
        ETERNAL_CENTRAL_API,
        accept="application/json",
        timeout=timeout,
        allowed_hosts=("eternalcentral.com",),
    )
    try:
        pages = json.loads(payload)
        page = pages[0]
        rendered = page["content"]["rendered"]
    except (
        json.JSONDecodeError,
        UnicodeError,
        IndexError,
        KeyError,
        TypeError,
    ) as error:
        raise ReferenceError(
            "unexpected Eternal Central WordPress API response"
        ) from error
    if not isinstance(rendered, str):
        raise ReferenceError("Eternal Central rendered content is not text")

    parser = SearchableText()
    parser.feed(rendered)
    body = parser.normalized()
    if "Old School 93-94 Rules" not in body or "Legal Sets" not in body:
        raise ReferenceError("Eternal Central rules extraction failed validation")

    title_record = page.get("title")
    if not isinstance(title_record, dict) or not isinstance(
        title_record.get("rendered"), str
    ):
        raise ReferenceError("Eternal Central title is not rendered text")
    title = html.unescape(title_record["rendered"])
    modified = page.get("modified_gmt") or page.get("modified")
    source_page = page.get("link") or ETERNAL_CENTRAL_PAGE
    if modified is not None and not isinstance(modified, str):
        raise ReferenceError("Eternal Central modified timestamp is not text")
    require_https_host(source_page, "eternalcentral.com")
    header = (
        f"{title}\n\n"
        f"Source: {source_page}\n"
        f"Source last modified: {modified or 'unknown'}\n\n"
    )
    content = (header + body + "\n").encode("utf-8")
    return RemoteResource(
        name="eternal-central-rules",
        source_url=source_page,
        source_updated_at=modified,
        expected_size=len(content),
        content=content,
        metadata={"source_api_url": ETERNAL_CENTRAL_API, "format": "text/plain"},
    )


def load_scryfall_index(timeout: float) -> dict[str, dict[str, Any]]:
    payload, _ = request_bytes(
        SCRYFALL_BULK_API,
        accept="application/json;q=0.9,*/*;q=0.8",
        timeout=timeout,
        allowed_hosts=("scryfall.com",),
    )
    try:
        response = json.loads(payload)
        rows = response["data"]
    except (json.JSONDecodeError, UnicodeError, KeyError, TypeError) as error:
        raise ReferenceError("unexpected Scryfall bulk-data response") from error
    if not isinstance(rows, list):
        raise ReferenceError("Scryfall bulk-data response has no data list")
    return {
        row["type"]: row
        for row in rows
        if isinstance(row, dict) and isinstance(row.get("type"), str)
    }


def load_scryfall_resource(
    name: str, *, timeout: float, cache: dict[str, Any]
) -> RemoteResource:
    if "scryfall_index" not in cache:
        cache["scryfall_index"] = load_scryfall_index(timeout)
    row = cache["scryfall_index"].get(SCRYFALL_TYPES[name])
    if not row:
        raise ReferenceError(f"Scryfall did not advertise {SCRYFALL_TYPES[name]}")
    download_url = row.get("jsonl_download_uri")
    if not isinstance(download_url, str) or not download_url:
        raise ReferenceError(f"Scryfall did not provide a JSONL download for {name}")
    require_https_host(download_url, "scryfall.io")
    size = row.get("compressed_size")
    if type(size) is not int or size <= 0 or size > MAX_BULK_BYTES:
        raise ReferenceError(f"Scryfall did not provide compressed_size for {name}")
    updated_at = row.get("updated_at")
    if parse_timestamp(updated_at) is None:
        raise ReferenceError(f"Scryfall did not provide a valid updated_at for {name}")
    return RemoteResource(
        name=name,
        source_url=download_url,
        source_updated_at=updated_at,
        expected_size=size,
        metadata={
            "source_index_url": SCRYFALL_BULK_API,
            "format": "application/jsonl+gzip",
            "scryfall_type": row.get("type"),
            "scryfall_id": row.get("id"),
        },
    )


def load_remote_resource(
    name: str, *, timeout: float, cache: dict[str, Any]
) -> RemoteResource:
    if name == "comprehensive-rules":
        return load_comprehensive_rules(timeout)
    if name == "eternal-central-rules":
        return load_eternal_central_rules(timeout)
    return load_scryfall_resource(name, timeout=timeout, cache=cache)


def local_problem(
    name: str, reference_dir: Path, record: dict[str, Any] | None
) -> tuple[str, str, str | None] | None:
    relative_path = RESOURCE_FILES[name]
    path = reference_dir / relative_path
    if not path.exists():
        return "missing", "local file does not exist", None
    if not record:
        return "untracked", "local file has no manifest record", None
    if not isinstance(record, dict):
        return "corrupt", "manifest resource record is not a JSON object", None
    if record.get("path") != relative_path:
        return "corrupt", "manifest path does not match the expected filename", None
    expected_size = record.get("size")
    actual_size = path.stat().st_size
    if expected_size != actual_size:
        return (
            "corrupt",
            f"local size is {actual_size} bytes; manifest records {expected_size}",
            record.get("source_updated_at"),
        )
    expected_sha = record.get("sha256")
    actual_sha = sha256_file(path)
    if expected_sha != actual_sha:
        return (
            "corrupt",
            "local SHA-256 does not match the manifest",
            record.get("source_updated_at"),
        )
    return None


def compact_json(value: Any) -> str:
    return json.dumps(value, ensure_ascii=False, separators=(",", ":"), sort_keys=True)


def normalized_card_name(value: str) -> str:
    return unicodedata.normalize("NFKC", value).casefold()


def required_text(item: dict[str, Any], key: str, *, context: str) -> str:
    value = item.get(key)
    if not isinstance(value, str) or not value:
        raise ReferenceError(f"{context} has no usable {key}")
    return value


def optional_text(item: dict[str, Any], key: str) -> str | None:
    value = item.get(key)
    return value if isinstance(value, str) else None


def nested_text(item: dict[str, Any], key: str, nested_key: str) -> str | None:
    value = item.get(key)
    if not isinstance(value, dict):
        return None
    nested = value.get(nested_key)
    return nested if isinstance(nested, str) else None


def database_input_records(
    reference_dir: Path, manifest: dict[str, Any]
) -> dict[str, dict[str, Any]]:
    records: dict[str, dict[str, Any]] = {}
    for name in DATABASE_INPUTS:
        record = manifest["resources"].get(name)
        problem = local_problem(name, reference_dir, record)
        if problem:
            status, detail, _ = problem
            raise ReferenceError(
                f"cannot build {DATABASE_NAME}: {name} is {status}: {detail}"
            )
        if not isinstance(record, dict):
            raise ReferenceError(f"cannot build {DATABASE_NAME}: invalid {name} record")
        if type(record.get("record_count")) is not int or record["record_count"] <= 0:
            raise ReferenceError(
                f"cannot build {DATABASE_NAME}: {name} has no validated record count"
            )
        records[name] = record
    return records


def inspect_scryfall_database(
    reference_dir: Path, manifest: dict[str, Any]
) -> ResourceStatus:
    relative_path = DATABASE_RELATIVE_PATH
    path = reference_dir / relative_path
    record = manifest["derived"].get(DATABASE_MANIFEST_KEY)
    built_at = record.get("built_at") if isinstance(record, dict) else None

    if not path.exists():
        return ResourceStatus(
            DATABASE_RESOURCE_NAME,
            "missing",
            relative_path,
            "derived SQLite database does not exist",
            built_at,
            None,
        )
    if not isinstance(record, dict):
        return ResourceStatus(
            DATABASE_RESOURCE_NAME,
            "untracked",
            relative_path,
            "database has no manifest record",
            built_at,
            None,
        )
    if record.get("path") != relative_path:
        return ResourceStatus(
            DATABASE_RESOURCE_NAME,
            "corrupt",
            relative_path,
            "manifest path does not match the database filename",
            built_at,
            None,
        )
    if record.get("schema_version") != DATABASE_SCHEMA_VERSION:
        return ResourceStatus(
            DATABASE_RESOURCE_NAME,
            "stale",
            relative_path,
            "database schema version has changed",
            built_at,
            None,
        )
    actual_size = path.stat().st_size
    if record.get("size") != actual_size:
        return ResourceStatus(
            DATABASE_RESOURCE_NAME,
            "corrupt",
            relative_path,
            f"database size is {actual_size} bytes; manifest records {record.get('size')}",
            built_at,
            None,
        )
    if record.get("sha256") != sha256_file(path):
        return ResourceStatus(
            DATABASE_RESOURCE_NAME,
            "corrupt",
            relative_path,
            "database SHA-256 does not match the manifest",
            built_at,
            None,
        )

    inputs = record.get("inputs")
    if not isinstance(inputs, dict):
        return ResourceStatus(
            DATABASE_RESOURCE_NAME,
            "corrupt",
            relative_path,
            "database manifest record has no inputs object",
            built_at,
            None,
        )
    for name in DATABASE_INPUTS:
        input_record = inputs.get(name)
        source_record = manifest["resources"].get(name)
        if not isinstance(input_record, dict) or not isinstance(source_record, dict):
            return ResourceStatus(
                DATABASE_RESOURCE_NAME,
                "stale",
                relative_path,
                f"cannot match the database to the cached {name} input",
                built_at,
                None,
            )
        if any(
            input_record.get(field) != source_record.get(field)
            for field in ("sha256", "record_count")
        ):
            return ResourceStatus(
                DATABASE_RESOURCE_NAME,
                "stale",
                relative_path,
                f"database was built from different {name} input metadata",
                built_at,
                None,
            )

    table_counts = record.get("table_counts")
    if not isinstance(table_counts, dict) or any(
        type(table_counts.get(table)) is not int or table_counts[table] < 0
        for table in DATABASE_TABLES
    ):
        return ResourceStatus(
            DATABASE_RESOURCE_NAME,
            "corrupt",
            relative_path,
            "database manifest record has invalid table counts",
            built_at,
            None,
        )
    manifest_fts5 = record.get("fts5")
    if type(manifest_fts5) is not bool:
        return ResourceStatus(
            DATABASE_RESOURCE_NAME,
            "corrupt",
            relative_path,
            "database manifest record has no valid FTS5 flag",
            built_at,
            None,
        )

    try:
        with sqlite3.connect(
            f"{path.resolve().as_uri()}?mode=ro", uri=True
        ) as database:
            database.execute("PRAGMA query_only = ON")
            user_version = database.execute("PRAGMA user_version").fetchone()[0]
            if user_version != DATABASE_SCHEMA_VERSION:
                raise ReferenceError(
                    f"database user_version is {user_version}; expected "
                    f"{DATABASE_SCHEMA_VERSION}"
                )
            metadata = dict(database.execute("SELECT key, value FROM metadata"))
            if metadata.get("database_schema_version") != str(DATABASE_SCHEMA_VERSION):
                raise ReferenceError("database metadata has the wrong schema version")
            if metadata.get("fts5") != str(int(manifest_fts5)):
                raise ReferenceError("database metadata has the wrong FTS5 flag")
            for name in DATABASE_INPUTS:
                expected_sha = inputs[name].get("sha256")
                if metadata.get(f"input.{name}.sha256") != expected_sha:
                    raise ReferenceError(
                        f"database metadata does not match the {name} checksum"
                    )
                expected_count = inputs[name].get("record_count")
                if metadata.get(f"input.{name}.record_count") != str(expected_count):
                    raise ReferenceError(
                        f"database metadata does not match the {name} record count"
                    )
            actual_table_counts = {
                table: database.execute(f"SELECT count(*) FROM {table}").fetchone()[0]
                for table in DATABASE_TABLES
            }
            for table, actual_count in actual_table_counts.items():
                if actual_count != table_counts[table]:
                    raise ReferenceError(
                        f"{table} has {actual_count} rows; manifest records "
                        f"{table_counts[table]}"
                    )
                if metadata.get(f"table.{table}.rows") != str(actual_count):
                    raise ReferenceError(
                        f"database metadata has the wrong {table} row count"
                    )
            fts_table_names = {
                row[0]
                for row in database.execute(
                    "SELECT name FROM sqlite_schema WHERE type = 'table'"
                )
            }
            if manifest_fts5:
                missing_fts = set(DATABASE_FTS_TABLES) - fts_table_names
                if missing_fts:
                    raise ReferenceError(
                        "database is missing FTS5 tables: "
                        + ", ".join(sorted(missing_fts))
                    )
                expected_fts_counts = {
                    "card_search": actual_table_counts["cards"],
                    "ruling_search": actual_table_counts["rulings"],
                }
                for table, expected_count in expected_fts_counts.items():
                    actual_count = database.execute(
                        f"SELECT count(*) FROM {table}"
                    ).fetchone()[0]
                    if actual_count != expected_count:
                        raise ReferenceError(
                            f"{table} has {actual_count} rows; expected {expected_count}"
                        )
                    recorded_count = table_counts.get(table)
                    if recorded_count is not None and recorded_count != actual_count:
                        raise ReferenceError(
                            f"manifest records the wrong {table} row count"
                        )
            elif set(DATABASE_FTS_TABLES) & fts_table_names:
                raise ReferenceError(
                    "database contains FTS5 tables but its manifest disables FTS5"
                )
            foreign_key_problem = database.execute(
                "PRAGMA foreign_key_check"
            ).fetchone()
            if foreign_key_problem is not None:
                raise ReferenceError(
                    f"SQLite foreign_key_check failed: {foreign_key_problem!r}"
                )
            quick_check = database.execute("PRAGMA quick_check").fetchone()
            if quick_check != ("ok",):
                raise ReferenceError(f"SQLite quick_check failed: {quick_check!r}")
            cards = actual_table_counts["cards"]
            rulings = actual_table_counts["rulings"]
    except (sqlite3.Error, ReferenceError) as error:
        return ResourceStatus(
            DATABASE_RESOURCE_NAME,
            "corrupt",
            relative_path,
            f"database validation failed: {error}",
            built_at,
            None,
        )

    fts_detail = "with FTS5" if manifest_fts5 else "without FTS5"
    return ResourceStatus(
        DATABASE_RESOURCE_NAME,
        "current",
        relative_path,
        f"indexes {cards} cards and {rulings} unique rulings {fts_detail}",
        built_at,
        None,
    )


def create_database_schema(database: sqlite3.Connection) -> None:
    database.executescript(
        """
        CREATE TABLE metadata (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        ) WITHOUT ROWID;

        CREATE TABLE cards (
            card_id INTEGER PRIMARY KEY,
            oracle_id TEXT NOT NULL UNIQUE,
            scryfall_id TEXT NOT NULL UNIQUE,
            name TEXT NOT NULL,
            normalized_name TEXT NOT NULL,
            layout TEXT,
            mana_cost TEXT,
            mana_value REAL,
            type_line TEXT,
            oracle_text TEXT,
            power TEXT,
            toughness TEXT,
            loyalty TEXT,
            defense TEXT,
            colors_json TEXT NOT NULL,
            color_identity_json TEXT NOT NULL,
            produced_mana_json TEXT NOT NULL,
            keywords_json TEXT NOT NULL,
            legalities_json TEXT NOT NULL,
            games_json TEXT NOT NULL,
            reserved INTEGER NOT NULL,
            digital INTEGER NOT NULL,
            game_changer INTEGER NOT NULL,
            released_at TEXT,
            representative_set_code TEXT,
            representative_set_name TEXT,
            representative_collector_number TEXT,
            representative_rarity TEXT,
            scryfall_uri TEXT,
            rulings_uri TEXT,
            image_uri TEXT
        );

        CREATE TABLE card_faces (
            card_id INTEGER NOT NULL REFERENCES cards(card_id) ON DELETE CASCADE,
            face_index INTEGER NOT NULL,
            name TEXT NOT NULL,
            normalized_name TEXT NOT NULL,
            mana_cost TEXT,
            type_line TEXT,
            oracle_text TEXT,
            power TEXT,
            toughness TEXT,
            loyalty TEXT,
            defense TEXT,
            colors_json TEXT NOT NULL,
            image_uri TEXT,
            PRIMARY KEY (card_id, face_index)
        ) WITHOUT ROWID;

        CREATE TABLE card_names (
            card_id INTEGER NOT NULL REFERENCES cards(card_id) ON DELETE CASCADE,
            name_index INTEGER NOT NULL,
            name_kind TEXT NOT NULL CHECK (name_kind IN ('card', 'face')),
            name TEXT NOT NULL,
            normalized_name TEXT NOT NULL,
            PRIMARY KEY (card_id, name_index)
        ) WITHOUT ROWID;

        CREATE TABLE card_keywords (
            card_id INTEGER NOT NULL REFERENCES cards(card_id) ON DELETE CASCADE,
            keyword TEXT NOT NULL,
            PRIMARY KEY (card_id, keyword)
        ) WITHOUT ROWID;

        CREATE TABLE card_colors (
            card_id INTEGER NOT NULL REFERENCES cards(card_id) ON DELETE CASCADE,
            kind TEXT NOT NULL CHECK (kind IN ('color', 'identity', 'produced')),
            color TEXT NOT NULL,
            PRIMARY KEY (card_id, kind, color)
        ) WITHOUT ROWID;

        CREATE TABLE card_parts (
            card_id INTEGER NOT NULL REFERENCES cards(card_id) ON DELETE CASCADE,
            part_index INTEGER NOT NULL,
            component TEXT,
            related_scryfall_id TEXT,
            name TEXT,
            type_line TEXT,
            uri TEXT,
            PRIMARY KEY (card_id, part_index)
        ) WITHOUT ROWID;

        CREATE TABLE rulings (
            ruling_id INTEGER PRIMARY KEY,
            fingerprint BLOB NOT NULL UNIQUE,
            oracle_id TEXT NOT NULL,
            published_at TEXT NOT NULL,
            source TEXT NOT NULL,
            comment TEXT NOT NULL
        );
        """
    )


def insert_card(database: sqlite3.Connection, item: dict[str, Any], line: int) -> None:
    context = f"oracle-cards line {line}"
    oracle_id = required_text(item, "oracle_id", context=context)
    scryfall_id = required_text(item, "id", context=context)
    name = required_text(item, "name", context=context)
    mana_value = item.get("cmc")
    if isinstance(mana_value, bool) or not isinstance(mana_value, (int, float)):
        mana_value = None
    elif not math.isfinite(float(mana_value)):
        raise ReferenceError(f"{context} has a non-finite mana value")

    colors = item.get("colors") if isinstance(item.get("colors"), list) else []
    identity = (
        item.get("color_identity")
        if isinstance(item.get("color_identity"), list)
        else []
    )
    produced = (
        item.get("produced_mana") if isinstance(item.get("produced_mana"), list) else []
    )
    keywords = item.get("keywords") if isinstance(item.get("keywords"), list) else []
    legalities = (
        item.get("legalities") if isinstance(item.get("legalities"), dict) else {}
    )
    games = item.get("games") if isinstance(item.get("games"), list) else []
    card_values = {
        "oracle_id": oracle_id,
        "scryfall_id": scryfall_id,
        "name": name,
        "normalized_name": normalized_card_name(name),
        "layout": optional_text(item, "layout"),
        "mana_cost": optional_text(item, "mana_cost"),
        "mana_value": mana_value,
        "type_line": optional_text(item, "type_line"),
        "oracle_text": optional_text(item, "oracle_text"),
        "power": optional_text(item, "power"),
        "toughness": optional_text(item, "toughness"),
        "loyalty": optional_text(item, "loyalty"),
        "defense": optional_text(item, "defense"),
        "colors_json": compact_json(colors),
        "color_identity_json": compact_json(identity),
        "produced_mana_json": compact_json(produced),
        "keywords_json": compact_json(keywords),
        "legalities_json": compact_json(legalities),
        "games_json": compact_json(games),
        "reserved": int(item.get("reserved") is True),
        "digital": int(item.get("digital") is True),
        "game_changer": int(item.get("game_changer") is True),
        "released_at": optional_text(item, "released_at"),
        "representative_set_code": optional_text(item, "set"),
        "representative_set_name": optional_text(item, "set_name"),
        "representative_collector_number": optional_text(item, "collector_number"),
        "representative_rarity": optional_text(item, "rarity"),
        "scryfall_uri": optional_text(item, "scryfall_uri"),
        "rulings_uri": optional_text(item, "rulings_uri"),
        "image_uri": nested_text(item, "image_uris", "normal"),
    }
    cursor = database.execute(
        """
        INSERT INTO cards (
            oracle_id, scryfall_id, name, normalized_name, layout, mana_cost,
            mana_value, type_line, oracle_text, power, toughness, loyalty,
            defense, colors_json, color_identity_json, produced_mana_json,
            keywords_json, legalities_json, games_json, reserved, digital,
            game_changer, released_at, representative_set_code,
            representative_set_name, representative_collector_number,
            representative_rarity, scryfall_uri, rulings_uri, image_uri
        ) VALUES (
            :oracle_id, :scryfall_id, :name, :normalized_name, :layout, :mana_cost,
            :mana_value, :type_line, :oracle_text, :power, :toughness, :loyalty,
            :defense, :colors_json, :color_identity_json, :produced_mana_json,
            :keywords_json, :legalities_json, :games_json, :reserved, :digital,
            :game_changer, :released_at, :representative_set_code,
            :representative_set_name, :representative_collector_number,
            :representative_rarity, :scryfall_uri, :rulings_uri, :image_uri
        )
        """,
        card_values,
    )
    card_id = cursor.lastrowid
    if card_id is None:
        raise ReferenceError(f"failed to index {context}")
    database.execute(
        "INSERT INTO card_names VALUES (?, -1, 'card', ?, ?)",
        (card_id, name, normalized_card_name(name)),
    )

    faces = item.get("card_faces") if isinstance(item.get("card_faces"), list) else []
    for face_index, face in enumerate(faces):
        if not isinstance(face, dict):
            raise ReferenceError(f"{context} has an invalid card face")
        face_name = required_text(face, "name", context=f"{context} face {face_index}")
        face_colors = face.get("colors") if isinstance(face.get("colors"), list) else []
        database.execute(
            """
            INSERT INTO card_faces (
                card_id, face_index, name, normalized_name, mana_cost, type_line,
                oracle_text, power, toughness, loyalty, defense, colors_json,
                image_uri
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            """,
            (
                card_id,
                face_index,
                face_name,
                normalized_card_name(face_name),
                optional_text(face, "mana_cost"),
                optional_text(face, "type_line"),
                optional_text(face, "oracle_text"),
                optional_text(face, "power"),
                optional_text(face, "toughness"),
                optional_text(face, "loyalty"),
                optional_text(face, "defense"),
                compact_json(face_colors),
                nested_text(face, "image_uris", "normal"),
            ),
        )
        database.execute(
            "INSERT INTO card_names VALUES (?, ?, 'face', ?, ?)",
            (card_id, face_index, face_name, normalized_card_name(face_name)),
        )

    for keyword in dict.fromkeys(
        value for value in keywords if isinstance(value, str) and value
    ):
        database.execute("INSERT INTO card_keywords VALUES (?, ?)", (card_id, keyword))
    for kind, values in (
        ("color", colors),
        ("identity", identity),
        ("produced", produced),
    ):
        for color in dict.fromkeys(
            value for value in values if isinstance(value, str) and value
        ):
            database.execute(
                "INSERT INTO card_colors VALUES (?, ?, ?)",
                (card_id, kind, color),
            )

    parts = item.get("all_parts") if isinstance(item.get("all_parts"), list) else []
    for part_index, part in enumerate(parts):
        if not isinstance(part, dict):
            continue
        database.execute(
            """
            INSERT INTO card_parts (
                card_id, part_index, component, related_scryfall_id, name,
                type_line, uri
            ) VALUES (?, ?, ?, ?, ?, ?, ?)
            """,
            (
                card_id,
                part_index,
                optional_text(part, "component"),
                optional_text(part, "id"),
                optional_text(part, "name"),
                optional_text(part, "type_line"),
                optional_text(part, "uri"),
            ),
        )


def insert_ruling(
    database: sqlite3.Connection, item: dict[str, Any], line: int
) -> bool:
    context = f"rulings line {line}"
    oracle_id = required_text(item, "oracle_id", context=context)
    published_at = required_text(item, "published_at", context=context)
    source = required_text(item, "source", context=context)
    comment = required_text(item, "comment", context=context)
    fingerprint_payload = compact_json(
        [oracle_id, published_at, source, comment]
    ).encode("utf-8")
    fingerprint = hashlib.sha256(fingerprint_payload).digest()
    cursor = database.execute(
        """
        INSERT OR IGNORE INTO rulings (
            fingerprint, oracle_id, published_at, source, comment
        ) VALUES (?, ?, ?, ?, ?)
        """,
        (fingerprint, oracle_id, published_at, source, comment),
    )
    return cursor.rowcount == 1


def create_database_indexes(database: sqlite3.Connection) -> None:
    statements = (
        "CREATE INDEX idx_cards_normalized_name ON cards(normalized_name, layout)",
        "CREATE INDEX idx_cards_mana_value ON cards(mana_value, card_id)",
        "CREATE INDEX idx_cards_representative_set ON cards(representative_set_code, card_id)",
        "CREATE INDEX idx_card_faces_normalized_name ON card_faces(normalized_name, card_id)",
        "CREATE INDEX idx_card_names_normalized_name ON card_names(normalized_name, card_id)",
        "CREATE INDEX idx_card_keywords_keyword ON card_keywords(keyword, card_id)",
        "CREATE INDEX idx_card_colors_kind_color ON card_colors(kind, color, card_id)",
        "CREATE INDEX idx_card_parts_related_id ON card_parts(related_scryfall_id, card_id)",
        "CREATE INDEX idx_rulings_oracle_date ON rulings(oracle_id, published_at DESC, ruling_id)",
    )
    for statement in statements:
        database.execute(statement)


def create_fts_indexes(database: sqlite3.Connection) -> bool:
    try:
        database.execute(
            """
            CREATE VIRTUAL TABLE card_search USING fts5(
                name, type_line, oracle_text, keywords,
                content='',
                tokenize='unicode61 remove_diacritics 2'
            )
            """
        )
        database.execute(
            """
            INSERT INTO card_search(rowid, name, type_line, oracle_text, keywords)
            SELECT
                cards.card_id,
                cards.name || COALESCE((
                    SELECT char(10) || group_concat(card_faces.name, char(10))
                    FROM card_faces WHERE card_faces.card_id = cards.card_id
                ), ''),
                COALESCE(cards.type_line, '') || COALESCE((
                    SELECT char(10) || group_concat(card_faces.type_line, char(10))
                    FROM card_faces WHERE card_faces.card_id = cards.card_id
                ), ''),
                COALESCE(cards.oracle_text, '') || COALESCE((
                    SELECT char(10) || group_concat(card_faces.oracle_text, char(10))
                    FROM card_faces WHERE card_faces.card_id = cards.card_id
                ), ''),
                cards.keywords_json
            FROM cards
            """
        )
        database.execute(
            """
            CREATE VIRTUAL TABLE ruling_search USING fts5(
                comment,
                content='',
                tokenize='unicode61 remove_diacritics 2'
            )
            """
        )
        database.execute(
            """
            INSERT INTO ruling_search(rowid, comment)
            SELECT ruling_id, comment FROM rulings
            """
        )
    except sqlite3.OperationalError as error:
        if "no such module: fts5" not in str(error).lower():
            raise
        return False
    return True


def atomic_build_scryfall_database(
    reference_dir: Path, input_records: dict[str, dict[str, Any]]
) -> dict[str, Any]:
    destination = ensure_safe_cache_parent(reference_dir, DATABASE_RELATIVE_PATH)
    temporary: Path | None = None
    built_at = iso_now()
    source_counts = {name: 0 for name in DATABASE_INPUTS}
    unique_rulings = 0
    try:
        with tempfile.NamedTemporaryFile(
            dir=destination.parent,
            prefix=f".{DATABASE_NAME}.",
            suffix=".tmp",
            delete=False,
        ) as temporary_file:
            temporary = Path(temporary_file.name)

        database = sqlite3.connect(temporary)
        try:
            database.execute("PRAGMA journal_mode = OFF")
            database.execute("PRAGMA synchronous = OFF")
            database.execute("PRAGMA temp_store = MEMORY")
            database.execute("PRAGMA foreign_keys = ON")
            database.execute(f"PRAGMA user_version = {DATABASE_SCHEMA_VERSION}")
            create_database_schema(database)
            database.execute("BEGIN")

            oracle_path = reference_dir / RESOURCE_FILES["oracle-cards"]
            for line, item in enumerate(
                iter_scryfall_records(
                    oracle_path,
                    "oracle-cards",
                    compressed_size=input_records["oracle-cards"]["size"],
                ),
                start=1,
            ):
                insert_card(database, item, line)
                source_counts["oracle-cards"] += 1

            rulings_path = reference_dir / RESOURCE_FILES["rulings"]
            for line, item in enumerate(
                iter_scryfall_records(
                    rulings_path,
                    "rulings",
                    compressed_size=input_records["rulings"]["size"],
                ),
                start=1,
            ):
                unique_rulings += int(insert_ruling(database, item, line))
                source_counts["rulings"] += 1

            for name in DATABASE_INPUTS:
                expected = input_records[name]["record_count"]
                if source_counts[name] != expected:
                    raise ReferenceError(
                        f"{name} yielded {source_counts[name]} records; expected {expected}"
                    )

            create_database_indexes(database)
            fts5 = create_fts_indexes(database)
            table_counts = {
                table: database.execute(f"SELECT count(*) FROM {table}").fetchone()[0]
                for table in DATABASE_TABLES
            }
            if fts5:
                table_counts.update(
                    {
                        table: database.execute(
                            f"SELECT count(*) FROM {table}"
                        ).fetchone()[0]
                        for table in DATABASE_FTS_TABLES
                    }
                )
            orphan_rulings = database.execute(
                """
                SELECT count(*)
                FROM rulings LEFT JOIN cards USING (oracle_id)
                WHERE cards.card_id IS NULL
                """
            ).fetchone()[0]
            metadata = {
                "database_schema_version": str(DATABASE_SCHEMA_VERSION),
                "built_at": built_at,
                "sqlite_version": sqlite3.sqlite_version,
                "fts5": str(int(fts5)),
                "source.oracle-cards.records": str(source_counts["oracle-cards"]),
                "source.rulings.records": str(source_counts["rulings"]),
                "indexed.rulings.duplicates_removed": str(
                    source_counts["rulings"] - unique_rulings
                ),
                "indexed.rulings.orphans": str(orphan_rulings),
            }
            for name, record in input_records.items():
                metadata[f"input.{name}.path"] = record["path"]
                metadata[f"input.{name}.sha256"] = record["sha256"]
                metadata[f"input.{name}.source_updated_at"] = str(
                    record.get("source_updated_at") or ""
                )
                metadata[f"input.{name}.record_count"] = str(record["record_count"])
            for table, count in table_counts.items():
                metadata[f"table.{table}.rows"] = str(count)
            database.executemany(
                "INSERT INTO metadata(key, value) VALUES (?, ?)", metadata.items()
            )
            database.commit()
            database.execute("ANALYZE")
            database.execute("PRAGMA optimize")
            database.commit()

            integrity = database.execute("PRAGMA integrity_check").fetchone()
            if integrity != ("ok",):
                raise ReferenceError(f"SQLite integrity_check failed: {integrity!r}")
            foreign_key_problems = database.execute(
                "PRAGMA foreign_key_check"
            ).fetchall()
            if foreign_key_problems:
                raise ReferenceError(
                    f"SQLite foreign_key_check found {len(foreign_key_problems)} problems"
                )
        finally:
            database.close()

        with temporary.open("rb") as database_file:
            os.fsync(database_file.fileno())
        size = temporary.stat().st_size
        digest = sha256_file(temporary)
        os.replace(temporary, destination)
        temporary = None
        return {
            "path": DATABASE_RELATIVE_PATH,
            "format": "application/vnd.sqlite3",
            "schema_version": DATABASE_SCHEMA_VERSION,
            "built_at": built_at,
            "size": size,
            "sha256": digest,
            "fts5": fts5,
            "inputs": {
                name: {
                    "path": record["path"],
                    "sha256": record["sha256"],
                    "record_count": record["record_count"],
                    "source_updated_at": record.get("source_updated_at"),
                }
                for name, record in input_records.items()
            },
            "source_counts": source_counts,
            "table_counts": table_counts,
            "duplicate_rulings_removed": source_counts["rulings"] - unique_rulings,
            "orphan_rulings": orphan_rulings,
        }
    except (sqlite3.Error, UnicodeError, ValueError) as error:
        raise ReferenceError(f"cannot build {DATABASE_NAME}: {error}") from error
    finally:
        if temporary is not None:
            temporary.unlink(missing_ok=True)
            Path(f"{temporary}-journal").unlink(missing_ok=True)
            Path(f"{temporary}-wal").unlink(missing_ok=True)
            Path(f"{temporary}-shm").unlink(missing_ok=True)


def ensure_scryfall_database(
    reference_dir: Path, manifest: dict[str, Any], *, force: bool
) -> bool:
    status = inspect_scryfall_database(reference_dir, manifest)
    if not force and status.status == "current":
        print(f"{DATABASE_RESOURCE_NAME}: current; skipped ({status.detail})")
        return False
    input_records = database_input_records(reference_dir, manifest)
    print(f"{DATABASE_RESOURCE_NAME}: building {DATABASE_NAME}")
    record = atomic_build_scryfall_database(reference_dir, input_records)
    manifest["derived"][DATABASE_MANIFEST_KEY] = record
    write_manifest(reference_dir, manifest)
    print(
        f"{DATABASE_RESOURCE_NAME}: built {record['size']} bytes; "
        f"sha256={record['sha256']}"
    )
    return True


def inspect_resource(
    name: str,
    *,
    reference_dir: Path,
    manifest: dict[str, Any],
    max_age_days: float,
    timeout: float,
    remote_cache: dict[str, Any],
) -> tuple[ResourceStatus, RemoteResource | None]:
    record = manifest["resources"].get(name)
    problem = local_problem(name, reference_dir, record)
    local_updated = (
        record.get("source_updated_at") if isinstance(record, dict) else None
    )
    try:
        remote = load_remote_resource(name, timeout=timeout, cache=remote_cache)
    except (HTTPError, URLError, TimeoutError, OSError, ReferenceError) as error:
        if problem:
            status, detail, problem_updated = problem
            return (
                ResourceStatus(
                    name,
                    status,
                    RESOURCE_FILES[name],
                    f"{detail}; remote check failed: {error}",
                    problem_updated,
                    None,
                ),
                None,
            )
        return (
            ResourceStatus(
                name,
                "unknown",
                RESOURCE_FILES[name],
                f"local file is intact but remote freshness check failed: {error}",
                local_updated,
                None,
            ),
            None,
        )

    if problem:
        status, detail, problem_updated = problem
        return (
            ResourceStatus(
                name,
                status,
                RESOURCE_FILES[name],
                detail,
                problem_updated,
                remote.source_updated_at,
            ),
            remote,
        )

    assert record is not None
    local_path = reference_dir / RESOURCE_FILES[name]
    if remote.content is not None:
        remote_sha = sha256_bytes(remote.content)
        if remote_sha != record.get("sha256"):
            return (
                ResourceStatus(
                    name,
                    "stale",
                    RESOURCE_FILES[name],
                    "authoritative source content has changed",
                    local_updated,
                    remote.source_updated_at,
                ),
                remote,
            )
        return (
            ResourceStatus(
                name,
                "current",
                RESOURCE_FILES[name],
                f"matches authoritative source ({local_path.stat().st_size} bytes)",
                local_updated,
                remote.source_updated_at,
            ),
            remote,
        )

    local_source_time = parse_timestamp(local_updated)
    remote_source_time = parse_timestamp(remote.source_updated_at)
    if local_source_time is None or remote_source_time is None:
        return (
            ResourceStatus(
                name,
                "unknown",
                RESOURCE_FILES[name],
                "local or remote Scryfall revision is not a usable timestamp",
                local_updated,
                remote.source_updated_at,
            ),
            remote,
        )
    if remote_source_time < local_source_time:
        return (
            ResourceStatus(
                name,
                "unknown",
                RESOURCE_FILES[name],
                "advertised Scryfall revision is older than the local snapshot; refusing "
                "to infer a downgrade",
                local_updated,
                remote.source_updated_at,
            ),
            remote,
        )

    revision_changed = (
        remote_source_time > local_source_time
        or remote.source_url != record.get("source_url")
        or remote.expected_size != record.get("size")
    )
    if revision_changed:
        retrieved_at = parse_timestamp(record.get("retrieved_at"))
        if retrieved_at is None:
            return (
                ResourceStatus(
                    name,
                    "stale",
                    RESOURCE_FILES[name],
                    "manifest has no usable retrieval timestamp",
                    local_updated,
                    remote.source_updated_at,
                ),
                remote,
            )
        raw_age = utc_now() - retrieved_at
        if raw_age < timedelta(minutes=-5):
            return (
                ResourceStatus(
                    name,
                    "unknown",
                    RESOURCE_FILES[name],
                    "manifest retrieval timestamp is unexpectedly in the future",
                    local_updated,
                    remote.source_updated_at,
                ),
                remote,
            )
        age = max(raw_age, timedelta(0))
        freshness = timedelta(days=max_age_days)
        if age >= freshness:
            return (
                ResourceStatus(
                    name,
                    "stale",
                    RESOURCE_FILES[name],
                    f"newer Scryfall snapshot is available; local copy is {age.days}d old",
                    local_updated,
                    remote.source_updated_at,
                ),
                remote,
            )
        return (
            ResourceStatus(
                name,
                "fresh",
                RESOURCE_FILES[name],
                "newer Scryfall snapshot exists but local copy is within the "
                f"{max_age_days:g}-day gameplay-data window",
                local_updated,
                remote.source_updated_at,
            ),
            remote,
        )

    return (
        ResourceStatus(
            name,
            "current",
            RESOURCE_FILES[name],
            "matches the latest advertised Scryfall snapshot",
            local_updated,
            remote.source_updated_at,
        ),
        remote,
    )


def normalize_targets(targets: Iterable[str]) -> list[str]:
    values = list(targets) or list(DEFAULT_RESOURCES)
    return list(dict.fromkeys(values))


def print_statuses(statuses: list[ResourceStatus], *, as_json: bool) -> None:
    if as_json:
        print(json.dumps([status.as_dict() for status in statuses], indent=2))
        return
    width = max(len(status.name) for status in statuses)
    for status in statuses:
        print(f"{status.name:<{width}}  {status.status:<9}  {status.detail}")


def status_exit_code(statuses: list[ResourceStatus]) -> int:
    if any(status.status == "unknown" for status in statuses):
        return 2
    if any(status.status in BAD_STATUSES for status in statuses):
        return 1
    return 0


def require_shared_write_approval(args: argparse.Namespace) -> None:
    if args.approve_shared_write:
        return
    raise ReferenceError(
        "shared reference-cache mutation requires explicit human approval; inspect "
        "the cache first, then rerun with --approve-shared-write after approval"
    )


def reference_path(reference_dir: Path, target: str) -> Path:
    if target == "cache-root":
        return reference_dir
    if target == "manifest":
        return reference_dir / MANIFEST_NAME
    if target == "lock":
        return reference_dir / LOCK_NAME
    if target == DATABASE_RESOURCE_NAME:
        return reference_dir / DATABASE_RELATIVE_PATH
    return reference_dir / RESOURCE_FILES[target]


def probe_lock_kernel_state(lock_path: Path) -> tuple[str, str | None]:
    """Probe flock state without creating or modifying the persistent lock file."""

    try:
        lock_file = open_lock_file(
            lock_path, exclusive=False, create=False, binary=True
        )
    except FileNotFoundError:
        return "absent", None
    except (OSError, ReferenceError) as error:
        return "unknown", str(error)
    with lock_file:
        try:
            fcntl.flock(lock_file.fileno(), fcntl.LOCK_EX | fcntl.LOCK_NB)
        except BlockingIOError:
            try:
                fcntl.flock(lock_file.fileno(), fcntl.LOCK_SH | fcntl.LOCK_NB)
            except BlockingIOError:
                return "exclusive-writer", None
            except OSError as error:
                return "unknown", str(error)
            else:
                fcntl.flock(lock_file.fileno(), fcntl.LOCK_UN)
                return "shared-readers", None
        except OSError as error:
            return "unknown", str(error)
        else:
            fcntl.flock(lock_file.fileno(), fcntl.LOCK_UN)
            return "unlocked", None


def lock_event_signature(events: Iterable[dict[str, Any]]) -> tuple[Any, ...]:
    retained = list(events)
    owner = latest_lock_owner(retained)
    if owner is None:
        return (len(retained), None)
    return (
        len(retained),
        owner.get("owner_id"),
        owner.get("released"),
        owner.get("phase"),
        owner.get("updated_at"),
        owner.get("released_at"),
    )


def lock_diagnostics(lock_path: Path) -> dict[str, Any]:
    before_events, before_problems = read_lock_events(lock_path)
    stable = False
    kernel_state = "unknown"
    probe_error: str | None = None
    after_events = before_events
    after_problems = before_problems
    for _ in range(3):
        kernel_state, probe_error = probe_lock_kernel_state(lock_path)
        after_events, after_problems = read_lock_events(lock_path)
        if lock_event_signature(before_events) == lock_event_signature(after_events):
            stable = True
            break
        before_events = after_events
        before_problems = after_problems
        time.sleep(0.01)

    last_owner = latest_lock_owner(after_events)
    owner = (
        last_owner
        if last_owner is not None and not last_owner.get("released")
        else None
    )
    problems = list(dict.fromkeys([*before_problems, *after_problems]))
    if not stable:
        metadata_state = "metadata-in-transition"
    elif kernel_state == "unknown":
        metadata_state = "unknown"
    elif kernel_state == "absent":
        metadata_state = "absent"
    elif kernel_state == "exclusive-writer":
        metadata_state = "active-owner" if owner else "active-writer-owner-unknown"
    elif kernel_state == "unlocked" and owner:
        metadata_state = "stale-owner-metadata"
    elif kernel_state == "shared-readers" and owner:
        metadata_state = "shared-readers-with-unreleased-owner-metadata"
    elif last_owner is not None:
        metadata_state = "released-owner"
    else:
        metadata_state = "empty"
    result: dict[str, Any] = {
        "path": str(lock_path),
        "kernel_state": kernel_state,
        "metadata_state": metadata_state,
        "snapshot_stable": stable,
        "event_count_retained": len(after_events),
        "owner": owner,
        "last_owner": last_owner,
        "same_host_process": same_host_process_state(owner),
        "warnings": problems,
    }
    if probe_error:
        result["probe_error"] = probe_error
    return result


def confined_manifest_path(root: Path, relative: Any, *, label: str) -> Path:
    if not isinstance(relative, str) or not relative or Path(relative).is_absolute():
        raise ReferenceError(f"{label} has an invalid relative path: {relative!r}")
    resolved_root = root.resolve()
    resolved = (resolved_root / relative).resolve()
    try:
        resolved.relative_to(resolved_root)
    except ValueError:
        raise ReferenceError(f"{label} escapes {resolved_root}: {relative!r}") from None
    return resolved


def validate_recorded_file(
    source: Path, record: dict[str, Any], *, label: str
) -> tuple[int, str]:
    expected_size = record.get("size")
    expected_sha = record.get("sha256")
    if type(expected_size) is not int or expected_size < 0:
        raise ReferenceError(f"{label} has no valid recorded size")
    if not isinstance(expected_sha, str) or not re.fullmatch(
        r"[0-9a-f]{64}", expected_sha
    ):
        raise ReferenceError(f"{label} has no valid recorded SHA-256")
    if not source.is_file():
        raise ReferenceError(f"{label} does not exist: {source}")
    actual_size = source.stat().st_size
    if actual_size != expected_size:
        raise ReferenceError(
            f"{label} is {actual_size} bytes; manifest records {expected_size}"
        )
    actual_sha = sha256_file(source)
    if actual_sha != expected_sha:
        raise ReferenceError(f"{label} SHA-256 does not match its manifest record")
    return actual_size, actual_sha


def copy_recorded_file(
    source: Path,
    destination: Path,
    record: dict[str, Any],
    *,
    label: str,
) -> str:
    expected_size, expected_sha = validate_recorded_file(source, record, label=label)
    try:
        destination_info = os.lstat(destination)
    except FileNotFoundError:
        destination_info = None
    if destination_info is not None:
        if stat.S_ISLNK(destination_info.st_mode) or not stat.S_ISREG(
            destination_info.st_mode
        ):
            raise ReferenceError(
                f"shared-cache destination is not a regular file: {destination}"
            )
        destination_matches = (
            destination_info.st_size == expected_size
            and sha256_file(destination) == expected_sha
        )
        if destination_matches:
            return "already present"
        raise ReferenceError(
            f"refusing to replace conflicting shared-cache file: {destination}"
        )

    destination.parent.mkdir(parents=True, exist_ok=True)
    temporary: Path | None = None
    try:
        digest = hashlib.sha256()
        copied = 0
        with (
            source.open("rb") as source_file,
            tempfile.NamedTemporaryFile(
                mode="wb",
                dir=destination.parent,
                prefix=f".{destination.name}.",
                suffix=".tmp",
                delete=False,
            ) as target,
        ):
            temporary = Path(target.name)
            while chunk := source_file.read(1024 * 1024):
                target.write(chunk)
                digest.update(chunk)
                copied += len(chunk)
            target.flush()
            os.fsync(target.fileno())
        if copied != expected_size or digest.hexdigest() != expected_sha:
            raise ReferenceError(f"{label} changed while it was being copied")
        os.replace(temporary, destination)
        temporary = None
        return "copied"
    finally:
        if temporary is not None:
            temporary.unlink(missing_ok=True)


def preflight_destination(
    destination: Path, record: dict[str, Any], *, label: str
) -> None:
    try:
        destination_info = os.lstat(destination)
    except FileNotFoundError:
        return
    if stat.S_ISLNK(destination_info.st_mode) or not stat.S_ISREG(
        destination_info.st_mode
    ):
        raise ReferenceError(
            f"shared-cache destination is not a regular file: {destination}"
        )
    validate_recorded_file(destination, record, label=f"shared-cache {label}")


def validate_target_manifest_record(
    existing: Any, record: dict[str, Any], *, label: str
) -> None:
    if existing is None:
        return
    if not isinstance(existing, dict):
        raise ReferenceError(f"existing shared-cache {label} record is not an object")
    for record_field in ("size", "sha256"):
        if existing.get(record_field) != record.get(record_field):
            raise ReferenceError(
                f"existing shared-cache {label} manifest record conflicts on "
                f"{record_field}"
            )


def validate_database_artifact(
    path: Path, record: dict[str, Any], *, schema_version: int, label: str
) -> None:
    validate_recorded_file(path, record, label=label)
    try:
        with sqlite3.connect(
            f"{path.resolve().as_uri()}?mode=ro", uri=True
        ) as database:
            database.execute("PRAGMA query_only = ON")
            user_version = database.execute("PRAGMA user_version").fetchone()[0]
            if user_version != schema_version:
                raise ReferenceError(
                    f"{label} user_version is {user_version}; expected {schema_version}"
                )
            metadata = dict(database.execute("SELECT key, value FROM metadata"))
            if metadata.get("database_schema_version") != str(schema_version):
                raise ReferenceError(f"{label} metadata has the wrong schema version")
            inputs = record.get("inputs")
            if not isinstance(inputs, dict):
                raise ReferenceError(f"{label} record has no inputs object")
            for name, input_record in inputs.items():
                if not isinstance(input_record, dict):
                    raise ReferenceError(f"{label} has an invalid {name} input record")
                expected_sha = input_record.get("sha256")
                expected_count = input_record.get("record_count")
                if metadata.get(f"input.{name}.sha256") != expected_sha:
                    raise ReferenceError(
                        f"{label} metadata does not match the {name} checksum"
                    )
                if metadata.get(f"input.{name}.record_count") != str(expected_count):
                    raise ReferenceError(
                        f"{label} metadata does not match the {name} record count"
                    )
            manifest_fts5 = record.get("fts5")
            if type(manifest_fts5) is not bool or metadata.get("fts5") != str(
                int(manifest_fts5)
            ):
                raise ReferenceError(f"{label} has inconsistent FTS5 metadata")
            table_counts = record.get("table_counts")
            if not isinstance(table_counts, dict):
                raise ReferenceError(f"{label} has no table counts")
            available_tables = {
                row[0]
                for row in database.execute(
                    "SELECT name FROM sqlite_schema WHERE type IN ('table', 'view')"
                )
            }
            for table, expected_count in table_counts.items():
                if (
                    not isinstance(table, str)
                    or not re.fullmatch(r"[A-Za-z_][A-Za-z0-9_]*", table)
                    or table not in available_tables
                    or type(expected_count) is not int
                    or expected_count < 0
                ):
                    raise ReferenceError(
                        f"{label} has an invalid {table!r} table count"
                    )
                actual_count = database.execute(
                    f"SELECT count(*) FROM {table}"
                ).fetchone()[0]
                if actual_count != expected_count:
                    raise ReferenceError(
                        f"{label} {table} count is {actual_count}; expected "
                        f"{expected_count}"
                    )
            if manifest_fts5:
                missing_fts = set(DATABASE_FTS_TABLES) - available_tables
                if missing_fts:
                    raise ReferenceError(
                        f"{label} is missing FTS5 tables: "
                        + ", ".join(sorted(missing_fts))
                    )
                for fts_table, base_table in (
                    ("card_search", "cards"),
                    ("ruling_search", "rulings"),
                ):
                    expected_count = table_counts.get(base_table)
                    if expected_count is None:
                        continue
                    actual_count = database.execute(
                        f"SELECT count(*) FROM {fts_table}"
                    ).fetchone()[0]
                    if actual_count != expected_count:
                        raise ReferenceError(
                            f"{label} {fts_table} count is {actual_count}; expected "
                            f"{expected_count}"
                        )
            elif set(DATABASE_FTS_TABLES) & available_tables:
                raise ReferenceError(
                    f"{label} contains FTS5 tables but its manifest disables FTS5"
                )
            foreign_key_problem = database.execute(
                "PRAGMA foreign_key_check"
            ).fetchone()
            if foreign_key_problem is not None:
                raise ReferenceError(
                    f"{label} foreign_key_check failed: {foreign_key_problem!r}"
                )
            quick_check = database.execute("PRAGMA quick_check").fetchone()
            if quick_check != ("ok",):
                raise ReferenceError(f"{label} quick_check failed: {quick_check!r}")
    except sqlite3.Error as error:
        raise ReferenceError(f"cannot validate {label}: {error}") from error


def legacy_database_plans(
    legacy_dir: Path,
    legacy_derived: dict[str, Any],
    reference_dir: Path,
    target_derived: dict[str, Any],
) -> list[tuple[int, str, Path, Path, dict[str, Any]]]:
    by_schema: dict[int, tuple[int, str, Path, Path, dict[str, Any]]] = {}
    for legacy_key, raw_record in legacy_derived.items():
        match = re.fullmatch(r"scryfall-index-schema-v([1-9][0-9]*)", legacy_key)
        if legacy_key != DATABASE_RESOURCE_NAME and match is None:
            raise ReferenceError(
                f"legacy manifest contains unsupported derived artifact {legacy_key!r}"
            )
        if not isinstance(raw_record, dict):
            raise ReferenceError(
                f"legacy {legacy_key} manifest record is not an object"
            )
        schema_version = raw_record.get("schema_version")
        if type(schema_version) is not int or schema_version <= 0:
            raise ReferenceError(f"legacy {legacy_key} has no valid schema version")
        if match is not None and int(match.group(1)) != schema_version:
            raise ReferenceError(
                f"legacy {legacy_key} key conflicts with schema version {schema_version}"
            )
        source = confined_manifest_path(
            legacy_dir,
            raw_record.get("path"),
            label=f"legacy Scryfall index schema v{schema_version}",
        )
        relative_path = f"indexes/schema-v{schema_version}/{DATABASE_NAME}"
        destination = validate_safe_cache_parent(reference_dir, relative_path)
        translated = json.loads(json.dumps(raw_record))
        translated["path"] = relative_path
        inputs = translated.get("inputs")
        if isinstance(inputs, dict):
            for name, input_record in inputs.items():
                if name in RESOURCE_FILES and isinstance(input_record, dict):
                    input_record["path"] = RESOURCE_FILES[name]
        target_key = f"{DATABASE_RESOURCE_NAME}-schema-v{schema_version}"
        existing_plan = by_schema.get(schema_version)
        if existing_plan is not None:
            existing_record = existing_plan[4]
            if any(
                existing_record.get(field) != translated.get(field)
                for field in ("size", "sha256")
            ):
                raise ReferenceError(
                    f"legacy manifest has conflicting schema-v{schema_version} indexes"
                )
            continue
        validate_database_artifact(
            source,
            raw_record,
            schema_version=schema_version,
            label=f"legacy Scryfall index schema v{schema_version}",
        )
        preflight_destination(
            destination,
            raw_record,
            label=f"Scryfall index schema v{schema_version}",
        )
        validate_target_manifest_record(
            target_derived.get(target_key),
            translated,
            label=f"Scryfall index schema v{schema_version}",
        )
        by_schema[schema_version] = (
            schema_version,
            target_key,
            source,
            destination,
            translated,
        )
    return list(by_schema.values())


def migrate_legacy_cache(
    legacy_dir: Path,
    reference_dir: Path,
    *,
    remove_source: bool,
    lock: CacheLock,
) -> tuple[int, int]:
    if legacy_dir.resolve() == reference_dir.resolve():
        raise ReferenceError(
            "legacy and shared reference directories resolve identically"
        )
    legacy_manifest = load_manifest(legacy_dir)
    if not (legacy_dir / MANIFEST_NAME).is_file():
        raise ReferenceError(
            f"legacy cache has no manifest: {legacy_dir / MANIFEST_NAME}"
        )
    target_manifest = load_manifest(reference_dir)
    source_plans: list[tuple[str, Path, Path, dict[str, Any]]] = []

    lock.update("preflighting-migration", legacy_dir=str(legacy_dir))
    for name, raw_record in legacy_manifest["resources"].items():
        if name not in RESOURCE_FILES:
            raise ReferenceError(
                f"legacy manifest contains unsupported resource {name!r}"
            )
        if not isinstance(raw_record, dict):
            raise ReferenceError(f"legacy {name} manifest record is not an object")
        source = confined_manifest_path(
            legacy_dir, raw_record.get("path"), label=f"legacy {name}"
        )
        destination = validate_safe_cache_parent(reference_dir, RESOURCE_FILES[name])
        validate_recorded_file(source, raw_record, label=f"legacy {name}")
        preflight_destination(destination, raw_record, label=name)
        translated = dict(raw_record)
        translated["path"] = RESOURCE_FILES[name]
        validate_target_manifest_record(
            target_manifest["resources"].get(name), translated, label=name
        )
        source_plans.append((name, source, destination, translated))

    database_plans = legacy_database_plans(
        legacy_dir,
        legacy_manifest["derived"],
        reference_dir,
        target_manifest["derived"],
    )

    lock.update("migrating-sources", count=len(source_plans))
    for name, source, destination, translated in source_plans:
        destination = ensure_safe_cache_parent(reference_dir, translated["path"])
        outcome = copy_recorded_file(
            source, destination, translated, label=f"legacy {name}"
        )
        existing = target_manifest["resources"].get(name)
        if not (
            isinstance(existing, dict)
            and existing.get("path") == RESOURCE_FILES[name]
            and all(
                existing.get(field) == translated.get(field)
                for field in ("size", "sha256")
            )
        ):
            target_manifest["resources"][name] = translated
        print(f"{name}: {outcome} at {destination}")

    lock.update("migrating-indexes", count=len(database_plans))
    for schema_version, key, source, destination, translated in database_plans:
        destination = ensure_safe_cache_parent(reference_dir, translated["path"])
        outcome = copy_recorded_file(
            source,
            destination,
            translated,
            label=f"legacy Scryfall index schema v{schema_version}",
        )
        existing = target_manifest["derived"].get(key)
        if not (
            isinstance(existing, dict)
            and existing.get("path") == translated["path"]
            and all(
                existing.get(field) == translated.get(field)
                for field in ("size", "sha256")
            )
        ):
            target_manifest["derived"][key] = translated
        print(f"{key}: {outcome} at {destination}")

    target_manifest["last_migration"] = {
        "at": iso_now(),
        "from": str(legacy_dir.resolve()),
        "layout_version": CACHE_LAYOUT_VERSION,
    }
    lock.update("validating-migration")
    for name in legacy_manifest["resources"]:
        problem = local_problem(
            name, reference_dir, target_manifest["resources"].get(name)
        )
        if problem:
            status, detail, _ = problem
            raise ReferenceError(f"migrated {name} is {status}: {detail}")
    for schema_version, key, _, destination, translated in database_plans:
        validate_database_artifact(
            destination,
            translated,
            schema_version=schema_version,
            label=f"migrated {key}",
        )
        if schema_version == DATABASE_SCHEMA_VERSION:
            database_status = inspect_scryfall_database(reference_dir, target_manifest)
            if database_status.status != "current":
                raise ReferenceError(
                    f"migrated {DATABASE_RESOURCE_NAME} is "
                    f"{database_status.status}: {database_status.detail}"
                )

    lock.update("recording-manifest")
    write_manifest(reference_dir, target_manifest)

    removed = 0
    if remove_source:
        lock.update("removing-legacy-payloads")
        removable = [
            (source, destination) for _, source, destination, _ in source_plans
        ]
        removable.extend(
            (source, destination) for _, _, source, destination, _ in database_plans
        )
        for source, destination in removable:
            if source == destination:
                continue
            if source.exists():
                source.unlink()
                removed += 1
        legacy_manifest_path = legacy_dir / MANIFEST_NAME
        if legacy_manifest_path.exists():
            legacy_manifest_path.unlink()
            removed += 1
    return len(source_plans) + len(database_plans), removed


def command_status(args: argparse.Namespace) -> int:
    repo_root = find_repo_root(args.repo_root)
    reference_dir = resolve_reference_dir(repo_root, args.reference_dir)
    with cache_lock(
        reference_dir,
        repo_root=repo_root,
        operation="status",
        exclusive=False,
        timeout=args.lock_timeout,
    ):
        manifest = load_manifest(reference_dir)
        remote_cache: dict[str, Any] = {}
        targets = normalize_targets(args.resources)
        source_targets = [name for name in targets if name != DATABASE_RESOURCE_NAME]
        statuses = [
            inspect_resource(
                name,
                reference_dir=reference_dir,
                manifest=manifest,
                max_age_days=args.max_age_days,
                timeout=args.timeout,
                remote_cache=remote_cache,
            )[0]
            for name in source_targets
        ]
        if DATABASE_RESOURCE_NAME in targets or not args.resources:
            statuses.append(inspect_scryfall_database(reference_dir, manifest))
    print_statuses(statuses, as_json=args.json)
    return status_exit_code(statuses)


def command_fetch(args: argparse.Namespace) -> int:
    require_shared_write_approval(args)
    repo_root = find_repo_root(args.repo_root)
    reference_dir = resolve_reference_dir(repo_root, args.reference_dir)
    targets = normalize_targets(args.resources)
    with cache_lock(
        reference_dir,
        repo_root=repo_root,
        operation="fetch",
        exclusive=True,
        timeout=args.lock_timeout,
        details={
            "resources": targets,
            "force": args.force,
            "human_approval_asserted": True,
        },
    ) as lock:
        lock.update("loading-manifest")
        manifest = load_manifest(reference_dir)
        remote_cache: dict[str, Any] = {}

        for name in targets:
            lock.update("checking-resource", resource=name)
            status, remote = inspect_resource(
                name,
                reference_dir=reference_dir,
                manifest=manifest,
                max_age_days=args.max_age_days,
                timeout=args.timeout,
                remote_cache=remote_cache,
            )
            if status.status == "unknown":
                raise ReferenceError(f"{name}: {status.detail}")
            if not args.force and status.status in USABLE_STATUSES:
                print(f"{name}: {status.status}; skipped ({status.detail})")
                continue
            if remote is None:
                remote = load_remote_resource(
                    name, timeout=args.timeout, cache=remote_cache
                )

            destination = ensure_safe_cache_parent(reference_dir, RESOURCE_FILES[name])
            print(f"{name}: fetching {remote.source_url}")
            lock.update(
                "fetching-resource", resource=name, source_url=remote.source_url
            )
            size, digest, validation = atomic_download(
                remote, destination, timeout=args.timeout
            )
            record = {
                "path": RESOURCE_FILES[name],
                "source_url": remote.source_url,
                "source_updated_at": remote.source_updated_at,
                "retrieved_at": iso_now(),
                "size": size,
                "sha256": digest,
                **remote.metadata,
                **validation,
            }
            manifest["resources"][name] = record
            lock.update("recording-resource", resource=name)
            write_manifest(reference_dir, manifest)
            print(f"{name}: fetched {size} bytes; sha256={digest}")

        if any(name in DATABASE_INPUTS for name in targets):
            unavailable_inputs: list[str] = []
            for name in DATABASE_INPUTS:
                problem = local_problem(
                    name, reference_dir, manifest["resources"].get(name)
                )
                if problem:
                    status, detail, _ = problem
                    unavailable_inputs.append(f"{name} is {status}: {detail}")
            if unavailable_inputs and not all(
                name in targets for name in DATABASE_INPUTS
            ):
                print(
                    f"{DATABASE_RESOURCE_NAME}: skipped; "
                    + "; ".join(unavailable_inputs)
                )
            else:
                lock.update("building-index", schema_version=DATABASE_SCHEMA_VERSION)
                ensure_scryfall_database(reference_dir, manifest, force=False)
    return 0


def command_index(args: argparse.Namespace) -> int:
    require_shared_write_approval(args)
    repo_root = find_repo_root(args.repo_root)
    reference_dir = resolve_reference_dir(repo_root, args.reference_dir)
    with cache_lock(
        reference_dir,
        repo_root=repo_root,
        operation="index",
        exclusive=True,
        timeout=args.lock_timeout,
        details={
            "force": args.force,
            "schema_version": DATABASE_SCHEMA_VERSION,
            "human_approval_asserted": True,
        },
    ) as lock:
        lock.update("loading-manifest")
        manifest = load_manifest(reference_dir)
        lock.update("building-index", schema_version=DATABASE_SCHEMA_VERSION)
        ensure_scryfall_database(reference_dir, manifest, force=args.force)
    return 0


def command_path(args: argparse.Namespace) -> int:
    repo_root = find_repo_root(args.repo_root)
    reference_dir = resolve_reference_dir(repo_root, args.reference_dir)
    path = reference_path(reference_dir, args.target)
    if args.json:
        print(json.dumps({"target": args.target, "path": str(path)}, indent=2))
    else:
        print(path)
    return 0


def command_lock_status(args: argparse.Namespace) -> int:
    repo_root = find_repo_root(args.repo_root)
    reference_dir = resolve_reference_dir(repo_root, args.reference_dir)
    diagnostics = lock_diagnostics(reference_dir / LOCK_NAME)
    exit_code = (
        2
        if diagnostics["kernel_state"] == "unknown"
        or diagnostics["metadata_state"] == "metadata-in-transition"
        or (diagnostics["warnings"] and diagnostics["last_owner"] is None)
        else 0
    )
    if args.json:
        print(json.dumps(diagnostics, indent=2, sort_keys=True))
        return exit_code
    print(
        f"lock: {diagnostics['kernel_state']}; "
        f"metadata: {diagnostics['metadata_state']}"
    )
    owner = diagnostics["owner"]
    if owner:
        print(f"owner: {lock_owner_summary(owner)}")
        process_state = diagnostics["same_host_process"]
        if process_state is not None:
            print(f"same-host process: {process_state}")
    elif diagnostics["metadata_state"] == "active-writer-owner-unknown":
        print("owner: a writer holds the kernel lock, but its metadata is unavailable")
    elif diagnostics["last_owner"]:
        print(f"last writer: {lock_owner_summary(diagnostics['last_owner'])}")
    for warning in diagnostics["warnings"]:
        print(f"warning: {warning}")
    if diagnostics.get("probe_error"):
        print(f"probe error: {diagnostics['probe_error']}")
    if diagnostics["metadata_state"] == "stale-owner-metadata":
        print(
            "The last writer did not record release, but no exclusive writer holds "
            "the kernel lock. This is diagnostic residue; do not delete refresh.lock."
        )
    return exit_code


def command_migrate(args: argparse.Namespace) -> int:
    require_shared_write_approval(args)
    repo_root = find_repo_root(args.repo_root)
    reference_dir = resolve_reference_dir(repo_root, args.reference_dir)
    legacy_dir = resolve_configured_path(
        args.legacy_reference_dir or str(LEGACY_REFERENCE_DIR), base=repo_root
    )
    if not (legacy_dir / MANIFEST_NAME).is_file():
        raise ReferenceError(
            f"legacy cache has no manifest: {legacy_dir / MANIFEST_NAME}"
        )
    with cache_lock(
        reference_dir,
        repo_root=repo_root,
        operation="migrate",
        exclusive=True,
        timeout=args.lock_timeout,
        details={
            "legacy_dir": str(legacy_dir),
            "remove_source": args.remove_source,
            "human_approval_asserted": True,
        },
    ) as lock:
        lock.update("waiting-for-legacy-lock", legacy_lock=LEGACY_LOCK_NAME)
        with legacy_cache_lock(legacy_dir, timeout=args.lock_timeout):
            migrated, removed = migrate_legacy_cache(
                legacy_dir,
                reference_dir,
                remove_source=args.remove_source,
                lock=lock,
            )
    print(
        f"migration complete: {migrated} payloads migrated, {removed} legacy files removed"
    )
    return 0


def add_common_arguments(
    parser: argparse.ArgumentParser, *, include_database: bool = False
) -> None:
    choices = list(RESOURCE_FILES)
    if include_database:
        choices.append(DATABASE_RESOURCE_NAME)
    parser.add_argument(
        "resources",
        nargs="*",
        choices=sorted(choices),
        help="resources to process; omit for the default reference set",
    )
    parser.add_argument(
        "--repo-root",
        help="repository root; defaults to the nearest parent containing .git",
    )
    parser.add_argument(
        "--reference-dir",
        help=(
            "cache layout directory; overrides PENTA_REFERENCE_CACHE_DIR, Git "
            "config penta.referenceDir, and the Git-common default"
        ),
    )
    parser.add_argument(
        "--lock-timeout",
        type=float,
        default=5.0,
        help="maximum wait for the shared cache lock in seconds",
    )
    parser.add_argument(
        "--max-age-days",
        type=float,
        default=7.0,
        help="maximum age for gameplay-oriented Scryfall snapshots",
    )
    parser.add_argument(
        "--timeout",
        type=float,
        default=30.0,
        help="per-network-operation timeout in seconds",
    )


def add_location_arguments(
    parser: argparse.ArgumentParser, *, include_lock_timeout: bool = False
) -> None:
    parser.add_argument(
        "--repo-root",
        help="repository root; defaults to the nearest parent containing .git",
    )
    parser.add_argument(
        "--reference-dir",
        help=(
            "cache layout directory; overrides PENTA_REFERENCE_CACHE_DIR, Git "
            "config penta.referenceDir, and the Git-common default"
        ),
    )
    if include_lock_timeout:
        parser.add_argument(
            "--lock-timeout",
            type=float,
            default=5.0,
            help="maximum wait for the shared cache lock in seconds",
        )


def add_shared_write_approval(parser: argparse.ArgumentParser) -> None:
    parser.add_argument(
        "--approve-shared-write",
        action="store_true",
        help="confirm that a human approved this shared-cache mutation",
    )


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        description="Check and refresh penta's optional Magic reference cache.",
        formatter_class=argparse.ArgumentDefaultsHelpFormatter,
    )
    subparsers = parser.add_subparsers(dest="command", required=True)

    status_parser = subparsers.add_parser(
        "status",
        help="compare local references with their authoritative source metadata",
        formatter_class=argparse.ArgumentDefaultsHelpFormatter,
    )
    add_common_arguments(status_parser, include_database=True)
    status_parser.add_argument("--json", action="store_true", help="emit JSON status")
    status_parser.set_defaults(handler=command_status)

    fetch_parser = subparsers.add_parser(
        "fetch",
        help="fetch references and maintain the derived Scryfall SQLite index",
        formatter_class=argparse.ArgumentDefaultsHelpFormatter,
    )
    add_common_arguments(fetch_parser)
    fetch_parser.add_argument(
        "--force", action="store_true", help="replace resources even when still fresh"
    )
    add_shared_write_approval(fetch_parser)
    fetch_parser.set_defaults(handler=command_fetch)

    index_parser = subparsers.add_parser(
        "index",
        help="build or repair the derived Scryfall SQLite index without networking",
        formatter_class=argparse.ArgumentDefaultsHelpFormatter,
    )
    add_location_arguments(index_parser, include_lock_timeout=True)
    index_parser.add_argument(
        "--force", action="store_true", help="rebuild even when the index is current"
    )
    add_shared_write_approval(index_parser)
    index_parser.set_defaults(handler=command_index)

    path_parser = subparsers.add_parser(
        "path",
        help="print a resolved shared-cache path without reading or writing the cache",
        formatter_class=argparse.ArgumentDefaultsHelpFormatter,
    )
    add_location_arguments(path_parser)
    path_parser.add_argument(
        "target",
        nargs="?",
        default="cache-root",
        choices=sorted(
            [
                "cache-root",
                "manifest",
                "lock",
                DATABASE_RESOURCE_NAME,
                *RESOURCE_FILES,
            ]
        ),
        help="cache path to resolve",
    )
    path_parser.add_argument("--json", action="store_true", help="emit JSON output")
    path_parser.set_defaults(handler=command_path)

    lock_status_parser = subparsers.add_parser(
        "lock-status",
        help="diagnose the persistent shared-cache lock without modifying it",
        formatter_class=argparse.ArgumentDefaultsHelpFormatter,
    )
    add_location_arguments(lock_status_parser)
    lock_status_parser.add_argument(
        "--json", action="store_true", help="emit JSON diagnostics"
    )
    lock_status_parser.set_defaults(handler=command_lock_status)

    migrate_parser = subparsers.add_parser(
        "migrate",
        help="copy a legacy worktree-local cache into the shared cache without rebuilding",
        formatter_class=argparse.ArgumentDefaultsHelpFormatter,
    )
    add_location_arguments(migrate_parser, include_lock_timeout=True)
    migrate_parser.add_argument(
        "--legacy-reference-dir",
        help="legacy cache path; defaults to the repository's former local cache",
    )
    migrate_parser.add_argument(
        "--remove-source",
        action="store_true",
        help="remove validated legacy payloads after successful migration",
    )
    add_shared_write_approval(migrate_parser)
    migrate_parser.set_defaults(handler=command_migrate)
    return parser


def main() -> int:
    parser = build_parser()
    args = parser.parse_args()
    max_age_days = getattr(args, "max_age_days", None)
    if max_age_days is not None and (
        not math.isfinite(max_age_days) or max_age_days < 0 or max_age_days > 3650
    ):
        parser.error("--max-age-days must be between 0 and 3650")
    timeout = getattr(args, "timeout", None)
    if timeout is not None and (
        not math.isfinite(timeout) or timeout <= 0 or timeout > 60
    ):
        parser.error("--timeout must be between 0 and 60 seconds")
    lock_timeout = getattr(args, "lock_timeout", None)
    if lock_timeout is not None and (
        not math.isfinite(lock_timeout) or lock_timeout < 0 or lock_timeout > 3600
    ):
        parser.error("--lock-timeout must be between 0 and 3600 seconds")
    try:
        return args.handler(args)
    except (HTTPError, URLError, TimeoutError, OSError, ReferenceError) as error:
        print(f"error: {error}", file=sys.stderr)
        return 2


if __name__ == "__main__":
    raise SystemExit(main())

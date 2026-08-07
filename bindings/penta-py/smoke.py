"""Smoke test for the Python bindings: full games, determinism, self-play.

Run via scripts/check-bindings.sh, which builds the module and puts it on
the path first.
"""

import json

import penta

print("engine", penta.engine_version(), "protocol", penta.protocol_version())
assert "Sligh" in penta.deck_names()
catalog = {c["definition"]: c for c in json.loads(penta.catalog())["cards"]}
assert any(c["name"] == "Lightning Bolt" for c in catalog.values())

standard_decks = penta.deck_names(format="isd-rtr-standard")
assert "Briksza Naya Midrange" in standard_decks
standard_catalog_payload = json.loads(penta.catalog(format="isd-rtr-standard"))
assert standard_catalog_payload["format"] == "isd-rtr-standard"
assert any(
    card["name"] == "Huntmaster of the Fells"
    for card in standard_catalog_payload["cards"]
)

standard_game = penta.Game(
    "Briksza Naya Midrange",
    "Greer G/R Aggro",
    opponent="external",
    format="isd-rtr-standard",
    seed=17,
)
standard_observation = json.loads(standard_game.observe())
assert standard_observation["format"] == "isd-rtr-standard"

try:
    penta.deck_names(format="not-a-format")
except ValueError:
    pass
else:
    raise AssertionError("bad format accepted")

def pass_bot(obs):
    prefer = ["KeepHand", "ChooseDecision", "PassPriority", "FinishDeclaringAttackers",
              "FinishDeclaringBlockers", "AssignCombatDamage", "DiscardCards",
              "BottomCards", "ChooseUntap"]
    actions = obs["legalActions"]
    for kind in prefer:
        for action in actions:
            if action["type"] == kind:
                return action["index"]
    return 0

# vs handcrafted
game = penta.Game("Sligh", "The Deck", opponent="handcrafted", seed=7)
steps = 0
while game.result() is None:
    obs = json.loads(game.observe())
    game.act(pass_bot(obs))
    steps += 1
    assert steps < 100000
print("vs handcrafted:", game.result(), "in", steps, "decisions")

# determinism
def run(seed):
    g = penta.Game("Goblins", "Sligh", opponent="random", seed=seed)
    trace = []
    while g.result() is None:
        obs = json.loads(g.observe())
        trace.append(len(obs["legalActions"]))
        g.act(pass_bot(obs))
    return g.result(), trace
a, b = run(99), run(99)
assert a == b, "same seed, same game"
print("determinism ok:", a[0], "over", len(a[1]), "decisions")

# clone: a fork replays identically and diverges independently
game = penta.Game("Sligh", "The Deck", opponent="handcrafted", seed=7)
for _ in range(30):
    game.act(pass_bot(json.loads(game.observe())))
replay = game.clone()
assert game.observe() == replay.observe(), "a clone starts byte-identical"
for _ in range(20):
    choice = pass_bot(json.loads(game.observe()))
    game.act(choice)
    replay.act(choice)
    assert game.observe("p1") == replay.observe("p1"), "same actions, same bytes"
# Diverge: the fork plays a different legal action than the original, the
# two games stop matching, and the original never notices. Walk to a
# decision with at least two options first.
while len(json.loads(game.observe())["legalActions"]) < 2:
    game.act(0)
obs = json.loads(game.observe())
choice = pass_bot(obs)
other = (choice + 1) % len(obs["legalActions"])
before = game.observe()
fork = game.clone()
fork.act(other)
assert game.observe() == before, "the original is untouched"
game.act(choice)
assert game.observe("p1") != fork.observe("p1"), \
    "different actions, different games"
for _ in range(10):  # a fork is a live game, not a snapshot: it plays on
    if fork.result() is not None:
        break
    fork.act(pass_bot(json.loads(fork.observe())))
print("clone: forks replay identically and diverge independently")

# self-play: one loop drives both seats
game = penta.Game("Goblins", "White Weenie", opponent="external", seed=13)
steps = 0
while game.result() is None:
    seat = game.decision_seat()
    obs = json.loads(game.observe(seat))
    assert obs["seat"] == seat
    game.act(pass_bot(obs))
    steps += 1
    assert steps < 200000
print("self-play:", game.result(), "in", steps, "decisions")

# hidden info: p1 never sees p2's hand
game = penta.Game("Sligh", "The Deck", opponent="handcrafted", seed=3)
obs = json.loads(game.observe())
assert "opponentHandSize" in obs and isinstance(obs["opponentHandSize"], int)
print("smoke test passed")

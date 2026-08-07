import { GameClient } from "./GameClient";

export default function Home() {
  // This is the human-facing game route. Bot runners and tests construct the
  // engine directly, while any reusable GameClient render stays fail-closed.
  return <GameClient cardArtMode="scryfall" />;
}

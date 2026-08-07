export type FormatId = "old-school-93-94" | "isd-rtr-standard";

type FormatConfig = {
  name: string;
  shortName: string;
  cardBackMark: string;
  description: string;
  deckNotes: Record<string, string>;
};

export const formatConfigs: Record<FormatId, FormatConfig> = {
  "old-school-93-94": {
    name: "Old School 93/94",
    shortName: "OLD SCHOOL · 93/94",
    cardBackMark: "93",
    description: "Eternal Central rules · mana burn · powered archetypes",
    deckNotes: {
      Goblins: "Tribal pressure · Grenade finish",
      Sligh: "Clean curve · Burn reach",
      Artifacts: "Fast mana · Atog engine",
      Robots: "Fast mana · Heavy artifact creatures",
      "The Deck": "Five-color control · Tome inevitability",
      "Mono Black": "Ritual starts · Discard and land destruction",
      "White Weenie": "Efficient threats · Crusade and Armageddon",
      Erhnamgeddon: "Green-white midrange · Armageddon lock",
      Counterburn: "Blue-red tempo · Counters and direct damage",
      "Lions DIB": "Blue-white tempo · Cheap threats and permission",
      "Lion Dib Bolt": "Blue-white tempo · Cheap threats and burn",
      "BWR Aggro": "Three-color pressure · Knights and burn",
      "GR Aggro": "Green-red pressure · Efficient creatures and tricks",
      "Troll Disk": "Black-red control · Trolls and sweepers",
      "Jeskai Aggro": "Blue-white-red tempo · Burn and permission",
    },
  },
  "isd-rtr-standard": {
    name: "ISD–RTR Standard",
    shortName: "ISD–RTR STANDARD · 2013",
    cardBackMark: "13",
    description: "Final pre-Theros pool · no mana burn · SCG Atlanta Top 8 · staged card effects",
    deckNotes: {
      "Briksza Naya Midrange": "1st · Rudy Briksza · Naya midrange",
      "Greer G/R Aggro": "2nd · Joseph Greer · G/R aggro",
      "Fyrberg B/G Midrange": "3rd · Mike Fyrberg · B/G midrange",
      "Smith Naya Midrange": "4th · Jimmie Smith · Naya midrange",
      "McDuffie U/W/R Flash": "5th · Korey McDuffie · U/W/R flash",
      "Lorren U/W Flash": "6th · Phillip Lorren · U/W flash",
      "Arch U/W Flash": "7th · Clayton Arch · U/W flash",
      "Kuenzinger Junk Reanimator": "8th · Drew Kuenzinger · Junk reanimator",
    },
  },
};

export const formatIds: FormatId[] = ["old-school-93-94", "isd-rtr-standard"];

export const defaultFormat: FormatId = "old-school-93-94";

export const isFormatId = (candidate: string | null): candidate is FormatId =>
  candidate !== null && formatIds.includes(candidate as FormatId);

/// Backwards-compatible view of the original default-format deck registry.
export const deckNotes = formatConfigs[defaultFormat].deckNotes;

/// Stands in for a deck in the setup dialog until the game is dealt, at which
/// point it resolves to a concrete deck from the selected format.
export const randomDeck = "Random";

export const randomDeckNote = "Rolled fresh every time you deal";

export const deckNamesForFormat = (format: FormatId) =>
  Object.keys(formatConfigs[format].deckNotes);

export const deckChoicesForFormat = (format: FormatId) => [
  randomDeck,
  ...deckNamesForFormat(format),
];

export const placeholderDeckForFormat = (format: FormatId) =>
  deckNamesForFormat(format)[0];

export const deckChoiceNote = (format: FormatId, choice: string) =>
  choice === randomDeck
    ? randomDeckNote
    : (formatConfigs[format].deckNotes[choice] ?? "");

export const defaultHumanDeck = randomDeck;
export const defaultBotDeck = randomDeck;

// `label` is the engine's identifier for a phase stop; `title` is what the
// player reads. They differ because the rules call it Main 2 and nobody else does.
export const turnPhases = [
  { label: "Beginning", title: "Beginning", steps: ["Upkeep", "Draw"] },
  { label: "Main 1", title: "First Main", steps: ["Precombat Main"] },
  {
    label: "Combat",
    title: "Combat",
    steps: [
      "Beginning Of Combat",
      "Declare Attackers",
      "Declare Blockers",
      "Combat Damage",
      "End Of Combat",
    ],
  },
  { label: "Main 2", title: "Second Main", steps: ["Postcombat Main"] },
  { label: "Ending", title: "Ending", steps: ["End", "Cleanup"] },
] as const;

// The board itself tells the story now — each beat only needs to be long
// enough to watch the cards move.
export const opponentActionDurationMs = 2000;

// A draw is one card moving and happens every single turn, so it gets just
// enough time to be seen leaving the library.
export const drawBeatDurationMs = 900;

export const turnBannerDurationMs = 1800;

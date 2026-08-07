/* C interface to the penta engine.
 *
 * Build the library with:  cargo build --release -p penta-ffi
 * then link  target/release/libpenta_ffi.a  (or the shared library) and
 * include this header. All JSON crossing this boundary follows the penta
 * bot protocol; see BOTS.md at the repository root for the schema.
 *
 * Ownership:
 *   - char *       returns are owned by you; free with penta_string_free().
 *   - const char * returns are borrowed; do not free them.
 *   - PentaGame *  returns are freed with penta_free().
 *
 * Thread safety: a PentaGame may only be used from one thread at a time.
 * penta_last_error() is thread-local.
 */

#ifndef PENTA_H
#define PENTA_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/* An in-progress game. Opaque. */
typedef struct PentaGame PentaGame;

/* The engine crate version, e.g. "0.3.0". Static; never free. */
const char *penta_engine_version(void);

/* The protocol version the JSON shapes follow. */
uint32_t penta_protocol_version(void);

/* The most recent error on this thread. Empty string until something
 * fails; valid until the next failing call. Never free. */
const char *penta_last_error(void);

/* Every card definition with Old School legality, as protocol JSON. */
char *penta_catalog_json(void);

/* Every card definition with legality for format, as protocol JSON.
 * format is "old-school-93-94" or "isd-rtr-standard". */
char *penta_catalog_json_for_format(const char *format);

/* The Old School built-in deck names, as a JSON array of strings. */
char *penta_deck_names_json(void);

/* The built-in deck names for format, as a JSON array of strings. */
char *penta_deck_names_for_format_json(const char *format);

/* Starts a game. config_json looks like:
 *
 *   {"format": "old-school-93-94",
 *    "p1Deck": "Sligh", "p2Deck": "The Deck",
 *    "opponent": "handcrafted", "opponentSeat": "p2", "seed": 42}
 *
 * format is optional and defaults to "old-school-93-94".
 * opponent is "random", "handcrafted", or "external" (you drive both
 * seats). Returns NULL on error; see penta_last_error(). */
PentaGame *penta_new(const char *config_json);

/* The seat that must act next: 0 for p1, 1 for p2, -1 when the game is
 * over. With a scripted opponent this is always your seat. */
int32_t penta_decision_seat(const PentaGame *game);

/* How many legal actions the acting seat has; 0 when the game is over.
 * Lets minimal clients act without parsing JSON. */
uint32_t penta_legal_action_count(const PentaGame *game);

/* One seat's observation as protocol JSON (seat: 0 or 1). Its
 * "legalActions" array carries the indices penta_act() accepts. */
char *penta_observe_json(const PentaGame *game, int32_t seat);

/* Plays one index from the acting seat's legalActions, then lets a
 * scripted opponent play until you have a real choice again. Returns 0
 * on success, -1 on error. */
int32_t penta_act(PentaGame *game, uint32_t action_index);

/* Answers a pending decision with explicit option ids (from the
 * observation's "decision" object), for multi-pick decisions where the
 * default in legalActions is not wanted. 0 on success, -1 on error. */
int32_t penta_choose_decision(PentaGame *game, const uint32_t *option_ids,
                              uint32_t count);

/* -1 while the game runs, 0 for a draw, 1 when p1 won, 2 when p2 won. */
int32_t penta_result(const PentaGame *game);

/* Free a string returned by any *_json function. NULL is a no-op. */
void penta_string_free(char *string);

/* Free a game. NULL is a no-op. */
void penta_free(PentaGame *game);

#ifdef __cplusplus
}
#endif

#endif /* PENTA_H */

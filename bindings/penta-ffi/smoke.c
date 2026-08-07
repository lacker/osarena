/* Smoke test for the C ABI: plays full games against both built-in bots
 * choosing pseudo-random legal actions, and checks the JSON surface looks
 * like the protocol. Run via scripts/check-bindings.sh. */

#include "include/penta.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static unsigned long rng_state = 12345;

static unsigned long next_rand(void) {
    /* Deterministic LCG so the smoke test never flakes. */
    rng_state = rng_state * 6364136223846793005UL + 1442695040888963407UL;
    return rng_state >> 33;
}

static int fail(const char *what) {
    fprintf(stderr, "FAIL: %s: %s\n", what, penta_last_error());
    return 1;
}

/* Plays one uniform-random action on game, and the same index on mirror
 * when mirror is not NULL, so two copies can be driven in lockstep. */
static int act_random(PentaGame *game, PentaGame *mirror) {
    uint32_t count = penta_legal_action_count(game);
    if (count == 0) {
        fprintf(stderr, "FAIL: no legal actions but no result\n");
        return 1;
    }
    uint32_t pick = (uint32_t)(next_rand() % count);
    if (penta_act(game, pick) != 0)
        return fail("penta_act");
    if (mirror && penta_act(mirror, pick) != 0)
        return fail("penta_act on a clone");
    return 0;
}

static int play_one(const char *config, int check_json) {
    PentaGame *game = penta_new(config);
    if (!game) return fail("penta_new");

    if (check_json) {
        int32_t seat = penta_decision_seat(game);
        char *observation = penta_observe_json(game, seat);
        if (!observation) return fail("penta_observe_json");
        if (!strstr(observation, "\"legalActions\"") ||
            !strstr(observation, "\"seat\"") ||
            !strstr(observation, "\"protocolVersion\"")) {
            fprintf(stderr, "FAIL: observation missing protocol fields\n");
            return 1;
        }
        penta_string_free(observation);
    }

    /* Uniform random over the whole list. Nothing in it resigns, so this
     * plays a real if witless game rather than ending on turn one. */
    int steps;
    for (steps = 0; steps < 200000; steps++) {
        if (penta_result(game) != -1) break;
        if (act_random(game, NULL) != 0) return 1;
    }

    int32_t result = penta_result(game);
    penta_free(game);
    if (result == -1) {
        fprintf(stderr, "FAIL: game did not finish in %d steps\n", steps);
        return 1;
    }
    printf("ok: result=%d after %d of your decisions\n", result, steps);
    return 0;
}

static int check_standard_game(void) {
    PentaGame *game = penta_new(
        "{\"format\":\"isd-rtr-standard\","
        "\"p1Deck\":\"Briksza Naya Midrange\","
        "\"p2Deck\":\"Greer G/R Aggro\","
        "\"opponent\":\"external\",\"seed\":17}");
    if (!game) return fail("penta_new Standard");

    int32_t seat = penta_decision_seat(game);
    char *observation = penta_observe_json(game, seat);
    if (!observation) {
        penta_free(game);
        return fail("penta_observe_json Standard");
    }
    int valid = strstr(observation, "\"format\":\"isd-rtr-standard\"") != NULL;
    penta_string_free(observation);
    penta_free(game);
    if (!valid) {
        fprintf(stderr, "FAIL: Standard observation has the wrong format\n");
        return 1;
    }
    return 0;
}

/* A cloned game replays identically under the same actions, and acting on
 * one copy never disturbs the other. */
static int check_clone(const char *config) {
    PentaGame *game = penta_new(config);
    if (!game) return fail("penta_new");

    /* Reach a mid-game state. */
    for (int step = 0; step < 30; step++) {
        if (act_random(game, NULL) != 0) return 1;
        if (penta_result(game) != -1) {
            fprintf(stderr, "FAIL: game ended before the clone check\n");
            return 1;
        }
    }

    /* Identical bytes at the fork, and after the same actions. */
    PentaGame *replay = penta_clone(game);
    if (!replay) return fail("penta_clone");
    for (int step = 0; step < 10; step++) {
        int32_t seat = penta_decision_seat(game);
        char *original = penta_observe_json(game, seat);
        char *cloned = penta_observe_json(replay, seat);
        if (!original || !cloned) return fail("penta_observe_json");
        if (strcmp(original, cloned) != 0) {
            fprintf(stderr, "FAIL: clone diverged under identical actions\n");
            return 1;
        }
        penta_string_free(original);
        penta_string_free(cloned);
        if (act_random(game, replay) != 0) return 1;
        if (penta_result(game) != penta_result(replay)) {
            fprintf(stderr, "FAIL: clone and original disagree on result\n");
            return 1;
        }
        if (penta_result(game) != -1) break;
    }
    penta_free(replay);

    /* Diverge: the fork plays a different legal action than the original,
     * the two games stop matching, and the original never notices. */
    if (penta_result(game) == -1) {
        /* Walk to a decision with at least two options to disagree on. */
        while (penta_result(game) == -1 && penta_legal_action_count(game) < 2) {
            if (act_random(game, NULL) != 0) return 1;
        }
        if (penta_result(game) != -1) {
            fprintf(stderr, "FAIL: game ended before the divergence check\n");
            return 1;
        }
        uint32_t count = penta_legal_action_count(game);
        uint32_t choice = (uint32_t)(next_rand() % count);
        uint32_t other = (choice + 1) % count;
        int32_t seat = penta_decision_seat(game);
        char *before = penta_observe_json(game, seat);
        if (!before) return fail("penta_observe_json");
        PentaGame *fork = penta_clone(game);
        if (!fork) return fail("penta_clone");
        if (penta_act(fork, other) != 0)
            return fail("penta_act on a fork");
        char *after = penta_observe_json(game, seat);
        if (!after) return fail("penta_observe_json");
        if (strcmp(before, after) != 0) {
            fprintf(stderr, "FAIL: the original changed when its fork acted\n");
            return 1;
        }
        penta_string_free(before);
        penta_string_free(after);
        if (penta_act(game, choice) != 0) return fail("penta_act");
        char *original = penta_observe_json(game, seat);
        char *forked = penta_observe_json(fork, seat);
        if (!original || !forked) return fail("penta_observe_json");
        if (strcmp(original, forked) == 0) {
            fprintf(stderr, "FAIL: different actions, same observation\n");
            return 1;
        }
        penta_string_free(original);
        penta_string_free(forked);
        /* A fork is a live game, not a snapshot: it plays on by itself. */
        for (int step = 0; step < 10; step++) {
            if (penta_result(fork) != -1) break;
            if (act_random(fork, NULL) != 0) return 1;
        }
        penta_free(fork);
    }
    penta_free(game);
    printf("clone: forks replay identically and diverge independently\n");
    return 0;
}

int main(void) {
    printf("engine %s, protocol %u\n", penta_engine_version(),
           penta_protocol_version());

    char *decks = penta_deck_names_json();
    if (!decks || !strstr(decks, "Sligh")) return fail("penta_deck_names_json");
    penta_string_free(decks);

    char *standard_decks =
        penta_deck_names_for_format_json("isd-rtr-standard");
    if (!standard_decks || !strstr(standard_decks, "Briksza Naya Midrange"))
        return fail("penta_deck_names_for_format_json");
    penta_string_free(standard_decks);

    char *catalog = penta_catalog_json();
    if (!catalog || !strstr(catalog, "Lightning Bolt"))
        return fail("penta_catalog_json");
    penta_string_free(catalog);

    char *standard_catalog =
        penta_catalog_json_for_format("isd-rtr-standard");
    if (!standard_catalog ||
        !strstr(standard_catalog, "\"format\":\"isd-rtr-standard\"") ||
        !strstr(standard_catalog, "Huntmaster of the Fells"))
        return fail("penta_catalog_json_for_format");
    penta_string_free(standard_catalog);

    if (check_standard_game()) return 1;

    /* Random moves against each built-in opponent, and a self-play game. */
    if (play_one("{\"p1Deck\":\"Sligh\",\"p2Deck\":\"The Deck\","
                 "\"opponent\":\"handcrafted\",\"opponentSeat\":\"p2\","
                 "\"seed\":7}", 1))
        return 1;
    if (play_one("{\"p1Deck\":\"Goblins\",\"p2Deck\":\"White Weenie\","
                 "\"opponent\":\"random\",\"opponentSeat\":\"p2\","
                 "\"seed\":11}", 0))
        return 1;
    if (play_one("{\"p1Deck\":\"Sligh\",\"p2Deck\":\"Goblins\","
                 "\"opponent\":\"external\",\"seed\":13}", 0))
        return 1;

    /* Clones: fork a game mid-state and check the copies are independent. */
    if (check_clone("{\"p1Deck\":\"Sligh\",\"p2Deck\":\"The Deck\","
                    "\"opponent\":\"handcrafted\",\"opponentSeat\":\"p2\","
                    "\"seed\":7}"))
        return 1;

    /* Error paths report through penta_last_error instead of crashing. */
    if (penta_new("{\"p1Deck\":\"Not A Deck\",\"p2Deck\":\"Sligh\"}") != NULL) {
        fprintf(stderr, "FAIL: bad deck accepted\n");
        return 1;
    }
    if (strlen(penta_last_error()) == 0) {
        fprintf(stderr, "FAIL: bad deck left no error message\n");
        return 1;
    }
    if (penta_catalog_json_for_format("not-a-format") != NULL) {
        fprintf(stderr, "FAIL: bad format accepted\n");
        return 1;
    }
    if (strlen(penta_last_error()) == 0) {
        fprintf(stderr, "FAIL: bad format left no error message\n");
        return 1;
    }

    printf("smoke test passed\n");
    return 0;
}

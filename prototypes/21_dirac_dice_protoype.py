import numpy as np
from tqdm import tqdm



def quantum_roll() -> np.ndarray:
    row = np.array([1., 3., 6., 7., 6., 3., 1.], dtype=np.int64).reshape(-1, 1)
    res = np.matmul(row, row.reshape(1, -1))
    # print(res)
    return res


def game_step(game_state, max_score):
    p1_pos, p2_pos, p1_score, p2_score, count = game_state
    outcomes = []

    for p1_roll_idx, p1_roll_count in     enumerate([1,3,6,7,6,3,1]):
        p1_roll = p1_roll_idx + 3
        for p2_roll_idx, p2_roll_count in enumerate([1,3,6,7,6,3,1]):
            p2_roll = p2_roll_idx + 3

            new_count = count * (p1_roll_count * p2_roll_count)
            new_p1_state = p1_pos + p1_roll
            if new_p1_state > 10:
                new_p1_state -= 10
            # assert 1 <= new_p1_state <= 10

            new_p1_score = p1_score + new_p1_state
            if new_p1_score >= max_score:
                outcomes.append(("p1_win", new_count))
                continue

            new_p2_state = p2_pos + p2_roll
            if new_p2_state > 10:
                new_p2_state -= 10
            # assert 1 <= new_p2_state <= 10

            new_p2_score = p2_score + new_p2_state
            if new_p2_score >= max_score:
                outcomes.append(("p2_win", new_count))
                continue

            # print(count, new_count)
            outcomes.append(("game", (new_p1_state, new_p2_state, new_p1_score, new_p2_score, new_count)))

    return outcomes



def update_quantum_state(state, roll, max_score):
    # Basically a histogram filter update - each value in state "moves" by each value in roll

    rows, cols, score_rows, score_cols = state.shape
    out = np.zeros_like(state)
    new_p1_wins = 0
    new_p2_wins = 0

    for in_row_idx in range(1, rows):
        for in_col_idx in range(1, cols):
            for in_p1_score in range(max_score + 1):
                for in_p2_score in range(max_score + 1):

                    count = state[in_row_idx][in_col_idx][in_p1_score][in_p2_score]

                    # Avoids MANY state updates!
                    if count == 0:
                        continue

                    # for p1_roll, p1_roll_count in enumerate([1,3,6,7,6,3,1]):
                    #     for p2_roll, p2_roll_count in enumerate([1,3,6,7,6,3,1]):
                    #         # # The roll val changes the state.
                    #         # roll_val = roll_row[roll_col_idx]
                    #         # assert roll_val != 0
                    #         # new_val = count * roll_val
                    #         new_val = (p1_roll_count * p2_roll_count) * count

                    #         # Roll indexes need a 1-offset since roll[0][0] is p(1, 1), since that's the lowest
                    #         # possible roll.
                    #         new_p1_state = (in_row_idx + p1_roll + 1) % rows
                    #         new_p2_state = (in_col_idx + p2_roll + 1) % cols
                    #         new_p1_score = in_p1_score + (new_p1_state + 1)
                    #         new_p2_score = in_p2_score + (new_p2_state + 1)

                    #         out[new_p1_state][new_p2_state][new_p1_score][new_p2_score] += new_val

                    # for roll_row_idx in range(len(roll)):
                    #     roll_row = roll[roll_row_idx]
                    #     for roll_col_idx in range(len(roll_row)):
                    for p1_roll, p1_roll_count in enumerate([1,3,6,7,6,3,1]):

                        # While browsing a Reddit thread with people restating the problem idea, it hit me - the
                        # second player's forks NEVER HAPPEN if P1 already won!
                        # Roll indexes need an offset since roll[0][0] is p(3, 3), since that's the lowest
                        # possible roll.
                        new_p1_state = (in_row_idx + p1_roll + 3)
                        if new_p1_state > 10:
                            # print(new_p1_state, "->", new_p1_state - 10)
                            new_p1_state -= 10

                        new_p1_score = in_p1_score + new_p1_state
                        if new_p1_score >= max_score:
                            new_p1_wins += count * p1_roll_count
                            continue

                        for p2_roll, p2_roll_count in enumerate([1,3,6,7,6,3,1]):
                            new_p2_state = (in_col_idx + p2_roll + 3)
                            if new_p2_state > 10:
                                new_p2_state -= 10
                            # if roll_val == 49:
                            #     print(roll_row_idx + 3, roll_col_idx + 3)

                            new_p2_score = in_p2_score + new_p2_state
                            if new_p2_score >= max_score:
                                new_p2_wins += count * p1_roll_count * p2_roll_count
                                continue

                            out[new_p1_state][new_p2_state][new_p1_score][new_p2_score] += count * p1_roll_count * p2_roll_count

    return out, new_p1_wins, new_p2_wins


def count_winners_and_stop_their_games(state, win_score: int):
    # Look at entities in any winning state (P1 or P2), and stop those games.

    init_games = state.sum()

    # If we have games where both players are at '> win_score', then P1 got there first by definition.
    # p1_win_mask = (:, :, win_score:, :)
    # print(state.shape)
    # print(p1_win_mask.shape)
    n_p1_wins = state[:, :, win_score:, :].sum()
    state[:, :, win_score:, :] = 0

    # p2_win_mask = state[:, :, :, win_score:]
    n_p2_wins = state[:, :, :, win_score:].sum()
    if state[:, :, win_score:, win_score:].sum() != 0:
        raise ValueError()
    state[:, :, :, win_score:] = 0

    final_games = state.sum()
    # print(n_p1_wins, n_p2_wins, n_p1_wins + n_p2_wins)
    # print(final_games, init_games, init_games - final_games)
    assert (n_p1_wins + n_p2_wins) == (init_games - final_games)

    return n_p1_wins, n_p2_wins


def day_21_dirac_dice():
    max_score = 21
    max_jump = 10
    state = np.zeros(shape=(11, 11, max_score + max_jump, max_score + max_jump), dtype=np.int64)
    # The starting universe - just one
    # Both players start with score 0 - so there's one initial universe, and it has a (0, 0) state.
    # Practice position (4, 8)
    # state[4][8][0][0] = 1
    # Contest position (7, 3)
    state[7][3][0][0] = 1

    n_p1_wins = 0
    n_p2_wins = 0

    active_games = [(4, 8, 0, 0, 1)]

    # for round_idx in range(12):
    #     print(f"Round #{round_idx}")
    #     new_games = []
    #     print("Will update {} active games".format(len(active_games)))
    #     for g in tqdm(active_games):
    #         outcomes = game_step(g, max_score=max_score)
    #         for o in outcomes:
    #             if o[0] == "p1_win":
    #                 n_p1_wins += o[1]
    #             elif o[0] == "p2_win":
    #                 n_p2_wins += o[1]
    #             elif o[0] == "game":
    #                 new_games.append(o[1])
    #             else:
    #                 raise ValueError

    #     print(f"{n_p1_wins=}, {n_p2_wins}")
    #     active_games = new_games


    for round_idx in range(12):
        print(f"Round #{round_idx}")
        roll = quantum_roll()
        new_state, new_wins_p1, new_wins_p2 = update_quantum_state(state, roll, max_score)
        n_active_games = np.sum(new_state)
        print(f"{n_active_games=}")
        print("games before: {}".format(new_state.sum()))
        aux_new_wins_p1, aux_new_wins_p2 = count_winners_and_stop_their_games(new_state, max_score)
        print(aux_new_wins_p1, aux_new_wins_p2)
        print("games after: {}".format(new_state.sum()))
        print(f"{new_wins_p1=}, {new_wins_p2}")
        n_p1_wins += new_wins_p1
        n_p2_wins += new_wins_p2
        state = new_state

    # old cur total:          5,765,538,026,405,977
    # offset fix:            12,339,574,895,141,313 (wtf)
    # track state from 1:    12,339,574,895,141,313
    # list-based baseline:
    # P1 wins 02-28:         11,997,614,504,960,505 (wtf x 2) - I feel like I'm misunderstanding the problem
    # P2 wins 02-28:            341,960,390,180,808
    # Yep, confirmed 02-28... I get the same result in my derpy as in my fast solution. So something else must be wrong.
    #
    #                         3,110,492,649,434,205
    # I got it right by dividing this value exactly by 7. WTF. Why??
    #
    # P1 wins 02-18:         11,997,614,504,960,505
    # expected P1 wins:         444,356,092,776,315
    # expected P2 wins:         341,960,390,180,808
    print(f"{n_p1_wins=}, {n_p2_wins=}")
    print("Total: {}".format(n_p1_wins + n_p2_wins))



if __name__ == "__main__":
    day_21_dirac_dice()
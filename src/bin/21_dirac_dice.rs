use std::cell::RefCell;
/// 2021 AoC Day 21: Dirac Dice
///
/// Rolling dice and forking universes - keep track of every possible roll of a set of dice at every round, using the
/// now very familiar histogram pattern to avoid exponential blow-ups. Special care must be taken in order to properly
/// account for when a game ends.
///
// Demo input
// Player 1 starting position: 4
// Player 2 starting position: 8
//
// Contest input
// Player 1 starting position: 7
// Player 2 starting position: 3
//
/// Hint used: I looked at the Reddit thread and the high-level things were already things I knew, like the fact that a
/// lot of game instances are actually duplicated many, many times. One hint, which is basically a less "fancy" way of
/// solving the problem is to keep a list of active games and their count, even if it may have a few dupes, and process
/// game tuples in the list basis, rather than being overly fancy about it with a fully tabulated state space. Let's see
/// if that works...
///
/// This hint was not useful - turns out, I was already one step ahead of this approach and I ended up getting the same
/// (incorrect) result with the list-based implementation. After a bit of thread reading (still not looking at code!),
/// looking at posts in which people were re-stating the problem idea, it hit me - the second player's "universe forks"
/// NEVER HAPPEN if the first player already won!
///
/// This was the final bug I had to fix in order to get the correct answer to this question!
use std::rc::Rc;

type UniverseHistogram = Vec<Vec<Vec<Vec<usize>>>>;

trait Die {
    fn roll(&mut self) -> usize;
}

struct DeterministicDie {
    n_sides: usize,
    _next_roll: usize,
    _n_rolls: usize,
}

impl Die for DeterministicDie {
    fn roll(&mut self) -> usize {
        let roll = self._next_roll;
        self._next_roll += 1;
        if self._next_roll > self.n_sides {
            self._next_roll = 1;
        }
        self._n_rolls += 1;
        roll
    }
}

impl DeterministicDie {
    fn new(n_sides: usize) -> Self {
        Self {
            n_sides: n_sides,
            _next_roll: 1,
            _n_rolls: 0,
        }
    }
}

struct Player {
    die: Rc<RefCell<dyn Die>>,
    score: usize,
    state: usize,
}

impl Player {
    fn new(start_state: usize, die: Rc<RefCell<dyn Die>>) -> Self {
        Self {
            die: die,
            score: 0,
            state: start_state,
        }
    }

    fn turn(&mut self) -> usize {
        let r1 = self.die.borrow_mut().roll();
        let r2 = self.die.borrow_mut().roll();
        let r3 = self.die.borrow_mut().roll();

        let jmp = (r1 + r2 + r3) as usize;
        self.state = (self.state + jmp) % 10;

        self.score += self.state + 1;

        self.score
    }
}

fn play(p1: &mut Player, p2: &mut Player, goal: usize) -> (usize, usize) {
    let mut p1_score: usize = 0;
    let mut p2_score: usize = 0;
    loop {
        p1_score = p1.turn();
        if p1_score >= goal {
            break;
        }
        p2_score = p2.turn();
        if p2_score >= goal {
            break;
        }
    }

    (p1_score, p2_score)
}

fn part_1() {
    let n_sides = 100;
    let mut die = DeterministicDie::new(n_sides);
    // let (start_p1, start_p2) = (3, 7);       // demo
    let (start_p1, start_p2) = (6, 2); // real

    let die_box = Rc::new(RefCell::new(die));

    let mut p1 = Player::new(start_p1, die_box.clone());
    let mut p2 = Player::new(start_p2, die_box.clone());

    let (p1_fin_score, p2_fin_score) = play(&mut p1, &mut p2, 1000);
    let part1_res = if p1_fin_score >= 1000 {
        println!("P1 won!! {} {}", p2_fin_score, die_box.borrow()._n_rolls);
        p2_fin_score * die_box.borrow()._n_rolls
    } else {
        println!("P2 won!! {} {}", p1_fin_score, die_box.borrow()._n_rolls);
        p1_fin_score * die_box.borrow()._n_rolls
    };
    // 867 888 is too big..., but I had '==' instead of '>=' in the final if-statement...
    println!("Part 1 result: {}", part1_res);
}

fn update_state(
    state: &UniverseHistogram,
    out_state: &mut UniverseHistogram,
    max_score: usize,
) -> (usize, usize) {
    let mut p1_wins = 0usize;
    let mut p2_wins = 0usize;

    for p1_pos in 1..=10 {
        for p2_pos in 1..=10 {
            for p1_score in 0..max_score {
                for p2_score in 0..max_score {
                    // TOOD(andrei): Can we use a fast memset-like op here?
                    out_state[p1_pos][p2_pos][p1_score][p2_score] = 0usize;
                }
            }
        }
    }

    for p1_pos in 1..=10 {
        for p2_pos in 1..=10 {
            for p1_score in 0..max_score {
                for p2_score in 0..max_score {
                    let count = state[p1_pos][p2_pos][p1_score][p2_score];
                    if 0usize == count {
                        continue;
                    }

                    for (p1_roll, p1_roll_count) in vec![1, 3, 6, 7, 6, 3, 1].iter().enumerate() {
                        // This logic must be done in the outer roll loop (P1's rolls), as otherwise if we just do
                        // `p1_wins += ...` and continue, we will go to the next P2 roll value (which is also
                        // technically never reached), add P1's wins again, etc., thereby multi-counting P1's rolls once
                        // for every value of the inner loop - hence the exact 7x over-estimation of P1's win count bug.
                        let p1_roll_val = p1_roll + 3;
                        let mut new_p1_pos = p1_pos + p1_roll_val;
                        if new_p1_pos > 10 {
                            new_p1_pos -= 10;
                        }
                        let new_p1_score = p1_score + new_p1_pos;
                        if new_p1_score >= max_score {
                            p1_wins += p1_roll_count * count;
                            continue;
                        }

                        for (p2_roll, p2_roll_count) in vec![1, 3, 6, 7, 6, 3, 1].iter().enumerate()
                        {
                            let p2_roll_val: usize = p2_roll + 3;
                            let mut new_p2_pos: usize = p2_pos + p2_roll_val;
                            if new_p2_pos > 10 {
                                new_p2_pos -= 10;
                            }
                            let new_p2_score = p2_score + new_p2_pos;
                            if new_p2_score >= max_score {
                                p2_wins += p1_roll_count * p2_roll_count * count;
                            } else {
                                // No winner so far
                                out_state[new_p1_pos][new_p2_pos][new_p1_score][new_p2_score] +=
                                    p1_roll_count * p2_roll_count * count;
                            }
                        }
                    }
                }
            }
        }
    }

    (p1_wins, p2_wins)
}

fn part_2() {
    let mut state_a = vec![vec![vec![vec![0usize; 21usize]; 21usize]; 11usize]; 11usize];
    let mut state_b = vec![vec![vec![vec![0usize; 21usize]; 21usize]; 11usize]; 11usize];
    let n_stages = 10usize;
    let max_score = 21usize;
    // state_a[4][8][0][0] = 1;
    state_a[7][3][0][0] = 1;
    let mut total_p1_wins = 0usize;
    let mut total_p2_wins = 0usize;

    for round in 0..n_stages {
        if round % 2 == 0 {
            let (p1_wins, p2_wins) = update_state(&state_a, &mut state_b, max_score);
            total_p1_wins += p1_wins;
            total_p2_wins += p2_wins;
        } else {
            let (p1_wins, p2_wins) = update_state(&state_b, &mut state_a, max_score);
            total_p1_wins += p1_wins;
            total_p2_wins += p2_wins;
        }
    }

    println!(
        "Part 2 result:\nP1 wins: {}\nP2 wins: {}",
        total_p1_wins, total_p2_wins
    );
}

fn day_21_dirac_dice() {
    part_1();
    part_2();
}

fn main() {
    day_21_dirac_dice();
}

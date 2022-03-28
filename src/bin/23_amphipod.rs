/// 2021 AoC Day 23: Amphipod
///
/// Find the most efficient way of moving amphipods around such that they each end up in their target room.
use std::collections::HashMap;

/// Corridor row
const COR_ROW: i32 = 0;
/// UR = upper-room
const UR_ROW: i32 = 1;
/// MR = mid-room
const MR_ROW: i32 = 2;
/// LR = low-room
const LR_ROW: i32 = 3;
/// BR = bottom-room
const BR_ROW: i32 = 4;

const PART_1_HEIGHT: usize = 3;
const PART_2_HEIGHT: usize = 5;

// Index of the last column in the corridor.
const END_COL: i32 = 10;

const AMBER_COL: i32 = 2;
const BRONZE_COL: i32 = 4;
const COPPER_COL: i32 = 6;
const DESERT_COL: i32 = 8;

const OK_STOP_COL: &[bool] = &[
    // 0, 1, 3, 5, 7, 9, 10 are OK to stop in after getting out
    true, true, false, true, false, true, false, true, false, true, true,
];

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
enum Kind {
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl Kind {
    fn move_cost(&self) -> i64 {
        match *self {
            Kind::Amber => 1,
            Kind::Bronze => 10,
            Kind::Copper => 100,
            Kind::Desert => 1000,
        }
    }

    fn initial(&self) -> &'static str {
        match *self {
            Kind::Amber => "A",
            Kind::Bronze => "B",
            Kind::Copper => "C",
            Kind::Desert => "D",
        }
    }

    fn target_col(&self) -> i32 {
        match *self {
            Kind::Amber => AMBER_COL,
            Kind::Bronze => BRONZE_COL,
            Kind::Copper => COPPER_COL,
            Kind::Desert => DESERT_COL,
        }
    }
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
enum State {
    Unmoved,
    InHallway,
    Done,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Amphipod {
    row: i32,
    col: i32,
    kind: Kind,
    state: State,
}

impl Amphipod {
    fn new(row: i32, col: i32, kind: Kind) -> Self {
        Amphipod {
            row: row,
            col: col,
            kind: kind,
            state: State::Unmoved,
        }
    }

    fn move_cost(&self) -> i64 {
        self.kind.move_cost()
    }

    fn initial(&self) -> &'static str {
        self.kind.initial()
    }

    fn target_col(&self) -> i32 {
        self.kind.target_col()
    }
}

#[derive(Debug, Clone)]
struct World {
    amphipods: Vec<Amphipod>,
    cost_so_far: i64,
    height: usize,
}

impl World {
    fn new(height: usize) -> Self {
        World {
            amphipods: Vec::new(),
            cost_so_far: 0,
            height: height,
        }
    }

    fn is_done(&self) -> bool {
        self.amphipods.iter().all(|a| a.state == State::Done)
    }

    fn is_solved(&self) -> bool {
        self.amphipods
            .iter()
            .all(|a| a.row > COR_ROW && a.col == a.target_col())
    }

    fn at(&self, row: i32, col: i32) -> &Amphipod {
        self.amphipods
            .iter()
            .find(|&a| a.row == row && a.col == col)
            .expect("invalid at(), no amphipod found!!")
    }

    fn with_move(
        &self,
        a_idx: usize,
        new_row: i32,
        new_col: i32,
        move_cost: i64,
        new_state: State,
    ) -> World {
        let mut cp = World::clone(self);
        cp.amphipods[a_idx].row = new_row;
        cp.amphipods[a_idx].col = new_col;
        cp.amphipods[a_idx].state = new_state;
        cp.cost_so_far += move_cost;
        cp
    }

    fn is_free(&self, row: i32, col: i32) -> bool {
        !self.amphipods.iter().any(|a| a.row == row && a.col == col)
    }

    /// Returns all valid non-end-goal moves for the unmoved amphipod at the given index, with associated cost.
    fn valid_moves(&self, a_idx: usize) -> Vec<(i32, i32, State, i64)> {
        let mut moves = Vec::new();
        let amphipod = &self.amphipods[a_idx];
        if amphipod.state != State::Unmoved {
            panic!("Attempted to move a hallway/finished amphipod!");
        }

        if amphipod.col == amphipod.target_col() {
            // We are on the right column
            if (amphipod.row as usize) == self.height - 1usize {
                // We are on the bottom row, with nobody to block. We're done.
                return vec![(amphipod.row, amphipod.col, State::Done, 0i64)];
            } else {
                // We are on a non-bottom row
                // If the amphipod(s) below us are all the same kind, we are done, otherwise, continue the algorithm
                // since we will need to try to move out.
                //
                // Note that this will panic if we're on the top row and the bottom row is blank, but that should never
                // happen by design.
                // if self.is_free(amphipod.row, LR_ROW) {
                //     println!("\n\n\n");
                //     print_world(&self);
                //     panic!("Invalid map");
                // }

                let mut homogeneous = true;
                for row in (amphipod.row + 1)..(self.height as i32) {
                    let neighbor = self.at(row, amphipod.col);
                    if neighbor.kind != amphipod.kind {
                        homogeneous = false;
                        break;
                    }
                }
                if homogeneous {
                    return vec![(amphipod.row, amphipod.col, State::Done, 0i64)];
                }
            }
        }

        if !self.is_free(amphipod.row - 1, amphipod.col) {
            // We are blocked in
            return moves;
        }

        // Not blocked in. Directly above is not an allowed move, but let's try above-and-left-ward moves...
        for col in (0..amphipod.col).rev() {
            if self.is_free(COR_ROW, col) {
                if OK_STOP_COL[col as usize] {
                    let n_hor_steps = (amphipod.col - col) as i64;
                    let n_ver_steps = amphipod.row as i64;
                    let cost_to_left = (n_hor_steps + n_ver_steps) * amphipod.move_cost();
                    moves.push((COR_ROW, col, State::InHallway, cost_to_left));
                }
            } else {
                break;
            }
        }
        // ...and right-ward moves
        for col in amphipod.col + 1..=END_COL {
            if self.is_free(COR_ROW, col) {
                if OK_STOP_COL[col as usize] {
                    let n_hor_steps = (col - amphipod.col) as i64;
                    let n_ver_steps = amphipod.row as i64;
                    let cost_to_right = (n_hor_steps + n_ver_steps) * amphipod.move_cost();
                    moves.push((COR_ROW, col, State::InHallway, cost_to_right));
                }
            } else {
                break;
            }
        }

        moves
    }

    /// Returns all valid end-goal moves for the in-hallway amphipod at the given index.
    fn valid_parking_spots(&self, a_idx: usize) -> Vec<(i32, i32, i64)> {
        let mut moves = Vec::new();
        let amphipod = &self.amphipods[a_idx];
        if amphipod.state != State::InHallway {
            panic!("Attempted to park an unmoved/finished amphipod!");
        }

        let target_col = amphipod.target_col();

        // Find the bottom-most slot in the target room.
        // If all occupied slots under _that_ one are the right kind, move in, otherwise do nothing.
        let mut target_row = -1i32;
        for r in 1i32..(self.height as i32) {
            if !self.is_free(r, target_col) {
                target_row = r - 1;
                break;
            }
        }
        if target_row == 0 {
            // Can't get into the target room, no action to do.
            return Vec::new();
        }
        if target_row == -1i32 {
            // All rows are free, set the bottom one as a target
            target_row = (self.height as i32) - 1;
        }

        let mut homogeneous = true;
        for r in target_row + 1i32..(self.height as i32) {
            if self.at(r, target_col).kind != amphipod.kind {
                homogeneous = false;
                break;
            }
        }

        if !homogeneous {
            return Vec::new();
        }

        // At this point, we've established the room is fine for us to go in, but now we need to check if we can
        // actually get to its entrance!
        let mut can_path = true;
        if target_col > amphipod.col {
            // check right move
            for col in amphipod.col + 1..=target_col {
                if !self.is_free(COR_ROW, col) {
                    can_path = false;
                    break;
                }
            }
        } else {
            // check left move
            for col in target_col..amphipod.col {
                if !self.is_free(COR_ROW, col) {
                    can_path = false;
                    break;
                }
            }
        }

        if can_path {
            let n_hor_steps = ((target_col - amphipod.col).abs()) as i64;
            let n_ver_steps = target_row as i64;
            let cost = (n_hor_steps + n_ver_steps) * amphipod.move_cost();
            moves.push((target_row, target_col, cost));
        }

        moves
    }

    /// Returns a vector of ALL possible worlds that can result from amphipod moves in the current world.
    fn moves(&self) -> Vec<World> {
        let mut new_worlds = Vec::new();

        for a_idx in 0..self.amphipods.len() {
            let amphipod = &self.amphipods[a_idx];
            match amphipod.state {
                State::Done => {}
                State::Unmoved => {
                    for (mv_row, mv_col, mv_state, mv_cost) in self.valid_moves(a_idx) {
                        let new_world = self.with_move(a_idx, mv_row, mv_col, mv_cost, mv_state);
                        new_worlds.push(new_world);
                    }
                }
                State::InHallway => {
                    for (mv_row, mv_col, mv_cost) in self.valid_parking_spots(a_idx) {
                        let new_world = self.with_move(a_idx, mv_row, mv_col, mv_cost, State::Done);
                        new_worlds.push(new_world);
                    }
                }
            }
        }

        new_worlds
    }
}

fn print_world(world: &World) {
    let mut out_arr = vec![
        String::from("#############"),
        String::from("#...........#"),
        String::from("### # # # ###"),
        String::from("  # # # # #  "),
        String::from("  #########  "),
    ];
    for a in &world.amphipods {
        let c = (a.col + 1) as usize;
        out_arr[(a.row + 1) as usize].replace_range(c..c + 1, a.initial());
    }

    for row in out_arr {
        println!("{}", row);
    }
    println!("Cost: {}", world.cost_so_far);
}

fn part_1_sample_world() -> World {
    // Sample input:
    // "BCBD / ADCA"
    let mut world = World::new(PART_1_HEIGHT);
    world.amphipods.push(Amphipod::new(UR_ROW, 2, Kind::Bronze));
    world.amphipods.push(Amphipod::new(MR_ROW, 2, Kind::Amber));

    world.amphipods.push(Amphipod::new(UR_ROW, 4, Kind::Copper));
    world.amphipods.push(Amphipod::new(MR_ROW, 4, Kind::Desert));

    world.amphipods.push(Amphipod::new(UR_ROW, 6, Kind::Bronze));
    world.amphipods.push(Amphipod::new(MR_ROW, 6, Kind::Copper));

    world.amphipods.push(Amphipod::new(UR_ROW, 8, Kind::Desert));
    world.amphipods.push(Amphipod::new(MR_ROW, 8, Kind::Amber));

    world
}

fn part_1_contest_world() -> World {
    let mut world = World::new(PART_1_HEIGHT);

    world.amphipods.push(Amphipod::new(UR_ROW, 2, Kind::Desert));
    world.amphipods.push(Amphipod::new(MR_ROW, 2, Kind::Bronze));

    world.amphipods.push(Amphipod::new(UR_ROW, 4, Kind::Amber));
    world.amphipods.push(Amphipod::new(MR_ROW, 4, Kind::Copper));

    world.amphipods.push(Amphipod::new(UR_ROW, 6, Kind::Copper));
    world.amphipods.push(Amphipod::new(MR_ROW, 6, Kind::Bronze));

    world.amphipods.push(Amphipod::new(UR_ROW, 8, Kind::Desert));
    world.amphipods.push(Amphipod::new(MR_ROW, 8, Kind::Amber));

    world
}

fn solve(initial_world: &World) {
    print_world(&initial_world);

    // TODO(andrei): Compute max step count meaningfully
    let mut worlds = vec![initial_world.clone()];
    let mut min_cost: i64 = i64::MAX;
    for generation in 0..25 {
        println!("Generation {}, {} worlds", generation, worlds.len());
        let mut new_worlds = Vec::new();
        let mut amphi_to_cost: HashMap<Vec<Amphipod>, i64> = HashMap::new();

        for w in &worlds {
            if w.is_solved() {
                // println!("Solved world of cost c = {}", w.cost_so_far);
                if w.cost_so_far <= min_cost {
                    min_cost = w.cost_so_far;
                }
                continue;
            }
            // if generation == 1 {
            //     if w.is_free(UR_ROW, DESERT_COL) {
            //         print_world(&w);
            //     }
            //     // if !w.is_free(UR_ROW, DESERT_COL) && !w.is_free(LR_ROW, DESERT_COL) {
            //     //     if w.at(LR_ROW, DESERT_COL).kind != Kind::Amber {
            //     //         print_world(&w);
            //     //     }
            //     // }
            // }
            // if generation == 2 {
            //     // this WORKS
            //     if w.is_free(UR_ROW, DESERT_COL) && w.is_free(LR_ROW, DESERT_COL) {
            //         print_world(&w);
            //         if !w.is_free(COR_ROW, 10) {
            //             println!("{:?}", w.at(COR_ROW, 10));
            //         }
            //     }
            // }
            // if generation == 3 {
            //     // this WORKS
            //     if !w.is_free(LR_ROW, DESERT_COL) && w.at(LR_ROW, DESERT_COL).kind == Kind::Desert {
            //         print_world(&w);
            //     }
            // }
            // if generation == 5 {
            // skip gen 4 because we need to "free" the other D as well
            // if !w.is_free(LR_ROW, DESERT_COL)
            //     && w.at(LR_ROW, DESERT_COL).kind == Kind::Desert
            //     && !w.is_free(UR_ROW, DESERT_COL)
            // {
            //     print_world(&w);
            // }
            // }
            let moves = &mut w.moves();
            // if moves.len() == 0 {
            //     print_world(&w);
            // }
            new_worlds.append(moves);
        }

        for nw in &mut new_worlds {
            if amphi_to_cost.contains_key(&nw.amphipods) {
                if amphi_to_cost.get(&nw.amphipods).unwrap() > &nw.cost_so_far {
                    amphi_to_cost.insert(nw.amphipods.clone(), nw.cost_so_far);
                }
            } else {
                amphi_to_cost.insert(nw.amphipods.clone(), nw.cost_so_far);
            }
        }

        worlds.clear();
        for el in amphi_to_cost {
            worlds.push(World {
                height: initial_world.height,
                amphipods: el.0,
                cost_so_far: el.1,
            });
        }
    }

    println!("min cost ever = {}", min_cost);
}

fn part_1() {
    println!("Part 1:");
    println!("Sample:");
    solve(&part_1_sample_world());

    println!("Contest:");
    solve(&part_1_contest_world());
}

fn day_23_amphipods() {
    part_1();
}

fn main() {
    day_23_amphipods();
}

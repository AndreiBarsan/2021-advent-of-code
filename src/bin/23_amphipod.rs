/*

row 0 = corridor
row 1 & 2 = dens

col = 0,1,9,10 are spaces
cols 2, 4, 6, 8 are right above dens
cols 3, 5, and 7 are between dens

#############
#...........#
###D#A#C#D###
  #B#C#B#A#
  #########
*/

/*

D:
5 * 1000 (D goes to the bottom)
8 * 1000

C:
1 * 100
5 * 100

B:
6 * 10
5 * 10

A:
6 * 1
9 * 1

=> 13725 total is too low

*/

const COR_ROW: i32 = 0;
// UR = upper-room
const UR_ROW: i32 = 1;
// LR = lower-room
const LR_ROW: i32 = 2;

// Index of the last column in the corridor.
const END_COL: i32 = 10;

const AMBER_COL: i32 = 2;
const BRONZE_COL: i32 = 4;
const COPPER_COL: i32 = 6;
const DESERT_COL: i32 = 8;

// 0, 1, 3, 5, 7, 9, 10 are OK to stop in after getting out
const OK_STOP_COL: &[bool] = &[
    true, true, false, true, false, true, false, true, false, true, true,
];

// TODO(andrei): Refactor this enum so each enum value contains its initial letter, cost, and target column.
#[derive(Debug, PartialEq, Clone)]
enum Kind {
    Amber,
    Bronze,
    Copper,
    Desert,
}

#[derive(Debug, PartialEq, Clone)]
enum State {
    Unmoved,
    InHallway,
    Done,
}

fn get_cost(kind: &Kind) -> i64 {
    match kind {
        Kind::Amber => 1,
        Kind::Bronze => 10,
        Kind::Copper => 100,
        Kind::Desert => 1000,
    }
}

fn get_initial(kind: &Kind) -> &'static str {
    match kind {
        Kind::Amber => "A",
        Kind::Bronze => "B",
        Kind::Copper => "C",
        Kind::Desert => "D",
    }
}

fn get_target_col(kind: &Kind) -> i32 {
    match kind {
        Kind::Amber => AMBER_COL,
        Kind::Bronze => BRONZE_COL,
        Kind::Copper => COPPER_COL,
        Kind::Desert => DESERT_COL,
    }
}

#[derive(Debug, Clone)]
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
}

#[derive(Debug, Clone)]
struct World {
    amphipods: Vec<Amphipod>,
    cost_so_far: i64,
}

impl World {
    fn new() -> Self {
        World {
            amphipods: Vec::new(),
            cost_so_far: 0,
        }
    }

    fn is_done(&self) -> bool {
        for a in &self.amphipods {
            if a.state != State::Done {
                return false;
            }
        }
        true
    }

    fn is_solved(&self) -> bool {
        for a in &self.amphipods {
            let ok_row = a.row == UR_ROW || a.row == LR_ROW;
            let ok_col = a.col == get_target_col(&a.kind);

            if !ok_row || !ok_col {
                return false;
            }
        }
        true
    }

    fn at(&self, row: i32, col: i32) -> &Amphipod {
        for a in &self.amphipods {
            if a.row == row && a.col == col {
                return &a;
            }
        }
        panic!("invalid at(), no amphipod found!!");
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
        for a in &self.amphipods {
            if a.row == row && a.col == col {
                return false;
            }
        }
        true
    }

    /// Returns all valid non-end-goal moves for the unmoved amphipod at the given index, with associated cost.
    fn valid_moves(&self, a_idx: usize) -> Vec<(i32, i32, State, i64)> {
        let mut moves = Vec::new();
        let amphipod = &self.amphipods[a_idx];
        if amphipod.state != State::Unmoved {
            panic!("Attempted to move a hallway/finished amphipod!");
        }

        if amphipod.col == get_target_col(&amphipod.kind) {
            // We are on the right column
            if amphipod.row == LR_ROW {
                // We are on the bottom row, with nobody to block. We're done.
                return vec![(amphipod.row, amphipod.col, State::Done, 0i64)];
            } else {
                // We are on the top row
                // If the amphipod below us is the same kind, we are done, otherwise, continue the algorithm since we
                // will need to try to move out.
                //
                // Note that this will panic if we're on the top row and the bottom row is blank, but that should never
                // happen by design.
                // if self.is_free(amphipod.row, LR_ROW) {
                //     println!("\n\n\n");
                //     print_world(&self);
                //     panic!("Invalid map");
                // }

                let neighbor = self.at(LR_ROW, amphipod.col);
                if neighbor.kind == amphipod.kind {
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
                    let mut n_steps = (amphipod.col - col + 1) as i64;
                    if amphipod.row == LR_ROW {
                        n_steps += 1;
                    }
                    let cost_to_left = n_steps * get_cost(&amphipod.kind);
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
                    let mut n_steps = (col - amphipod.col + 1) as i64;
                    if amphipod.row == LR_ROW {
                        n_steps += 1;
                    }
                    let cost_to_right = n_steps * get_cost(&amphipod.kind);
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

        let target_col = get_target_col(&amphipod.kind);
        if !self.is_free(UR_ROW, target_col) {
            // Can't get into the target room, no action to do.
            // Note that we can't already be at target if we are in the hallway.
            return Vec::new();
        }

        // If both slots are free in the room, go to the bottom one, if you can.
        let mut target_bot = false;
        let mut can_path: bool = true;
        if self.is_free(LR_ROW, target_col) {
            target_bot = true;
            // Both slots are free, go to the bottom one, provided we can path there.
        } else if self.at(LR_ROW, target_col).kind != amphipod.kind {
            // Upper is free but lower is occupied by something that needs to get out, nothing to do
            can_path = false;
        } else {
            // Upper is free and lower is same type as me
        }

        if target_col > amphipod.col {
            // check right move
            for col in amphipod.col..=target_col {
                if !self.is_free(COR_ROW, col) {
                    can_path = false;
                    break;
                }
            }
        } else {
            // check left move
            for col in target_col..=amphipod.col {
                if !self.is_free(COR_ROW, col) {
                    can_path = false;
                    break;
                }
            }
        }

        if can_path {
            let n_vert_steps = if target_bot { 2 } else { 1 };
            let n_steps = ((target_col - amphipod.col).abs() + n_vert_steps) as i64;
            let cost = n_steps * get_cost(&amphipod.kind);
            let target_row = if target_bot { LR_ROW } else { UR_ROW };
            moves.push((target_row, target_col, cost));
        }

        moves
    }

    /// Returns a vector of ALL possible worlds that can result from amphipod moves in the current world.
    fn moves(&self) -> Vec<World> {
        let mut new_worlds = Vec::new();

        for a_idx in 0..self.amphipods.len() {
            let a = &self.amphipods[a_idx];
            match a.state {
                State::Done => {}
                State::Unmoved => {
                    // Generate a new world for every possible move
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
        out_arr[(a.row + 1) as usize].replace_range(c..c + 1, get_initial(&a.kind));
    }

    for row in out_arr {
        println!("{}", row);
    }
    println!("Cost: {}", world.cost_so_far);
}

fn day_23_amphipods() {
    let mut world = World::new();
    world.amphipods.push(Amphipod::new(UR_ROW, 2, Kind::Bronze));
    world.amphipods.push(Amphipod::new(LR_ROW, 2, Kind::Amber));

    world.amphipods.push(Amphipod::new(UR_ROW, 4, Kind::Copper));
    world.amphipods.push(Amphipod::new(LR_ROW, 4, Kind::Desert));

    world.amphipods.push(Amphipod::new(UR_ROW, 6, Kind::Bronze));
    world.amphipods.push(Amphipod::new(LR_ROW, 6, Kind::Copper));

    world.amphipods.push(Amphipod::new(UR_ROW, 8, Kind::Desert));
    world.amphipods.push(Amphipod::new(LR_ROW, 8, Kind::Amber));
    // Sample input:
    // "BCBD / ADCA"

    // Dummy case where the initial world is actually solved
    //
    // world.amphipods.push(Amphipod::new(UR_ROW, 2, Kind::Amber));
    // world.amphipods.push(Amphipod::new(LR_ROW, 2, Kind::Amber));

    // world.amphipods.push(Amphipod::new(UR_ROW, 4, Kind::Bronze));
    // world.amphipods.push(Amphipod::new(LR_ROW, 4, Kind::Bronze));

    // world.amphipods.push(Amphipod::new(UR_ROW, 6, Kind::Copper));
    // world.amphipods.push(Amphipod::new(LR_ROW, 6, Kind::Copper));

    // world.amphipods.push(Amphipod::new(UR_ROW, 8, Kind::Desert));
    // world.amphipods.push(Amphipod::new(LR_ROW, 8, Kind::Desert));

    // println!("{:?}", world.amphipods[3]);
    // println!("{:?}", get_initial(&world.amphipods[3].kind));
    // println!("{:?}", get_initial(&world.amphipods[4].kind));
    // println!("{:?}", get_initial(&world.amphipods[5].kind));

    print_world(&world);

    // TODO(andrei): Compute max step count meaningfully
    let mut worlds = vec![world];
    let mut min_cost: i64 = 100000000;
    for generation in 0..10 {
        println!("Generation {}, {} worlds", generation, worlds.len());
        let mut new_worlds = Vec::new();

        for (w_idx, w) in worlds.iter().enumerate() {
            if w.is_solved() {
                println!("Solved world of cost c = {}", w.cost_so_far);
                if w.cost_so_far <= min_cost {
                    min_cost = w.cost_so_far;
                }
            }
            if generation == 1 {
                if w.is_free(UR_ROW, DESERT_COL) {
                    print_world(&w);
                }
                // if !w.is_free(UR_ROW, DESERT_COL) && !w.is_free(LR_ROW, DESERT_COL) {
                //     if w.at(LR_ROW, DESERT_COL).kind != Kind::Amber {
                //         print_world(&w);
                //     }
                // }
            }
            // if generation == 2 {
            //     // this WORKS
            //     if w.is_free(UR_ROW, DESERT_COL) && w.is_free(LR_ROW, DESERT_COL) {
            //         print_world(&w);
            //     }
            // }
            let moves = &mut w.moves();
            // if moves.len() == 0 {
            //     print_world(&w);
            // }
            new_worlds.append(moves);
        }

        worlds = new_worlds;
    }

    println!("min cost ever = {}", min_cost);

    // let new_worlds = world.moves();
    // println!("{}", new_worlds.len());
    // for w in &new_worlds {
    //     print_world(w);

    //     // TODO(andrei): This is just extremely lazy coding...
    //     let new_new_worlds = w.moves();
    //     println!("=============================");
    //     println!("{}", new_new_worlds.len());
    //     for nw in new_new_worlds {
    //         print_world(&nw);
    //     }
    //     println!("\n\n");
    // }
}

fn main() {
    day_23_amphipods();
}

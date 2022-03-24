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

const END_COL: i32 = 10;

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

    fn with_move(&self, a_idx: usize, new_row: i32, new_col: i32) -> World {
        let mut cp = World::clone(self);
        cp.amphipods[a_idx].row = new_row;
        cp.amphipods[a_idx].col = new_col;
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

    /// Returns all valid non-end-goal moves for the unmoved amphipod at the given index.
    fn valid_moves(&self, a_idx: usize) -> Vec<(i32, i32)> {
        let mut moves = Vec::new();
        let amphipod = &self.amphipods[a_idx];
        if amphipod.state != State::Unmoved {
            panic!("Attempted to move a hallway/finished amphipod!");
        }

        if !self.is_free(amphipod.row - 1, amphipod.col) {
            // We are blocked in
            return Vec::new();
        }

        // Not blocked in. Directly above is not an allowed move, but let's try left-ward moves...
        for col in (0..amphipod.col).rev() {
            if self.is_free(COR_ROW, col) {
                moves.push((COR_ROW, col));
            } else {
                break;
            }
        }
        // ...and right-ward moves
        for col in amphipod.col + 1..=END_COL {
            if self.is_free(COR_ROW, col) {
                moves.push((COR_ROW, col));
            } else {
                break;
            }
        }

        moves
    }

    fn moves(&self) -> Vec<World> {
        // let's look at every amphipod and enumerate all possible ways it could move
        let mut new_worlds = Vec::new();

        for a_idx in 0..self.amphipods.len() {
            let a = &self.amphipods[a_idx];
            if a.state == State::Done {
                continue;
            }
            if a.state == State::Unmoved {
                // Generate a new world for every possible move
                for (mv_row, mv_col) in self.valid_moves(a_idx) {
                    let new_world = self.with_move(a_idx, mv_row, mv_col);
                    new_worlds.push(new_world);
                }
                // for each reachable cell
                // yield a move there
            } else if a.state == State::InHallway {
                // If my target is available and I can reach it, move there.
            }

            // if a.state == State::InHallway {

            // }
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

    // println!("{:?}", world.amphipods[3]);
    // println!("{:?}", get_initial(&world.amphipods[3].kind));
    // println!("{:?}", get_initial(&world.amphipods[4].kind));
    // println!("{:?}", get_initial(&world.amphipods[5].kind));

    print_world(&world);
}

fn main() {
    day_23_amphipods();
}

/// 2021 AoC Day 17: Trick Shot
///
/// Spam shots from your cannon to see which ones land in a target area.

// use std::cmp::{max, min};
use std::collections::HashSet;

struct World {
    target_x: (i64, i64),
    target_y: (i64, i64),
}

impl World {
    fn in_target(&self, x: i64, y: i64) -> bool {
        x >= self.target_x.0 && x <= self.target_x.1 && y >= self.target_y.0 && y <= self.target_y.1
    }

    fn beyond_target(&self, x: i64, y: i64) -> bool {
        x > self.target_x.1 || y < self.target_y.0
    }
}


/// Launches from (0, 0) with (vx, vy) initial velocity, returning a (success, max_y) tuple.
fn shoot(vx: i64, vy: i64, world: &World, max_iter: usize) -> (bool, i64) {
    let mut cur_x: i64 = 0;
    let mut cur_y: i64 = 0;
    let mut cur_vx: i64 = vx;
    let mut cur_vy: i64 = vy;
    let mut max_y: i64 = i64::MIN;

    for _ in 0..max_iter {
        cur_x += cur_vx;
        cur_y += cur_vy;
        if cur_vx > 0 {
            cur_vx -= 1;
        }
        else if cur_vx < 0 {
            cur_vx += 1;
        }
        cur_vy -= 1;

        if cur_y > max_y {
            max_y = cur_y;
        }

        if world.in_target(cur_x, cur_y) {
            return (true, max_y);
        }
        else if world.beyond_target(cur_x, cur_y) {
            return (false, max_y);
        }
    }

    panic!("Could not reach our pass world after {} iterations...", max_iter);
}


fn day_17_trick_shot() {
    // demo
    // let world = World { target_x: (20, 30), target_y: (-10, -5) };
    // challenge
    let world = World { target_x: (153, 199), target_y: (-114, -75) };

    let mut good_inits = HashSet::new();

    let mut max_y = i64::MIN;
    for vx in 0..=world.target_x.1 {
        for vy in world.target_y.0..10000 {
            let (success, traj_max_y) = shoot(vx, vy, &world, 100000);
            if success && traj_max_y > max_y {
                max_y = traj_max_y;
            }
            if success {
                good_inits.insert((vx, vy));
            }
        }
    }

    println!("Part 1: {}", max_y);
    // First try, 3186, worked for Part 2's answer. Took a few s to compute in '--release' mode. Looks like we didn't
    // need any DP after all.
    println!("Part 2: {}", good_inits.len());
}


fn main() {
    day_17_trick_shot();
}

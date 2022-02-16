/// 2021 AoC Day 19: Beacon Scanner
///
/// While there are probably more efficient ways of solving this problem, I decided to solve it using a geometric
/// computer vision approach for fun.
///
/// On the flip side, I learned several new things about Rust:
///  - operator overloading
///  - the basics of nalgebra
extern crate nalgebra as na;

use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::ops;
use std::str::FromStr;

use na::geometry::{IsometryMatrix3, Rotation3, Translation3};
use na::Point3;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Point3d {
    x: i64,
    y: i64,
    z: i64,
}

impl Point3d {
    fn dist(&self, other: &Self) -> f64 {
        (*other - *self).norm()
    }

    fn norm_squared(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z) as f64
    }

    fn norm(&self) -> f64 {
        self.norm_squared().sqrt()
    }

    fn rotated(&self, rmat: &Rotation3<f64>) -> Point3d {
        let self_na = Point3::new(self.x as f64, self.y as f64, self.z as f64);

        let res = rmat * self_na;

        Point3d {
            x: res[0].round() as i64,
            y: res[1].round() as i64,
            z: res[2].round() as i64,
        }
    }
}

impl ops::Sub<Point3d> for Point3d {
    type Output = Point3d;

    fn sub(self, _rhs: Point3d) -> Point3d {
        Point3d {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
        }
    }
}

type EulerPose = ((f64, f64, f64), Point3d);

#[derive(Debug)]
enum Spec {
    NewScanner(i64),
    NewBeacon(Point3d),
}

#[derive(Debug)]
struct Triangle3d {
    a: Point3d,
    b: Point3d,
    c: Point3d,
}

impl Triangle3d {
    fn area(&self) -> f64 {
        // Heron's formula, since we're too lazy to find the height lol. I had not used this since literally middle
        // school lmao.
        let ab = self.a.dist(&self.b);
        let bc = self.b.dist(&self.c);
        let ac = self.a.dist(&self.c);

        let sp = 0.5 * (ab + bc + ac);

        (sp * (sp - ab) * (sp - ac) * (sp - bc)).sqrt()
    }

    fn ab(&self) -> Point3d {
        Point3d {
            x: self.b.x - self.a.x,
            y: self.b.y - self.a.y,
            z: self.b.z - self.a.z,
        }
    }

    fn ac(&self) -> Point3d {
        Point3d {
            x: self.c.x - self.a.x,
            y: self.c.y - self.a.y,
            z: self.c.z - self.a.z,
        }
    }

    fn bc(&self) -> Point3d {
        Point3d {
            x: self.c.x - self.b.x,
            y: self.c.y - self.b.y,
            z: self.c.z - self.b.z,
        }
    }

    fn a_angle_rad(&self) -> f64 {
        let ab_sq = self.ab().norm_squared();
        let ac_sq = self.ac().norm_squared();
        let ab = self.ab().norm();
        let ac = self.ac().norm();
        let bc_sq = self.bc().norm_squared();
        ((ab_sq + ac_sq - bc_sq) / (2f64 * ab * ac)).acos()
    }

    fn b_angle_rad(&self) -> f64 {
        let ab_sq = self.ab().norm_squared();
        let ac_sq = self.ac().norm_squared();
        let ab = self.ab().norm();
        let bc = self.bc().norm();
        let bc_sq = self.bc().norm_squared();
        ((ab_sq + bc_sq - ac_sq) / (2f64 * ab * bc)).acos()
    }

    fn c_angle_rad(&self) -> f64 {
        let ab_sq = self.ab().norm_squared();
        let ac_sq = self.ac().norm_squared();
        let ac = self.ac().norm();
        let bc = self.bc().norm();
        let bc_sq = self.bc().norm_squared();
        ((ac_sq + bc_sq - ab_sq) / (2f64 * ac * bc)).acos()
    }

    fn rotated(&self, rmat: &Rotation3<f64>) -> Triangle3d {
        let a_rot = self.a.rotated(rmat);
        let b_rot = self.b.rotated(rmat);
        let c_rot = self.c.rotated(rmat);

        Triangle3d {
            a: a_rot,
            b: b_rot,
            c: c_rot,
        }
    }

    // Check for congruency, assuming a/b/c follow consistent convention.
    fn congruent(&self, other: &Triangle3d) -> bool {
        self.ab() == other.ab() && self.ac() == other.ac() && self.bc() == other.bc()
    }
}

fn parse_beacon(line_str: &str) -> Spec {
    let parts: Vec<&str> = line_str.split(',').collect();
    Spec::NewBeacon(Point3d {
        x: i64::from_str(parts[0]).unwrap(),
        y: i64::from_str(parts[1]).unwrap(),
        z: i64::from_str(parts[2]).unwrap(),
    })
}

fn str_to_coords_or_scanner(line_str: &str) -> Spec {
    lazy_static! {
        static ref SCANNER_START_RE: Regex = Regex::new(r"---\s+scanner\s+(\d+)\s+---").unwrap();
    }

    match SCANNER_START_RE.captures(line_str) {
        Some(captures) => Spec::NewScanner(i64::from_str(&captures[1]).unwrap()),
        None => parse_beacon(line_str),
    }
}

#[derive(Debug, Copy, Clone)]
struct Candidate {
    cost: f64,
    point: Point3d,
}

impl PartialEq for Candidate {
    fn eq(&self, other: &Self) -> bool {
        (self.cost - other.cost).abs() < 1e-9
    }
}

impl Eq for Candidate {}

impl Ord for Candidate {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.cost > other.cost {
            Ordering::Greater
        } else if self.cost < other.cost {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for Candidate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Extracts a dynamic number k_i of keypoints and float fingerprints from the list points.
fn extract_keypoint_features(readings: &[Point3d]) -> Vec<(Triangle3d, f64)> {
    // TODO(andrei): If necessary, use a KD-tree here.
    // NOTE(andrei): There seem to be 28 scanners, each with ~20 points. I could potentially even compute ALL triangle
    // areas if I wanted to. For such small point clouds, it may actually end up slower if I use a KD-tree.

    let mut results = Vec::new();
    let max_dist = 1500f64;
    let k = 5;

    for p_idx in 0..(readings.len() - 1) {
        let mut neighbors: BinaryHeap<Candidate> = BinaryHeap::new();
        let mut knn = Vec::new();
        let p = readings[p_idx];

        for q in readings.iter().skip(p_idx + 1) {
            // if p_idx == q_idx {
            //     continue;
            // }

            neighbors.push(Candidate {
                cost: -1f64 * p.dist(q),
                point: *q,
            });
        }

        // XXX(andrei): Check that the neighbors are getting popped in the right order...
        while !neighbors.is_empty() {
            let n = neighbors.pop().unwrap();
            let cost = -1f64 * n.cost;
            if cost > max_dist {
                break;
            }
            knn.push(n.point);
            if knn.len() >= k {
                break;
            }
        }

        let actual_k = knn.len();

        if actual_k > 0 {
            for i in 0..(actual_k - 1) {
                for j in (i + 1)..actual_k {
                    let tri_tmp = Triangle3d {
                        a: p,
                        b: knn[i],
                        c: knn[j],
                    };

                    // Skip isosceles triangles as they could be ambiguous when matching
                    if (tri_tmp.ab() - tri_tmp.ac()).norm() < 1e-5
                        || (tri_tmp.ab() - tri_tmp.bc()).norm() < 1e-5
                        || (tri_tmp.ac() - tri_tmp.ab()).norm() < 1e-5
                    {
                        continue;
                    }
                    // Debug code
                    // if (tri_tmp.a_angle_rad() + tri_tmp.b_angle_rad() + tri_tmp.c_angle_rad() - 3.1415926535).abs() > 1e-5 {
                    //     panic!("Incorrect angles in triangle!");
                    // }

                    // Name points consistently using the largest angle as a hint
                    let tri = {
                        if tri_tmp.a_angle_rad() > tri_tmp.b_angle_rad()
                            && tri_tmp.a_angle_rad() > tri_tmp.c_angle_rad()
                        {
                            Triangle3d {
                                a: p,
                                b: knn[i],
                                c: knn[j],
                            }
                        } else if tri_tmp.b_angle_rad() > tri_tmp.a_angle_rad()
                            && tri_tmp.b_angle_rad() > tri_tmp.c_angle_rad()
                        {
                            Triangle3d {
                                a: knn[i],
                                b: p,
                                c: knn[j],
                            }
                        } else {
                            if !(tri_tmp.c_angle_rad() > tri_tmp.a_angle_rad()
                                && tri_tmp.c_angle_rad() > tri_tmp.b_angle_rad())
                            {
                                panic!("Inconsistent angles. Math likely incorrect.");
                            }
                            Triangle3d {
                                a: knn[i],
                                b: knn[j],
                                c: p,
                            }
                        }
                    };

                    let area = tri.area();
                    results.push((tri, area));
                }
            }
        }
    }

    results
}

/// Returns the (roll, pitch, yaw) that rotate triangle B to match triangle A.
fn match_rotation_naive(tri_a: &Triangle3d, tri_b: &Triangle3d) -> Option<(f64, f64, f64)> {
    // TODO(andrei): Clean up the code. Cache rotations.
    let pi = std::f64::consts::PI;
    for roll in &[0f64, -pi / 2f64, pi / 2f64, pi] {
        for pitch in &[0f64, -pi / 2f64, pi / 2f64, pi] {
            for yaw in &[0f64, -pi / 2f64, pi / 2f64, pi] {
                // TODO(andrei): Watch out for gimbal lock.
                let rot = Rotation3::from_euler_angles(*roll, *pitch, *yaw);
                // println!("{:?}", rot);

                let r_tri_b = tri_b.rotated(&rot);
                if tri_a.congruent(&r_tri_b) {
                    // If we don't break, we will definitely find a few more good rotations due to Gimbal lock - we are
                    // looping an over-parametrized space because Andrei is lazy.
                    // println!("{} {} {} = good rot", roll, pitch, yaw);
                    return Some((*roll, *pitch, *yaw));
                }
            }
        }
    }
    None
}

/// Returns the transform from B's to A's frame.
fn match_translation_naive(
    tri_a: &Triangle3d,
    tri_b: &Triangle3d,
    initial_rpy: (f64, f64, f64),
) -> Option<Point3d> {
    let rot = Rotation3::from_euler_angles(initial_rpy.0, initial_rpy.1, initial_rpy.2);
    let r_tri_b = tri_b.rotated(&rot);

    // let a_off = r_tri_b.a - tri_a.a;
    // let b_off = r_tri_b.b - tri_a.b;
    // let c_off = r_tri_b.c - tri_a.c;
    let a_off = tri_a.a - r_tri_b.a;
    let b_off = tri_a.b - r_tri_b.b;
    let c_off = tri_a.c - r_tri_b.c;

    if (a_off - b_off).norm() > 1e-5 || (a_off - c_off).norm() > 1e-5 {
        panic!("Inconsistent translation estimation. Math bug likely.");
    }

    Some(a_off)
}

/// Finds the SE(3) relative transform between two triangles, searching over fixed rotation candidates.
///
/// Assumes rotations are multiples of pi/2 and triangles are not equilateral or isosceles.
fn match_triangles_naive(
    tri_a: &Triangle3d,
    tri_b: &Triangle3d,
) -> Option<((f64, f64, f64), Point3d)> {
    let rot = match_rotation_naive(tri_a, tri_b);

    // TODO(andrei): Clean up this ugly "functional" code.
    let maybe_trans = rot
        .map(|rpy| match_translation_naive(tri_a, tri_b, rpy))
        .flatten();
    maybe_trans.map(|trans| (rot.unwrap(), trans))
}

fn match_features_and_solve_poses(
    scanner_kp_feats: &HashMap<i64, Vec<(Triangle3d, f64)>>,
    n_scanners: i64,
) -> (HashMap<(i64, i64), EulerPose>, Vec<Vec<i64>>) {
    // Brute-force matching since the number of scanners is <30 and each will have something like 5 triangles.
    let mut pose_graph: HashMap<(i64, i64), EulerPose> = HashMap::new();
    let mut adj: Vec<Vec<i64>> = vec![vec![0; n_scanners as usize]; n_scanners as usize];

    // Keep track of the pose from scanner K to 0.
    let mut scanner_to_0: HashMap<i64, ((f64, f64, f64), Point3d)> = HashMap::new();
    // Identity transform from 0 to itself.
    scanner_to_0.insert(0, ((0f64, 0f64, 0f64), Point3d { x: 0, y: 0, z: 0 }));

    // TODO(andrei): Rewrite this functionally.
    for scan_a in 0..n_scanners {
        for scan_b in (scan_a + 1)..n_scanners {
            println!("\n\n{} --> {}", scan_a, scan_b);

            let mut found_tform = false;

            for (tri_a, fingerprint_a) in scanner_kp_feats.get(&scan_a).unwrap() {
                if found_tform {
                    break;
                }
                for (tri_b, fingerprint_b) in scanner_kp_feats.get(&scan_b).unwrap() {
                    if (fingerprint_a - fingerprint_b).abs() < 1e-1 {
                        // println!("Scan {} matches {} @ \n\t{:?}\n\t{:?}", scan_a, scan_b, tri_a, tri_b);

                        let maybe_tform = match_triangles_naive(tri_a, tri_b);
                        if let Some((rpy, trans)) = maybe_tform {
                            println!("{:?}, {:?}", rpy, trans);
                            pose_graph.insert((scan_a, scan_b), (rpy, trans));
                            adj[scan_a as usize][scan_b as usize] = 1;
                            adj[scan_b as usize][scan_a as usize] = 1;
                            found_tform = true;
                            break;
                        }
                    }
                }
            }

            if found_tform {
                // ...
            } else {
                println!("ooh wee, could not find a transform...")
            }
        }
    }

    println!("{:?}", pose_graph.keys());

    (pose_graph, adj)
}

fn transform_point(p: &Point3d, transform: &IsometryMatrix3<f64>) -> Point3d {
    let p = Point3::new(p.x as f64, p.y as f64, p.z as f64);
    let tp = transform * p;
    Point3d {
        x: tp[0].round() as i64,
        y: tp[1].round() as i64,
        z: tp[2].round() as i64,
    }
}

fn transform_points(input: &[Point3d], transform: &IsometryMatrix3<f64>) -> Vec<Point3d> {
    input
        .iter()
        .map(|p3d| transform_point(p3d, transform))
        .collect()
}

fn align_all_points(
    scanners: &HashMap<i64, Vec<Point3d>>,
    pose_graph: &HashMap<(i64, i64), EulerPose>,
    adj: &[Vec<i64>],
    n_scanners: i64,
) {
    // TODO(andrei): Point transform chains are very inefficient. Instead, you want to pre-combine the transform chains
    //               into one, and THEN transform the point cloud.

    let mut all_points: Vec<Point3d> = Vec::new();

    for scanner in 0..n_scanners {
        let path_to_zero = bfs(vec![scanner as usize], adj, n_scanners as usize);
        println!("{:?}", path_to_zero);

        let mut current_pts = scanners[&scanner].clone();

        match path_to_zero {
            Some(path) => {
                for path_idx in path.windows(2) {
                    println!("{:?}", path_idx);
                    let pose_raw = pose_graph[&(path_idx[1] as i64, path_idx[0] as i64)];
                    let tra = Translation3::new(
                        pose_raw.1.x as f64,
                        pose_raw.1.y as f64,
                        pose_raw.1.z as f64,
                    );
                    // let tra = Translation3::new((pose_raw.1.x as f64) * -1.0f64, (pose_raw.1.y as f64) * -1.0f64, (pose_raw.1.z as f64) * -1.0f64);

                    let (roll, pitch, yaw) = pose_raw.0;
                    let rot_mat = Rotation3::from_euler_angles(roll, pitch, yaw);
                    let iso = IsometryMatrix3::from_parts(tra, rot_mat);

                    // if path_idx[0] == 3 {
                    //     println!("HMM: {:?}", pose_raw);
                    //     for xx in &current_pts {
                    //         println!("{:?}", xx);
                    //     }
                    // }
                    // println!("Doing transform...");
                    // println!("{:?}", iso);

                    current_pts = transform_points(&current_pts, &iso);
                    // if path_idx[0] == 3 {
                    // }
                }
            }
            None => println!("Warning: no path found!"),
        }

        // if scanner == 1 {
        let mut overlaps = 0;
        // for zero_pt in &scanners[&0i64] {
        //     println!("{:?}", zero_pt);
        // }
        println!("====");
        for new_p in &current_pts {
            // println!("{:?}", new_p);
            for zero_pt in &scanners[&0i64] {
                if new_p.x == zero_pt.x && new_p.y == zero_pt.y && new_p.z == zero_pt.z {
                    println!("Overlap: {:?} vs {:?}", new_p, zero_pt);
                    overlaps += 1;
                }
            }
        }
        println!(
            "scanner {}, {}/{} overlaps w/ 0",
            scanner,
            overlaps,
            current_pts.len()
        );

        // println!("{:?}", current_pts);
        all_points.append(&mut current_pts);
    }

    all_points.sort();
    for p in &all_points {
        println!("{} {} {}", p.x, p.y, p.z);
    }
    println!("{} points", all_points.len());

    let mut all_pts_set: HashSet<Point3d> = HashSet::new();
    for pt in &all_points {
        all_pts_set.insert(*pt);
    }
    println!("{}", all_pts_set.len());
}

/// Breadth-first search using an adjacency matrix, used to find a valid pose graph path to the root node, '0'.
fn bfs(path: Vec<usize>, adj: &[Vec<i64>], n_scanners: usize) -> Option<Vec<usize>> {
    let cur = path[path.len() - 1];
    if cur == 0 {
        return Some(path);
    }

    for n in 0..n_scanners {
        if adj[n][cur] == 1 && !path.contains(&n) {
            let mut new_path: Vec<usize> = path.clone();
            new_path.push(n);

            if let Some(good_path) = bfs(new_path, adj, n_scanners) {
                return Some(good_path);
            }
        }
    }

    None
}

fn day_19_beacon_scanner() {
    let input_fname = "input/19-demo.txt";
    // let input_fname = "input/19.txt";
    let scanner_beacons: Vec<Spec> = fs::read_to_string(input_fname)
        .expect("Unable to read file.")
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|x| str_to_coords_or_scanner(x))
        .collect();

    // println!("{:?}", scanner_beacons);

    let mut scanners: HashMap<i64, Vec<Point3d>> = HashMap::new();
    let mut cur_scanner: i64 = 0;
    for cmd in scanner_beacons {
        match cmd {
            Spec::NewScanner(scanner_id) => {
                cur_scanner = scanner_id;
            }
            Spec::NewBeacon(point) => scanners
                .entry(cur_scanner)
                .or_insert_with(Vec::new)
                .push(point),
        }
    }

    let scanner_keypoint_features: HashMap<i64, Vec<(Triangle3d, f64)>> = scanners
        .iter()
        .map(|(k, v)| (*k, extract_keypoint_features(v)))
        .collect();

    // println!("Features for scanner #2: {:?}", scanner_keypoint_features[&2]);
    let (pose_graph, adj) =
        match_features_and_solve_poses(&scanner_keypoint_features, scanners.len() as i64);

    align_all_points(&scanners, &pose_graph, &adj, scanners.len() as i64);
}

fn main() {
    day_19_beacon_scanner();
}

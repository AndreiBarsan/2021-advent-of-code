/// 2021 AoC Day 19: Beacon Scanner
///
/// While there are probably more efficient ways of solving this problem, I decided to solve it using a geometric
/// computer vision approach for fun.
///
/// On the flip side, I learned several new things about Rust:
///  - operator overloading
///  - the basics of nalgebra
extern crate nalgebra as na;

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::ops;
use std::str::FromStr;
use std::time::Instant;

use na::geometry::{IsometryMatrix3, Rotation3, Translation3};
use na::{Point3, Vector3};

type AdjacencyMatrix = Vec<Vec<i64>>;

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
    if line_str.chars().nth(1).unwrap() == '-' {
        let parts: Vec<&str> = line_str.split(' ').collect();
        let id = i64::from_str(parts[2]).unwrap();

        Spec::NewScanner(id)
    } else {
        parse_beacon(line_str)
    }

    // This regex approach also works, but it several ms slower than manual parsing.
    // lazy_static! {
    //     static ref SCANNER_START_RE: Regex = Regex::new(r"---\s+scanner\s+(\d+)\s+---").unwrap();
    // }
    // match SCANNER_START_RE.captures(line_str) {
    //     Some(captures) => Spec::NewScanner(i64::from_str(&captures[1]).unwrap()),
    //     None => parse_beacon(line_str),
    // }
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

/// Extracts a dynamic number k_i of keypoints and float fingerprints from the points belonging to the i-th scanner.
fn extract_keypoint_features(
    readings: &[Point3d],
    max_dist: f64,
    max_neighbors: usize,
) -> Vec<(Triangle3d, f64)> {
    // TODO(andrei): If necessary, use a KD-tree here.
    // NOTE(andrei): There seem to be 28 scanners, each with ~20 points. I could potentially even compute ALL triangle
    // areas if I wanted to. For such small point clouds, it may actually end up slower if I use a KD-tree.
    let mut results = Vec::with_capacity(max_neighbors * max_neighbors);
    let k = max_neighbors;

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

                    // This trick ensures we name points consistently using the largest angle as a hint.
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
                            // if !(tri_tmp.c_angle_rad() > tri_tmp.a_angle_rad()
                            //     && tri_tmp.c_angle_rad() > tri_tmp.b_angle_rad())
                            // {
                            //     panic!("Inconsistent angles. Math likely incorrect.");
                            // }
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

/// Returns the (roll, pitch, yaw) that rotates triangle B to match triangle A.
fn match_rotation_naive(tri_a: &Triangle3d, tri_b: &Triangle3d) -> Option<Rotation3<f64>> {
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
                    // If we don't return immediately, we will definitely find a few more good rotations due to Gimbal
                    // lock - we are looping an over-parametrized space because Andrei is lazy. Even axis-angle should
                    // be able to solve this.
                    // println!("{} {} {} = good rot", roll, pitch, yaw);
                    return Some(rot);
                }
            }
        }
    }
    None
}

/// Returns the translation from B's to A's frame, given a known rotation.
fn match_translation_naive(
    tri_a: &Triangle3d,
    tri_b: &Triangle3d,
    initial_rot: &Rotation3<f64>,
) -> Option<Translation3<f64>> {
    let r_tri_b = tri_b.rotated(initial_rot);

    let a_off = tri_a.a - r_tri_b.a;

    // let b_off = tri_a.b - r_tri_b.b;
    // let c_off = tri_a.c - r_tri_b.c;
    // if (a_off - b_off).norm() > 1e-5 || (a_off - c_off).norm() > 1e-5 {
    //     panic!("Inconsistent translation estimation. Math bug likely.");
    // }

    Some(Translation3::new(
        a_off.x as f64,
        a_off.y as f64,
        a_off.z as f64,
    ))
}

/// Finds the SE(3) relative transform from triangle A to triangle B, searching over fixed rotation candidates.
///
/// Assumes rotations are multiples of pi/2 and triangles are not equilateral or isosceles.
fn match_triangles_naive(tri_a: &Triangle3d, tri_b: &Triangle3d) -> Option<IsometryMatrix3<f64>> {
    let rot = match_rotation_naive(tri_a, tri_b);

    // TODO(andrei): Clean up this ugly "functional" code.
    let maybe_trans = rot
        .map(|rotation| match_translation_naive(tri_a, tri_b, &rotation))
        .flatten();
    maybe_trans.map(|trans| IsometryMatrix3::from_parts(trans, rot.unwrap()))
}

fn match_features_and_solve_poses(
    scanner_kp_feats: &HashMap<i64, Vec<(Triangle3d, f64)>>,
    n_scanners: i64,
) -> (HashMap<(i64, i64), IsometryMatrix3<f64>>, AdjacencyMatrix) {
    // Brute-force matching since the number of scanners is <30 and each will have something like 5 triangles.
    let mut pose_graph: HashMap<(i64, i64), IsometryMatrix3<f64>> = HashMap::new();
    let mut adj: AdjacencyMatrix = vec![vec![0; n_scanners as usize]; n_scanners as usize];

    // Keep track of the pose from scanner K to 0.
    let mut scanner_to_0: HashMap<i64, IsometryMatrix3<f64>> = HashMap::new();
    scanner_to_0.insert(0, IsometryMatrix3::identity());

    // TODO(andrei): Rewrite this functionally.
    for scan_a in 0..n_scanners {
        for scan_b in (scan_a + 1)..n_scanners {
            // println!("\n\n{} --> {}", scan_a, scan_b);

            let mut found_tform = false;

            for (tri_a, fingerprint_a) in scanner_kp_feats.get(&scan_a).unwrap() {
                if found_tform {
                    break;
                }
                for (tri_b, fingerprint_b) in scanner_kp_feats.get(&scan_b).unwrap() {
                    if (fingerprint_a - fingerprint_b).abs() < 1e-1 {
                        // println!("Scan {} matches {} @ \n\t{:?}\n\t{:?}", scan_a, scan_b, tri_a, tri_b);

                        let maybe_tform = match_triangles_naive(tri_a, tri_b);
                        if let Some(isometry) = maybe_tform {
                            // println!("{:?}, {:?}", rpy, trans);
                            pose_graph.insert((scan_a, scan_b), isometry);
                            adj[scan_a as usize][scan_b as usize] = 1;
                            adj[scan_b as usize][scan_a as usize] = 1;
                            found_tform = true;
                            break;
                        }
                    }
                }
            }
        }
    }

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

// fn invert_pose_hacky(input: &EulerPose) -> EulerPose {
//     let tra = Translation3::new(input.1.x as f64, input.1.y as f64, input.1.z as f64);
//     let (roll, pitch, yaw) = input.0;
//     let rot_mat = Rotation3::from_euler_angles(roll, pitch, yaw);
//     let iso = IsometryMatrix3::from_parts(tra, rot_mat);
//     let iso_inv = iso.inverse();
//     let rot_inv = iso_inv.rotation;
//     let trans_inv = iso_inv.translation;
//     let (i_roll, i_pitch, i_yaw) = rot_inv.euler_angles();

//     (
//         (i_roll, i_pitch, i_yaw),
//         Point3d {
//             x: trans_inv.vector.x.round() as i64,
//             y: trans_inv.vector.y.round() as i64,
//             z: trans_inv.vector.z.round() as i64,
//         },
//     )
// }

fn compute_absolute_poses(
    pose_graph: &HashMap<(i64, i64), IsometryMatrix3<f64>>,
    adjacency: &[Vec<i64>],
    n_scanners: i64,
) -> HashMap<i64, IsometryMatrix3<f64>> {
    let mut relative_poses: HashMap<i64, IsometryMatrix3<f64>> = HashMap::new();

    for scanner in 0..n_scanners {
        // Gather the relative pose from some scanner to scanner 0.
        let path_to_zero = bfs(vec![scanner as usize], adjacency, n_scanners as usize);
        // println!("{:?}", path_to_zero);

        let mut relative_pose = IsometryMatrix3::identity();

        match path_to_zero {
            Some(path) => {
                for path_idx in path.windows(2) {
                    let edge_key = (path_idx[1] as i64, path_idx[0] as i64);
                    let rev_edge_key = (path_idx[0] as i64, path_idx[1] as i64);
                    let edge_pose = if pose_graph.contains_key(&edge_key) {
                        pose_graph[&edge_key]
                    } else {
                        pose_graph[&rev_edge_key].inverse()
                    };

                    relative_pose = edge_pose * relative_pose;
                }
            }
            None => panic!("Warning: no path found between scanner {} and 0!", scanner),
        }
        relative_poses.insert(scanner, relative_pose);
    }

    relative_poses
}

fn count_unique_points(
    scanners: &HashMap<i64, Vec<Point3d>>,
    absolute_poses: &HashMap<i64, IsometryMatrix3<f64>>,
) -> usize {
    let mut all_points: Vec<Point3d> = Vec::new();
    for (scanner, scanner_points) in scanners {
        let mut current_pts = transform_points(scanner_points, &absolute_poses[scanner]);
        all_points.append(&mut current_pts);
    }

    let mut all_pts_set: HashSet<Point3d> = HashSet::new();
    for pt in &all_points {
        all_pts_set.insert(*pt);
    }
    all_pts_set.len()
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

fn l1(vec: &Vector3<f64>) -> u64 {
    (vec.x.abs().round() as u64) + (vec.y.abs().round() as u64) + (vec.z.abs().round() as u64)
}

fn compute_largest_manhattan(absolute_poses: &HashMap<i64, IsometryMatrix3<f64>>) -> u64 {
    let mut max_l1: u64 = 0;
    for s1_pose in absolute_poses.values() {
        for s2_pose in absolute_poses.values() {
            let offset = s1_pose.translation.vector - s2_pose.translation.vector;
            let offset_l1 = l1(&offset);
            if offset_l1 > max_l1 {
                max_l1 = offset_l1;
            }
        }
    }

    max_l1
}

fn day_19_beacon_scanner() {
    // let input_fname = "input/19-demo.txt";
    let input_fname = "input/19.txt";
    let max_dist = 1500f64;
    let max_neighbors = 3usize;
    let scanner_beacons: Vec<Spec> = fs::read_to_string(input_fname)
        .expect("Unable to read file.")
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(|x| str_to_coords_or_scanner(x))
        .collect();

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

    let start = Instant::now();
    let scanner_keypoint_features: HashMap<i64, Vec<(Triangle3d, f64)>> = scanners
        .iter()
        .map(|(k, v)| (*k, extract_keypoint_features(v, max_dist, max_neighbors)))
        .collect();

    // println!("Features for scanner #2: {:?}", scanner_keypoint_features[&2]);
    let (pose_graph, adj) =
        match_features_and_solve_poses(&scanner_keypoint_features, scanners.len() as i64);

    let absolute_poses = compute_absolute_poses(&pose_graph, &adj, scanners.len() as i64);

    // Part 1
    //
    // Note: For Part 1, 490 is too high - which makes sense considering I got this number while not properly aligning
    //       several point clouds.
    //
    // In my case, 367 is correct for Part 1 - I just needed to process the pose graph properly.
    let n_unique = count_unique_points(&scanners, &absolute_poses);
    println!("{}", n_unique);

    // Part 2
    let largest_distance = compute_largest_manhattan(&absolute_poses);
    println!("Largest manhattan distance: {}", largest_distance);

    let solve_ms = start.elapsed().as_micros();
    println!(
        "Solver took {}µs. (Excluding input parsing and process initialization.)",
        solve_ms
    );
}

fn main() {
    day_19_beacon_scanner();
}

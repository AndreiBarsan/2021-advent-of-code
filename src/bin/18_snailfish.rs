use std::cell::Ref;
use std::cell::RefCell;
use std::fmt;
/// 2021 AoC Day 18: Snailfish
///
/// Very tricky for me as a Rust beginner, since we need to build, manage, and operate upon (mutate) a tree, which is
/// difficult to do with Rust's reference and borrowing semantics.
///
/// After solving it in Python and continuing to struggle with the Rust implementation, I glanced at some other people's
/// Rust solutions - some just operate on token sequences (simpler, IMO), others build trees using enum and Box. I'll
/// try the token approach next, even though I used trees in Python, since my initial attempt at Rc/RefCell trees did
/// not go well.
use std::fs;
use std::rc::Rc;

// #[derive(Debug, Eq, PartialEq)]
// enum SnailNum {
//     Val {
//         parent: Option<Box<SnailNum>>,
//         val: u32,
//     },
//     Pair {
//         parent: Option<Box<SnailNum>>,
//         left: Option<Box<SnailNum>>,
//         right: Option<Box<SnailNum>>,
//     },
// }

type NodeLink = Option<Rc<RefCell<SnailNode>>>;

// TODO(andrei): Learn to use this once you solve the problem, so your trees don't leak.
// type NodeWeakLink = Option<Weak<RefCell<SnailNode>>>;

#[derive(Debug)]
struct SnailNode {
    value: Option<u32>,
    parent: NodeLink,
    left: NodeLink,
    right: NodeLink,
}

impl fmt::Display for SnailNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.value {
            Some(val) => write!(f, "{}", val),
            // Bootleg comparison since we are not using an enum to differentiate between leaves and non-leaves.
            _ => match &self.left {
                Some(left_ref) => write!(
                    f,
                    "[{},{}]",
                    left_ref.borrow(),
                    self.right.as_ref().unwrap().borrow()
                ),
                None => Ok(()),
            },
        }
    }
}

impl SnailNode {
    fn new(val: Option<u32>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(SnailNode {
            value: val,
            parent: None,
            left: None,
            right: None,
        }))
    }

    /// Access left, assuming it is there
    fn bl(&self) -> &RefCell<SnailNode> {
        self.left.as_ref().unwrap().as_ref()
    }

    /// Access right, assuming it is there
    fn br(&self) -> &RefCell<SnailNode> {
        self.right.as_ref().unwrap().as_ref()
    }

    // Note to self - we need to annotate the lifetimes since the compiler must know (and enforce) the fact that all
    // tree node references have the same lifetime. This is normal, as all nodes of a tree should share the same
    // lifetime!
    // fn as_flat(
    //     &'a mut self,
    //     parent: Option<&'a mut Self>,
    //     base_depth: u32,
    // ) -> Vec<(u32, u32, Option<&'a mut Self>)> {
    //     match self {
    //         SnailNum::Val(nr) => vec![(base_depth, *nr, parent)],
    //         SnailNum::Pair(left, right) => {
    //             let some_self = Some(self);
    //             let mut res = left.as_flat(some_self, base_depth + 1);
    //             {
    //                 let some_other_self = Some(self);
    //                 let poop = right.as_flat(some_other_self, base_depth + 1);
    //                 res.extend(poop);
    //             }
    //             res
    //         }
    //     }
    // }
}

// I legitimately gave up on this after 8+ hours spent trying to get Rc/Ref to work on a simple double-linked tree...
//
// fn find_explode_candidate(num: Rc<RefCell<SnailNode>>, depth: usize, min_depth: usize) -> Rc<RefCell<SnailNode>> {
//     if depth >= min_depth {
//         return num;
//     }

//     if (&*num.borrow()).left.is_some() {
//         let something = num.clone();
//         let new_left = Ref::map(something.borrow(), |n| &n.left.as_ref().unwrap().borrow());
//         // let left_cand = find_explode_candidate((&*num.borrow()).left.unwrap(), depth + 1, min_depth);
//     }

//     //     if (left_cand.is_some()) {
//     //         return left_cand;
//     //     }
//     // }

//     num

//     // if (&*num.unwrap().borrow()).left.is_some() {
//     // }

fn from_flat(flat_data: Vec<(u32, u32)>) -> String {
    let mut cur_depth: u32 = 0;
    let mut out_chars: Vec<char> = Vec::new();

    for (depth, val) in flat_data {
        if depth > cur_depth {
            for _ in cur_depth..depth {
                out_chars.push('[');
            }
        } else if depth < cur_depth {
            for _ in depth..cur_depth {
                out_chars.push(']');
            }
        }
        cur_depth = depth;
        out_chars.push('0');
    }

    out_chars.iter().collect()
}

/// Used in very inefficient parsing
fn find_matching_bracket(data: &[char]) -> usize {
    let mut n_open: usize = 0;

    for (idx, ch) in data.iter().enumerate() {
        if ch == &'[' {
            n_open += 1
        } else if ch == &']' {
            if n_open == 0 {
                return idx;
            }

            n_open -= 1;
        }
    }

    panic!("Invalid syntax");
}

/// sv = snail val
// macro_rules! sv {
//     ($x:expr) => {
//         SnailNum::Val($x)
//     };
// }

/// Simply syntax sugar for building a snail number pair (of literals, other pairs, or combinations thereof).
// macro_rules! snail_pair {
//     ($x:expr,$y:expr) => {{
//         SnailNum::Pair(Box::new($x), Box::new($y))
//     }};
// }

fn parse_snail_pair_str(data: &str, p: NodeLink) -> NodeLink {
    parse_snail_pair(&data.replace(" ", "").chars().collect::<Vec<char>>(), p)
}

fn parse_snail_pair(data: &[char], p: NodeLink) -> NodeLink {
    let mut cur = SnailNode::new(None);
    cur.borrow_mut().parent = p;
    let (left, right_start) = if data[1] == '[' {
        // it's a nested pair
        let matching_idx = find_matching_bracket(&data[2..]) + 2;
        (
            parse_snail_pair(&data[1..(matching_idx + 1)], Some(Rc::clone(&cur))),
            matching_idx + 1 + 1,
        )
    } else {
        // it's just an integer
        let literal = (data[1] as u32) - ('0' as u32);
        // println!("Found literal: {}", literal);
        // The '3' comes from '[N,' - i.e., point to whatever is after the comma.
        (Some(SnailNode::new(Some(literal))), 3)
    };

    let right = if data[right_start] == '[' {
        let matching_idx = find_matching_bracket(&data[right_start..]);
        // println!("right - {:?}", &data[right_start..(right_start + matching_idx + 1)]);
        parse_snail_pair(
            &data[right_start..(right_start + matching_idx + 1)],
            Some(Rc::clone(&cur)),
        )
    } else {
        let literal = (data[right_start] as u32) - ('0' as u32);
        // println!("Found literal: {}", literal);
        Some(SnailNode::new(Some(literal)))
    };
    cur.borrow_mut().left = left;
    cur.borrow_mut().right = right;
    Some(cur)
}

// fn decimal(input: &str) -> IResult<&str, &str> {
//     recognize(
//       many1(
//         terminated(one_of("0123456789"), many0(char('_')))
//       )
//     )(input)
//   }

// fn snail_tup(input: &str) -> IResult<&str, (u32, u32)> {
//     let (input, _) = tag("[")(input)?;
//     let stuff = tuple((decimal, multispace0, char(','), multispace0, decimal))(input);

//     println!("{:?}", stuff);

//     Ok((input, (1, 2) ))
// }

// fn first_explode(root: &SnailNum) {
//     let flat = root.as_flat(None, 0u32);
//     let res = flat.iter().find(|tup| tup.1 > 4);
//     // match res {
//     //     None => (),
//     //     Some(el) => el.1
//     // }
// }

fn day_18_snailfish() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_parsing() {
        let res = parse_snail_pair_str(&"[3,4]", None);
        match &*res.unwrap().borrow() {
            SnailNode {
                value,
                parent,
                left,
                right,
            } => {
                assert_eq!(left.as_ref().unwrap().borrow().value.unwrap(), 3u32);
                assert_eq!(right.as_ref().unwrap().borrow().value.unwrap(), 4u32);
            }
            _ => panic!("Invalid tree"),
        }
    }

    #[test]
    fn test_basic_nested() {
        let res = parse_snail_pair_str(&"[3, [5, 7]]", None);
        let actual = (&*res.unwrap().borrow())
            .br()
            .borrow()
            .bl()
            .borrow()
            .value
            .unwrap();
        let expected = 5u32;
        assert_eq!(expected, actual);

        let res_left = parse_snail_pair_str(&"[[5, 7], 9]", None);
        let actual_left = (&*res_left.unwrap().borrow())
            .bl()
            .borrow()
            .br()
            .borrow()
            .value
            .unwrap();
        let expected_left = 7u32;
        assert_eq!(expected_left, actual_left);

        let res_deep = parse_snail_pair_str("[[[1,2],5],[6,7]]", None).unwrap();
        let actual_deep = (&*res_deep.borrow())
            .bl()
            .borrow()
            .bl()
            .borrow()
            .br()
            .borrow()
            .value
            .unwrap();
        let expected_deep = 2u32;
        assert_eq!(expected_deep, actual_deep);
    }

    /// Tests snail number parsing by verifying that the 'to_string()' of the parsed number equals the original.
    #[test]
    fn test_advanced_parse() {
        let samples: Vec<&str> = vec![
            "[[[[1,2],[3,4]],[[5,6],[7,8]]],9]",
            "[[[[1,2],[3,4]],[[5,6],[7,8]]],9]",
            "[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]",
            "[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]",
        ];
        for raw_str in samples {
            let parsed = parse_snail_pair_str(raw_str, None).unwrap();
            let string_again = (&*parsed.borrow()).to_string();
            assert_eq!(string_again, raw_str);
        }
    }

    // #[test]
    // fn test_find_first_explode() {
    //     // List of inputs and expected outputs.
    //     let samples: Vec<(&str, &str)> = vec![
    //         ("[[[[[9,8],1],2],3],4]", "[9, 8]"),
    //         ("[7,[6,[5,[4,[3,2]]]]]", "[3, 2]"),
    //         ("[7,[6,[5,[4,[3,2]]]]]", "[3, 2]"),
    //         ("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]", "[7, 3]"),
    //     ];
    //     for (raw_input, raw_expected_pair) in samples {
    //         // let raw_in_chars: Vec<char> = raw_input.to_string().chars().collect();
    //         // let exp_pair_ch: Vec<char> = expected_pair.to_string().chars().collect();
    //         let parsed_in = parse_snail_pair_str(&raw_input, None);
    //         let parsed_expected = parse_snail_pair_str(&raw_expected_pair, None);
    //         let explode_candidate = find_explode_candidate(parsed_in.unwrap(), 1, 4);

    //         // assert_eq!(parsed_expected, explode_candidate);
    //     }
    // }

    // #[test]
    // fn test_basic_reduction() {
    //     // List of inputs and expected outputs.
    //     let samples: Vec<(&str, &str)> = vec![
    //         ("[[[[1,2],[3,4]],[[5,6],[7,8]]],9]", "[[[[1,2],[3,4]],[[5,6],[7,8]]],9]"),
    //         ("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]"),
    //     ];
    //     for (raw_input, raw_output) in samples {
    //         let raw_in_chars: Vec<char> = raw_input.to_string().chars().collect();
    //         let raw_out_chars: Vec<char> = raw_output.to_string().chars().collect();
    //         let parsed_in = parse_snail_pair(&raw_in_chars[..]);
    //         let parsed_out = parse_snail_pair(&raw_out_chars[..]);
    //     }
    // }
}

fn main() {}

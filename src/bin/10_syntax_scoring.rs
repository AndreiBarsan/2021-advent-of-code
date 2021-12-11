use std::fs;
use std::collections::HashMap;


fn get_char_to_type() -> HashMap<char, (char, bool)> {
     HashMap::from([
        ('(', ('(', true)),
        (')', ('(', false)),
        ('[', ('[', true)),
        (']', ('[', false)),
        ('{', ('{', true)),
        ('}', ('{', false)),
        ('<', ('<', true)),
        ('>', ('<', false)),
    ])
}



/// Returns a vector of all illegal characters on the given line and the remaining char queue.
fn parse_line(line: &str) -> (Vec<usize>, Vec<char>) {
    let mut error_positions = Vec::new();
    let char_to_type = get_char_to_type();
    let mut char_queue = Vec::new();

    for (idx, ch) in line.chars().enumerate() {
        if char_queue.is_empty() {
            char_queue.push(ch);
        }
        else {
            let last = char_queue[char_queue.len() - 1];
            let (last_type, _)  = char_to_type[&last];
            let (cur_type, cur_open) = char_to_type[&ch];

            if cur_open {
                char_queue.push(ch);
            }
            else if cur_type == last_type {
                // Closed something valid
                char_queue.pop();
            }
            else {
                // Attempting to close something that doesn't match
                error_positions.push(idx);
            }
        }
    }

    (error_positions, char_queue)
}


fn day_10_syntax_scoring() {
    // let data = fs::read_to_string("input/10-demo.txt").expect("Unable to read file.");
    let data = fs::read_to_string("input/10.txt").expect("Unable to read file.");
    let mut syntax_error_score: u32 = 0;
    let mut auto_complete_scores: Vec<u64> = Vec::new();
    let char_to_type = get_char_to_type();

    let marker_to_cost: HashMap<char, u32> = HashMap::from([
        ('(', 3),
        ('[', 57),
        ('{', 1197),
        ('<', 25137),
    ]);
    let marker_to_fix_cost: HashMap<char, u64> = HashMap::from([
        ('(', 1),
        ('[', 2),
        ('{', 3),
        ('<', 4),
    ]);

    for line in data.split("\n") {
        // println!("Line: {}", line);
        let (error_positions, mut pending_queue) = parse_line(line);
        if error_positions.len() > 0 {
            // Line with syntax errors!
            let raw_char = &line.chars().nth(error_positions[0usize]).unwrap();
            let char_type = char_to_type[raw_char].0;
            syntax_error_score += marker_to_cost[&char_type];
        }
        else {
            // Valid line. We would like to find what the remainder of the line should be in order for it to be
            // syntactically valid, i.e., balanced. Turns out, this information is already envoded in our parser's
            // state, in the operator queue. All we need to do is read it out from right to left and output the
            // corresponding matching character. Since we're just scoring the solution, we just look up the value of
            // every character and sum it up according to the formula from the problem.
            pending_queue.reverse();
            let mut auto_complete_score: u64 = 0;
            for ch in pending_queue {
                let char_type = char_to_type[&ch].0;
                auto_complete_score = auto_complete_score * 5u64 + marker_to_fix_cost[&char_type];
            }

            auto_complete_scores.push(auto_complete_score);
        }
    }

    println!("Part 1 syntax error score:  {}", syntax_error_score);

    // Find the middle auto-complete score as the answer to Part 2.
    auto_complete_scores.sort();
    println!("Part 2 auto-complete score: {}", auto_complete_scores[auto_complete_scores.len() / 2usize]);
}

fn main() {
    day_10_syntax_scoring();
}
/// 2021 AoC Day 16: Packet Decoder
///
/// Decoding packets from bits and evaluating operator trees.

use std::fs;
use lazy_static::lazy_static;

#[derive(Debug, Eq, PartialEq)]
enum OperatorType {
    SUM,
    PROD,
    MIN,
    MAX,
    GT,
    LT,
    EQ,
}

const SUM_ID: usize = 0;
const PROD_ID: usize = 1;
const MIN_ID: usize = 2;
const MAX_ID: usize = 3;
const LITERAL_ID: usize = 4usize;
const GT_ID: usize = 5;
const LT_ID: usize = 6;
const EQ_ID: usize = 7;

#[derive(Debug, Eq, PartialEq)]
enum PacketContent {
    LITERAL(i64),
    OPERATOR(OperatorType, Vec<Packet>),
}


#[derive(Debug, Eq, PartialEq)]
struct Packet {
    version: usize,
    content: PacketContent,
}

/// Converts chars [0..9] and [A..F] into their corresponding bits.
fn hex_char_to_bits(ch: char) -> Vec<u8> {
    lazy_static! {
        static ref LOOKUP: Vec<Vec<u8>> = vec![
            vec![0,0,0,0],
            vec![0,0,0,1],
            vec![0,0,1,0],
            vec![0,0,1,1],
            vec![0,1,0,0],
            vec![0,1,0,1],
            vec![0,1,1,0],
            vec![0,1,1,1],
            vec![1,0,0,0],
            vec![1,0,0,1],
            vec![1,0,1,0],
            vec![1,0,1,1],
            vec![1,1,0,0],
            vec![1,1,0,1],
            vec![1,1,1,0],
            vec![1,1,1,1],
        ];
    }
    let ch_dig = if ch <= '9' && '0' <= ch {
        (ch as usize) - ('0' as usize)
    }
    else {
        (ch as usize) - ('A' as usize) + 10usize
    };

    LOOKUP[ch_dig].to_vec()
}


fn hex_str_to_bits(hex: &str) -> Vec<u8> {
    // Once I'm better with Rust I should use a BitSet.
    hex.chars().flat_map(hex_char_to_bits).collect()
}

fn bin_to_dec(stuff: &[u8]) -> i64 {
    let mut idx: i64 = (stuff.len() - 1) as i64;
    let mut acc: i64 = 0;
    let mut exp: i64 = 1;

    loop {
        if stuff[idx as usize] == 1 {
            acc += exp;
        }
        exp = exp * 2;
        idx -= 1;
        if idx < 0 {
            break
        }
    }

    acc
}

/// Parses nested packets up to 'n_bits_sp'
fn parse_sub_packets_n_total_bits(bits: &[u8], n_bits_sp: usize) -> Vec<Packet> {
    let mut cur_bit = 0usize;
    let mut packets = Vec::new();

    while cur_bit < n_bits_sp {
        let (packet, bits_consumed) = parse_packet_bits(&bits[cur_bit..]);
        cur_bit += bits_consumed;
        println!("Packet = {:?}, consumed = {} / {}", packet, cur_bit, n_bits_sp);
        packets.push(packet);
    }

    packets
}

/// Parses the next 'n_packets', assumed to be consecutively encoded in 'bits', returning the index after the last one.
fn parse_sub_packets_n_packets(bits: &[u8], n_packets: usize) -> (Vec<Packet>, usize) {
    let mut cur_bit = 0usize;
    let mut packets = Vec::new();
    for _ in 0..n_packets {
        let (packet, n_consumed) = parse_packet_bits(&bits[cur_bit..]);
        cur_bit += n_consumed;
        packets.push(packet);
    }

    (packets, cur_bit)
}

fn parse_packet_bits(bits: &[u8]) -> (Packet, usize) {
    let version_bits = &bits[0..3];
    let type_bits = &bits[3..6];

    let version = bin_to_dec(version_bits) as usize;
    let type_id = bin_to_dec(type_bits) as usize;
    // println!("type_id = {}, version = {}", type_id, version);

    if type_id == LITERAL_ID {
        // no-op for now, just parsing a literal

        let mut cur = 6usize;
        let mut nr_bits: Vec<u8> = Vec::new();
        loop {
            nr_bits.extend(&bits[(cur + 1)..(cur + 5)]);

            if bits[cur] == 0 {
                break;
            }
            cur += 5;
        }

        let lit_val = bin_to_dec(&nr_bits);
        (Packet { version: version, content: PacketContent::LITERAL(lit_val) }, cur + 5)
    }
    else {
        // parse nested packets as needed
        let mut children = Vec::new();
        let mut bit_type_id = bits[6];
        let mut packet_start = 0;
        let mut end = 0;
        if bit_type_id == 0 {
            println!("Reading kids by bit count...");
            packet_start = 7 + 15;
            let n_bits_bits = &bits[7..packet_start];
            let n_bits_sp = bin_to_dec(n_bits_bits) as usize;

            let mut result = parse_sub_packets_n_total_bits(&bits[packet_start..(packet_start+n_bits_sp)], n_bits_sp);
            children.append(&mut result);
            end = packet_start + n_bits_sp;
            println!("Read {} kid bits from {} to {}", n_bits_sp, packet_start, packet_start+n_bits_sp)
        }
        else {
            println!("Reading kids by kid count...");
            packet_start = 7 + 11;
            let n_sub_packets_bits = &bits[7..packet_start];
            let n_packets = bin_to_dec(n_sub_packets_bits) as usize;

            let (mut result, bits_consumed) = parse_sub_packets_n_packets(&bits[packet_start..], n_packets);
            children.append(&mut result);
            end = packet_start + bits_consumed;
        }

        // TODO(andrei): Can we actually model the enum to support this mapping directly?
        let op_type = match type_id {
            SUM_ID => OperatorType::SUM,
            PROD_ID => OperatorType::PROD,
            MIN_ID => OperatorType::MIN,
            MAX_ID => OperatorType::MAX,
            GT_ID => OperatorType::GT,
            LT_ID => OperatorType::LT,
            EQ_ID => OperatorType::EQ,
            _ => panic!("Invalid type ID {}", type_id),
        };

        (Packet{ version: version, content: PacketContent::OPERATOR(op_type, children) }, end)
    }
}


fn parse_packet(data: &String) -> (Packet, usize) {
    let packet_bits = hex_str_to_bits(data);
    let (packets, end_idx) = parse_packet_bits(&packet_bits);

    // Seems like end_idx can often be much smaller than the nr. of bits - garbage at the end?
    // However, it should never ever be bigger!
    // if packet_bits.len() as i64 - end_idx as i64 > 4i64 || end_idx > packet_bits.len() {
    if end_idx > packet_bits.len() {
        println!("{:?}", packet_bits);
        panic!("{} packet bits, {} final end idx", packet_bits.len(), end_idx);
    }

    (packets, end_idx)
}


// TODO(andrei): Move entity definitions and related tests to their own file.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_literal() {
        let raw_packet = "D2FE28".to_string();
        let (packet, bits_consumed) = parse_packet(&raw_packet);
        let expected_packet = Packet { version: 6usize, content: PacketContent::LITERAL(2021i64) };
        assert_eq!(packet, expected_packet);
    }

    #[test]
    fn test_eval_basic_literal() {
        let raw_packet = "D2FE28".to_string();
        let (packet, _) = parse_packet(&raw_packet);
        let expected_eval = 2021i64;
        let actual_eval = evaluate(&packet);
        assert_eq!(actual_eval, expected_eval);
    }


    #[test]
    fn test_basic_operator() {
        let raw_packet = "38006F45291200".to_string();
        let (packet, bits_consumed) = parse_packet(&raw_packet);
        let sub_a = Packet { version: 6usize, content: PacketContent::LITERAL(10i64) };
        let sub_b = Packet { version: 2usize, content: PacketContent::LITERAL(20i64) };
        let expected_packet = Packet {
            version: 1usize,
            content: PacketContent::OPERATOR(OperatorType::LT, vec![sub_a, sub_b]),
        };
        assert_eq!(packet, expected_packet);
    }

    #[test]
    fn test_eval_basic_operator() {
        let raw_packet = "38006F45291200".to_string();
        let (packet, _) = parse_packet(&raw_packet);
        let expected_eval = 1i64;
        let actual_eval = evaluate(&packet);
        assert_eq!(actual_eval, expected_eval);
    }

    #[test]
    fn test_operator_three_children() {
        let raw_packet = "EE00D40C823060".to_string();
        let (packet, bits_consumed) = parse_packet(&raw_packet);
        let sub_a = Packet { version: 2usize, content: PacketContent::LITERAL(1i64) };
        let sub_b = Packet { version: 4usize, content: PacketContent::LITERAL(2i64) };
        let sub_c = Packet { version: 1usize, content: PacketContent::LITERAL(3i64) };
        let expected_packet = Packet {
            version: 7usize,
            content: PacketContent::OPERATOR(OperatorType::MAX, vec![sub_a, sub_b, sub_c]),
        };
        assert_eq!(packet, expected_packet);
    }

    #[test]
    fn test_eval_operator_three_children() {
        let raw_packet = "EE00D40C823060".to_string();
        let (packet, _) = parse_packet(&raw_packet);
        let expected_eval = 3i64;
        let actual_eval = evaluate(&packet);
        assert_eq!(actual_eval, expected_eval);
    }

    #[test]
    fn test_nested_operators() {
        let raw_packet = "8A004A801A8002F478".to_string();
        let (packet, _) = parse_packet(&raw_packet);
        let sub_a = Packet { version: 6usize, content: PacketContent::LITERAL(15i64) };
        let sub_b = Packet {
            version: 5usize,
            content: PacketContent::OPERATOR(OperatorType::MIN, vec![sub_a])
        };
        let sub_c = Packet {
            version: 1usize,
            content: PacketContent::OPERATOR(OperatorType::MIN, vec![sub_b])
        };
        let expected_packet = Packet {
            version: 4usize,
            content: PacketContent::OPERATOR(OperatorType::MIN, vec![sub_c]),
        };
        assert_eq!(packet, expected_packet);
    }

    #[test]
    fn test_eval_nested_operators() {
        let raw_packet = "8A004A801A8002F478".to_string();
        let (packet, _) = parse_packet(&raw_packet);
        let expected_eval = 15i64;
        let actual_eval = evaluate(&packet);
        assert_eq!(actual_eval, expected_eval);
    }

    #[test]
    fn test_op_tree() {
        let raw_packet = "620080001611562C8802118E34".to_string();
        let (packet, _) = parse_packet(&raw_packet);
        let sub_a_1 = Packet { version: 0usize, content: PacketContent::LITERAL(10i64) };
        let sub_a_2 = Packet { version: 5usize, content: PacketContent::LITERAL(11i64) };
        let sub_b_1 = Packet { version: 0usize, content: PacketContent::LITERAL(12i64) };
        let sub_b_2 = Packet { version: 3usize, content: PacketContent::LITERAL(13i64) };
        let sub_c = Packet {
            version: 0usize,
            content: PacketContent::OPERATOR(OperatorType::SUM, vec![sub_a_1, sub_a_2])
        };
        let sub_d = Packet {
            version: 1usize,
            content: PacketContent::OPERATOR(OperatorType::SUM, vec![sub_b_1, sub_b_2])
        };
        let expected_packet = Packet {
            version: 3usize,
            content: PacketContent::OPERATOR(OperatorType::SUM, vec![sub_c, sub_d]),
        };
        assert_eq!(packet, expected_packet);
    }

    #[test]

    fn test_op_tree_v2() {
        let raw_packet = "C0015000016115A2E0802F182340".to_string();
        let (packet, _) = parse_packet(&raw_packet);
        let sub_a_1 = Packet { version: 0usize, content: PacketContent::LITERAL(10i64) };
        let sub_a_2 = Packet { version: 6usize, content: PacketContent::LITERAL(11i64) };
        let sub_b_1 = Packet { version: 7usize, content: PacketContent::LITERAL(12i64) };
        let sub_b_2 = Packet { version: 0usize, content: PacketContent::LITERAL(13i64) };
        let sub_c = Packet {
            version: 0usize,
            content: PacketContent::OPERATOR(OperatorType::SUM, vec![sub_a_1, sub_a_2])
        };
        let sub_d = Packet {
            version: 4usize,
            content: PacketContent::OPERATOR(OperatorType::SUM, vec![sub_b_1, sub_b_2])
        };
        let expected_packet = Packet {
            version: 6usize,
            content: PacketContent::OPERATOR(OperatorType::SUM, vec![sub_c, sub_d]),
        };
        assert_eq!(packet, expected_packet);
    }

}


fn version_sum(packet: &Packet) -> usize {
    let child_version_sum = match &packet.content {
        PacketContent::OPERATOR(_, sub) => sub.iter().map(|x| version_sum(x)).sum(),
        PacketContent::LITERAL(_) => 0usize
    };
    child_version_sum + packet.version
}

/// Evaluates the integer value of a packet - including all operations and children.
fn evaluate(packet: &Packet) -> i64 {
    match &packet.content {
        PacketContent::OPERATOR(op_id, sub) => match op_id {
            OperatorType::SUM => sub.iter().map(|packet| evaluate(packet)).sum::<i64>(),
            OperatorType::PROD => sub.iter().map(|packet| evaluate(packet)).product::<i64>(),
            OperatorType::MIN => sub.iter().map(|packet| evaluate(packet)).min().unwrap(),
            OperatorType::MAX => sub.iter().map(|packet| evaluate(packet)).max().unwrap(),
            OperatorType::GT => if evaluate(&sub[0]) > evaluate(&sub[1]) { 1 } else { 0 },
            OperatorType::LT => if evaluate(&sub[0]) < evaluate(&sub[1]) { 1 } else { 0 },
            OperatorType::EQ => if evaluate(&sub[0]) == evaluate(&sub[1]) { 1 } else { 0 },
            _ => panic!("Unsupported oepration type for packet: {:?}", packet)
        },
        PacketContent::LITERAL(value) => *value as i64,
    }
}

fn day_16_packet_decoder() {
    let input_fname = "input/16.txt";
    let data: String = fs::read_to_string(input_fname).expect("Unable to read file.");

    let (packet, _) = parse_packet(&data);
    let part_1_answer = version_sum(&packet);
    println!("Part 1 result: {}", part_1_answer);

    let part_2_answer = evaluate(&packet);
    println!("Part 2 result: {}", part_2_answer);
}


fn main() {
    day_16_packet_decoder();
}
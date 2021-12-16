/// 2021 AoC Day 16: Packet Decoder
///
/// Shunting yard (??) and packet bit decoding.

use std::fs;
use std::collections::HashMap;
use lazy_static::lazy_static;

#[derive(Debug, Eq, PartialEq)]
enum OperatorType {
    ADD,
    SUB,
    MUL,
    DIV,
}

const LITERAL_ID: usize = 4usize;

#[derive(Debug, Eq, PartialEq)]
enum PacketContent {
    LITERAL(i32),
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

fn bin_to_dec(stuff: &[u8]) -> i32 {
    let mut idx: i32 = (stuff.len() - 1) as i32;
    let mut acc: i32 = 0;
    let mut exp: i32 = 1;

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

fn parse_packet_bits(bits: &[u8]) -> (Packet, usize) {
    let version_bits = &bits[0..3];
    let type_bits = &bits[3..6];

    let version = bin_to_dec(version_bits) as usize;
    let type_id = bin_to_dec(type_bits) as usize;
    println!("type_id = {}, version = {}", type_id, version);

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
        let mut children = Vec::new();
        // parse nested packets as needed
        let mut bit_type_id = bits[6];
        let mut packet_start = 0;
        let mut end = 0;
        if bit_type_id == 0 {
            packet_start = 7 + 15;
            let n_bits_bits = &bits[7..packet_start];
            let n_bits_sp = bin_to_dec(n_bits_bits) as usize;

            let mut result = parse_sub_packets_n_total_bits(&bits[packet_start..], n_bits_sp);
            children.append(&mut result);
            end = packet_start + n_bits_sp;
        }
        else {
            packet_start = 7 + 11;
            let n_sub_packets_bits = &bits[7..packet_start];
            let n_packets = bin_to_dec(n_sub_packets_bits);

            panic!("Not yet implemented... :(");
            // TODO return how many bits you actually consumed
            // let sp = parse_sub_packets_n_pack(&bits[packet_start..], n_packets);
        }


        (Packet{ version: version, content: PacketContent::OPERATOR(OperatorType::ADD, children) }, end)
    }
}


fn parse_packet(data: &String) -> (Packet, usize) {
    let packet_bits = hex_str_to_bits(data);
    parse_packet_bits(&packet_bits)
}


// TODO(andrei): Move entity definitions and related tests to their own file.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_basic_literal() {
        let raw_packet = "D2FE28".to_string();
        let (packet, bits_consumed) = parse_packet(&raw_packet);
        let expected_packet = Packet { version: 6usize, content: PacketContent::LITERAL(2021i32) };
        assert_eq!(packet, expected_packet);
    }

    #[test]
    fn test_basic_operator() {
        let raw_packet = "38006F45291200".to_string();
        let (packet, bits_consumed) = parse_packet(&raw_packet);
        let sub_a = Packet { version: 6usize, content: PacketContent::LITERAL(10i32) };
        let sub_b = Packet { version: 2usize, content: PacketContent::LITERAL(20i32) };
        let expected_packet = Packet {
            version: 1usize,
            content: PacketContent::OPERATOR(OperatorType::ADD, vec![sub_a, sub_b] ),
        };
        assert_eq!(packet, expected_packet);
    }

    #[test]
    fn test_operator_chain() {
        let raw_packet = "8A004A801A8002F478".to_string();
    }
}


fn day_16_packet_decoder() {
    let input_fname = "input/16.txt";
    // let input_fname = "input/16-demo.txt";

    // Input data processing
    let data: Vec<String> = fs::read_to_string(input_fname).expect("Unable to read file.")
        .split("\n").map(|x| x.to_string()).collect();

}


fn main() {
    day_16_packet_decoder();
}
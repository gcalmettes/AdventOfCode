use std::path::Path;
use std::fs;
use std::collections::HashMap;

struct Packet {
  version: u32,
  type_id: u8,
  data: PacketData,
}

enum PacketData {
  LiteralValue(u64),
  Operator(Vec<Packet>),
}

impl Packet {

  fn from_hex(hex: &str) -> Self {
    let mut code: HashMap<char, Vec<char>> = HashMap::new();
    code.insert('0', "0000".chars().collect::<Vec<_>>());
    code.insert('1', "0001".chars().collect::<Vec<_>>());
    code.insert('2', "0010".chars().collect::<Vec<_>>());
    code.insert('3', "0011".chars().collect::<Vec<_>>());
    code.insert('4', "0100".chars().collect::<Vec<_>>());
    code.insert('5', "0101".chars().collect::<Vec<_>>());
    code.insert('6', "0110".chars().collect::<Vec<_>>());
    code.insert('7', "0111".chars().collect::<Vec<_>>());
    code.insert('8', "1000".chars().collect::<Vec<_>>());
    code.insert('9', "1001".chars().collect::<Vec<_>>());
    code.insert('A', "1010".chars().collect::<Vec<_>>());
    code.insert('B', "1011".chars().collect::<Vec<_>>());
    code.insert('C', "1100".chars().collect::<Vec<_>>());
    code.insert('D', "1101".chars().collect::<Vec<_>>());
    code.insert('E', "1110".chars().collect::<Vec<_>>());
    code.insert('F', "1111".chars().collect::<Vec<_>>());

    let bits = hex.trim().chars()
        .fold(Vec::new(), |mut acc, c| {
            let mut bits = code.get(&c).unwrap().clone();
            acc.append(&mut bits);
            acc
        });
    // let bits = hex_to_bits(hex.trim());
    let binary = bits.into_iter().collect::<String>();
    Packet::parse(&binary).1
  }

  fn parse(s: &str) -> (usize, Self) {
    let mut consumed = 0;

    let (version, rest) = s.split_at(3);
    let version = u32::from_str_radix(version, 2).unwrap();
    consumed += 3;

    let (type_id, mut rest) = rest.split_at(3);
    let type_id = u8::from_str_radix(type_id, 2).unwrap();
    consumed += 3;

    let data = match type_id {
      4 => {
        // Literal value
        let mut s = String::new();
        loop {
          consumed += 5;
          let (bits, leftover) = rest.split_at(5);
          s.push_str(&bits[1..5]);
          rest = leftover;
          if bits.starts_with('0') {
            break;
          }
        }
        PacketData::LiteralValue(u64::from_str_radix(&s, 2).unwrap())
      }
      _ => {
        let (length_type, rest) = rest.split_at(1);
        consumed += 1;
        match length_type {
          "0" => { // Total length
            let (total_length, mut leftover) = rest.split_at(15);
            let mut leftover_count = usize::from_str_radix(total_length, 2).unwrap();
            consumed += 15;

            let mut sub_packets = Vec::new();
            while leftover_count > 0 {
              let (length, packet) = Packet::parse(leftover);
              let (_, rest) = leftover.split_at(length);
              consumed += length;
              sub_packets.push(packet);
              leftover_count -= length;
              leftover = rest;
            }
            PacketData::Operator(sub_packets)
          }
          "1" => { // Number of subpackets
            let (number_of_subpackets, mut leftover) = rest.split_at(11);
            let number_of_subpackets = usize::from_str_radix(number_of_subpackets, 2).unwrap();
            consumed += 11;
            let mut sub_packets = Vec::new();
            for _ in 0..number_of_subpackets {
              let (length, packet) = Packet::parse(leftover);
              let (_, rest) = leftover.split_at(length);
              consumed += length;
              sub_packets.push(packet);
              leftover = rest;
            }
            PacketData::Operator(sub_packets)
          }
          _ => unimplemented!(),
        }
      }
    };
    (consumed, Self{version, type_id, data})
  }

  fn compute_sum_version(&self) -> u32 {
    match self.data {
      PacketData::LiteralValue(_) => self.version,
      PacketData::Operator(ref sub_packets) => self.version + sub_packets.iter().map(|p| p.compute_sum_version()).sum::<u32>(),
    }
  }

  fn evaluate_expression(&self) -> u64 {
    match self.data {
      PacketData::LiteralValue(v) => v,
      PacketData::Operator(ref sub_packets) => match self.type_id {
        0 => sub_packets.iter().map(|p| p.evaluate_expression()).sum::<u64>(),
        1 => sub_packets.iter().map(|p| p.evaluate_expression()).product::<u64>(),
        2 => sub_packets.iter().map(|p| p.evaluate_expression()).min().unwrap(),
        3 => sub_packets.iter().map(|p| p.evaluate_expression()).max().unwrap(),
        5 => {
          if sub_packets[0].evaluate_expression() > sub_packets[1].evaluate_expression() {
            1
          } else {
            0
          }
        }
        6 => {
          if sub_packets[0].evaluate_expression() < sub_packets[1].evaluate_expression() {
            1
          } else {
            0
          }
        }
        7 => {
          if sub_packets[0].evaluate_expression() == sub_packets[1].evaluate_expression() {
            1
          } else {
            0
          }
        }
        _ => unimplemented!(),
      },
    }
  }
}

fn main() {
    let path = Path::new("./inputs/day16.txt");
    let input = fs::read_to_string(path).expect("Unable to read file");

    let packet = Packet::from_hex(&input);
    let p1 = packet.compute_sum_version();
    println!("{}", p1);
    let p2 = packet.evaluate_expression();
    println!("{}", p2);
}

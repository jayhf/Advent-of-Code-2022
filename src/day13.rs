use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Less};

#[derive(Clone)]
enum Packet {
    Val(i32),
    More(Vec<Packet>),
}

impl Packet {
    fn parse(packet: &str) -> Packet {
        let mut subpackets: Vec<Packet> = Vec::new();
        let mut depth = 0;
        let mut last_end = 1;
        for (i, c) in packet.chars().enumerate() {
            if c == '[' {
                depth = depth + 1;
            }
            if depth == 1 && (c == ',' || c == ']') && last_end != i {
                subpackets.push(match packet[last_end..i].parse::<i32>() {
                    Ok(x) => Packet::Val(x),
                    Err(_) => Packet::parse(&packet[last_end..i]),
                });
                last_end = i + 1;
            }
            if c == ']' {
                depth = depth - 1;
            }
        }
        Packet::More(subpackets)
    }
}

impl Eq for Packet {}

impl PartialEq<Self> for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Equal
    }
}

impl PartialOrd<Self> for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Val(x1), Packet::Val(x2)) => x1.cmp(&x2),
            (Packet::Val(x), p) => Packet::More(Vec::from([Packet::Val(*x)])).cmp(p),
            (p, Packet::Val(x)) => p.cmp(&Packet::More(Vec::from([Packet::Val(*x)]))),
            (Packet::More(m1), Packet::More(m2)) => m1
                .iter()
                .zip(m2.iter())
                .map(|(p1, p2)| p1.cmp(p2))
                .filter(|ord| *ord != Equal)
                .next()
                .unwrap_or(m1.len().cmp(&m2.len())),
        }
    }
}

pub fn solve_part_1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|line| {
            let (l1, l2) = line.split_once("\n").unwrap();
            (Packet::parse(l1), Packet::parse(l2))
        })
        .enumerate()
        .filter(|(_, (p1, p2))| p1.cmp(&p2) == Less)
        .map(|(i, _)| i + 1)
        .sum()
}

pub fn solve_part_2(input: &str) -> usize {
    let mut packets: Vec<Packet> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| Packet::parse(line))
        .collect();
    let d1 = Packet::More(Vec::from([Packet::Val(2)]));
    let d2 = Packet::More(Vec::from([Packet::Val(6)]));
    packets.push(d1.clone());
    packets.push(d2.clone());
    packets.sort();
    let p1 = packets.iter().position(|p| p == &d1).unwrap();
    let p2 = packets.iter().position(|p| p == &d2).unwrap();
    (p1 + 1) * (p2 + 1)
}

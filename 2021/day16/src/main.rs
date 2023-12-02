use std::path::Path;

#[derive(Debug, PartialEq, Eq)]
struct BitStream {
    data: Vec<u64>,
    cursor: usize,
}

impl BitStream {
    fn from_hex(s: &str) -> Self {
        let mut data: Vec<u64> = Vec::new();
        let mut cur: u64 = 0;
        let mut digit_count: usize = 0;

        const DIGITS_U64: usize = 16;

        for c in s.chars() {
            let v = c.to_digit(16).unwrap() as u8;
            let v = (v.reverse_bits() >> 4) as u64;
            cur |= v << (digit_count * 4);
            digit_count += 1;
            if digit_count == DIGITS_U64 {
                data.push(cur);
                cur = 0;
                digit_count = 0;
            }
        }

        if digit_count < DIGITS_U64 && digit_count != 0 {
            data.push(cur);
        }

        Self { data, cursor: 0 }
    }

    fn full_mask(n: u8) -> u64 {
        (1 << n) - 1
    }

    fn get_bits_in_word(&self, idx: usize, start: u8, n: u8) -> u64 {
        assert!(start + n <= 64);
        (self.data[idx] & (Self::full_mask(n) << start)) >> start
    }

    fn take_bits(&mut self, n: u8) -> Option<u64> {
        assert!(n <= 64, "Can't handle bits > 64");
        let end = self.cursor + n as usize;
        if (self.data.len() * 64) < end {
            return None;
        }

        let word_cursor: u8 = (self.cursor % 64).try_into().unwrap();
        let vi_before = self.cursor / 64;
        let vi_after = end / 64;
        let out;

        if vi_after == vi_before {
            out = self.get_bits_in_word(vi_before, word_cursor, n);
        } else {
            assert_eq!(vi_before + 1, vi_after);
            let n_before = 64 - word_cursor;
            let n_after = n - n_before;
            let bits_before = self.get_bits_in_word(vi_before, word_cursor, n_before);
            let bits_after = self.get_bits_in_word(vi_before + 1, 0, n_after);

            out = bits_after << n_before | bits_before;
        }

        self.cursor = end;
        Some(out.reverse_bits() >> (64 - n))
    }

    fn pos(&self) -> usize {
        self.cursor
    }
}

// Debug utility
#[derive(Debug, PartialEq, Eq)]
struct Span {
    start: usize,
    end: usize,
}

#[derive(Debug, PartialEq, Eq)]
enum Op {
    Sum,
    Product,
    Min,
    Max,
    Gt,
    Lt,
    Eq,
}

#[derive(Debug, PartialEq, Eq)]
enum Packet {
    Lit {
        version: u8,
        value: u64,
        span: Span,
    },
    Op {
        version: u8,
        op: Op,
        packets: Vec<Packet>,
        span: Span,
    },
}

impl Packet {
    const LITERAL_TYPE_ID: u8 = 4;

    fn decode(bs: &mut BitStream) -> Option<Self> {
        let start = bs.pos();
        let version: u8 = bs.take_bits(3)?.try_into().unwrap();
        let ty: u8 = bs.take_bits(3)?.try_into().unwrap();
        if ty == Self::LITERAL_TYPE_ID {
            let mut done = false;
            let mut value: u64 = 0;
            for _ in 0..64 {
                let group_prefix = bs.take_bits(1)?;
                value = value << 4 | bs.take_bits(4)?;
                if group_prefix == 0 {
                    done = true;
                    break;
                }
                assert_eq!(group_prefix, 1, "Bad group prefix");
            }
            assert!(done, "Bad literal, did not finish in 64 bits");
            let end = bs.pos();
            done.then_some(Self::Lit {
                version,
                value,
                span: Span { start, end },
            })
        } else {
            let op = match ty {
                0 => Op::Sum,
                1 => Op::Product,
                2 => Op::Min,
                3 => Op::Max,
                5 => Op::Gt,
                6 => Op::Lt,
                7 => Op::Eq,
                _bad => unreachable!("Invalid op type: {}", _bad),
            };

            let length_type = bs.take_bits(1)?;
            let mut packets = Vec::new();
            if length_type == 0 {
                let n_bits = bs.take_bits(15)?;
                let start = bs.pos();
                while bs.pos() - start < n_bits as usize {
                    packets.push(Self::decode(bs)?);
                }
            } else {
                assert_eq!(length_type, 1);
                let n_packets = bs.take_bits(11)?;
                packets.reserve(n_packets as usize);
                for _ in 0..n_packets {
                    packets.push(Self::decode(bs)?);
                }
            }

            let end = bs.pos();
            Some(Self::Op {
                version,
                op,
                packets,
                span: Span { start, end },
            })
        }
    }

    fn parse(s: &str) -> Option<Self> {
        let mut bs = BitStream::from_hex(s);

        Self::decode(&mut bs)
    }

    fn visit<F>(&self, f: &mut F)
    where
        F: FnMut(&Packet),
    {
        f(self);
        if let Self::Op { packets, .. } = self {
            for p in packets {
                p.visit(f);
            }
        }
    }

    fn eval(&self) -> u64 {
        match self {
            Self::Lit { value, .. } => *value,
            Self::Op { op, packets, .. } => {
                let mut args = packets.iter().map(|p| p.eval());
                match op {
                    Op::Sum => args.sum(),
                    Op::Product => args.product(),
                    Op::Min => args.min().unwrap(),
                    Op::Max => args.max().unwrap(),
                    Op::Gt => (args.next().unwrap() > args.next().unwrap()) as _,
                    Op::Lt => (args.next().unwrap() < args.next().unwrap()) as _,
                    Op::Eq => (args.next().unwrap() == args.next().unwrap()) as _,
                }
            }
        }
    }
}

fn sum_versions(p: &Packet) -> usize {
    let mut version_sum: usize = 0;
    p.visit(&mut |pkt| match pkt {
        Packet::Lit { version, .. } => {
            version_sum += *version as usize;
        }
        Packet::Op { version, .. } => {
            version_sum += *version as usize;
        }
    });

    version_sum
}

fn part1(path: impl AsRef<Path>) -> Result<(), Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string(path)?;
    let p = Packet::parse(&contents).expect("Failed to parse packet");
    println!("PART 1 sum: {}", sum_versions(&p));
    Ok(())
}

fn part2(path: impl AsRef<Path>) -> Result<(), Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string(path)?;
    let p = Packet::parse(&contents).expect("Failed to parse packet");
    println!("PART 2 result: {}", p.eval());
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("usage: {} <file>", args[0]);
        return Err("Error: expected file arg".into());
    }

    part1(&args[1])?;
    part2(&args[1])?;

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn lit() {
        let p = Packet::parse("D2FE28").expect("Expected packet");
        let expected = Packet::Lit {
            version: 6,
            value: 2021,
            span: Span { start: 0, end: 21 },
        };
        assert_eq!(p, expected);
    }

    #[test]
    fn op0() {
        let p = Packet::parse("38006F45291200").expect("Expected packet");
        let expected = Packet::Op {
            version: 1,
            op: Op::Lt,
            span: Span { start: 0, end: 49 },
            packets: vec![
                Packet::Lit {
                    version: 6,
                    value: 10,
                    span: Span { start: 22, end: 33 },
                },
                Packet::Lit {
                    version: 2,
                    value: 20,
                    span: Span { start: 33, end: 49 },
                },
            ],
        };
        assert_eq!(p, expected);
    }

    #[test]
    fn op1() {
        let p = Packet::parse("EE00D40C823060").expect("Expected packet");
        let expected = Packet::Op {
            version: 7,
            op: Op::Max,
            span: Span { start: 0, end: 51 },
            packets: vec![
                Packet::Lit {
                    version: 2,
                    value: 1,
                    span: Span { start: 18, end: 29 },
                },
                Packet::Lit {
                    version: 4,
                    value: 2,
                    span: Span { start: 29, end: 40 },
                },
                Packet::Lit {
                    version: 1,
                    value: 3,
                    span: Span { start: 40, end: 51 },
                },
            ],
        };
        assert_eq!(p, expected);
    }

    #[test]
    fn version_sum_0() {
        let p = Packet::parse("8A004A801A8002F478").expect("Expected packet");
        let sum = sum_versions(&p);
        assert_eq!(sum, 16);
    }

    #[test]
    fn version_sum_1() {
        let p = Packet::parse("620080001611562C8802118E34").expect("Expected packet");
        dbg!(&p);
        let sum = sum_versions(&p);
        assert_eq!(sum, 12);
    }

    #[test]
    fn version_sum_2() {
        let p = Packet::parse("C0015000016115A2E0802F182340").expect("Expected packet");
        let sum = sum_versions(&p);
        assert_eq!(sum, 23);
    }

    #[test]
    fn version_sum_3() {
        let p = Packet::parse("A0016C880162017C3686B18A3D4780").expect("Expected packet");
        let sum = sum_versions(&p);
        assert_eq!(sum, 31);
    }

    #[test]
    fn eval0() {
        let p = Packet::parse("C200B40A82").expect("Expected packet");
        assert_eq!(p.eval(), 3);
    }

    #[test]
    fn eval1() {
        let p = Packet::parse("04005AC33890").expect("Expected packet");
        assert_eq!(p.eval(), 54);
    }

    #[test]
    fn eval2() {
        let p = Packet::parse("04005AC33890").expect("Expected packet");
        assert_eq!(p.eval(), 54);
    }

    #[test]
    fn eval3() {
        let p = Packet::parse("880086C3E88112").expect("Expected packet");
        assert_eq!(p.eval(), 7);
    }

    #[test]
    fn eval4() {
        let p = Packet::parse("CE00C43D881120").expect("Expected packet");
        assert_eq!(p.eval(), 9);
    }

    #[test]
    fn eval5() {
        let p = Packet::parse("880086C3E88112").expect("Expected packet");
        assert_eq!(p.eval(), 7);
    }

    #[test]
    fn eval6() {
        let p = Packet::parse("CE00C43D881120").expect("Expected packet");
        assert_eq!(p.eval(), 9);
    }

    #[test]
    fn eval7() {
        let p = Packet::parse("880086C3E88112").expect("Expected packet");
        assert_eq!(p.eval(), 7);
    }

    #[test]
    fn eval8() {
        let p = Packet::parse("CE00C43D881120").expect("Expected packet");
        assert_eq!(p.eval(), 9);
    }

    #[test]
    fn eval9() {
        let p = Packet::parse("D8005AC2A8F0").expect("Expected packet");
        assert_eq!(p.eval(), 1);
    }
    #[test]
    fn eval10() {
        let p = Packet::parse("F600BC2D8F").expect("Expected packet");
        assert_eq!(p.eval(), 0);
    }
    #[test]
    fn eval11() {
        let p = Packet::parse("9C005AC2F8F0").expect("Expected packet");
        assert_eq!(p.eval(), 0);
    }
    #[test]
    fn eval12() {
        let p = Packet::parse("9C0141080250320F1802104A08").expect("Expected packet");
        assert_eq!(p.eval(), 1);
    }
}

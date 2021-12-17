use std::cmp::{max, min};
use std::io;

enum Operation {
    Sum,
    Mul,
    Min,
    Max,
    Greater,
    Less,
    Equal,
}

impl Operation {
    fn unit(&self) -> Option<u64> {
        match *self {
            Operation::Sum => Some(0),
            Operation::Mul => Some(1),
            Operation::Min => Some(1_000_000_000_000_000_000),
            Operation::Max => Some(0),
            _ => None,
        }
    }

    fn eval(&self, left: u64, right: u64) -> u64 {
        match *self {
            Operation::Sum => left + right,
            Operation::Mul => left * right,
            Operation::Min => min(left, right),
            Operation::Max => max(left, right),
            Operation::Greater => {
                if left > right {
                    1
                } else {
                    0
                }
            }
            Operation::Less => {
                if left < right {
                    1
                } else {
                    0
                }
            }
            Operation::Equal => {
                if left == right {
                    1
                } else {
                    0
                }
            }
        }
    }
}

fn bin_to_u64(bin: &[char]) -> u64 {
    u64::from_str_radix(&bin.into_iter().collect::<String>(), 2).unwrap()
}

// return (version sum, literal value sum, read bytes)
fn process_data(data: &[char], op: Operation) -> (u64, u64, usize) {
    let mut version_sum = 0;
    let mut literal_value = op.unit();
    if data[0] == '0' {
        let sub_bytes = bin_to_u64(&data[1..16]) as usize;
        let mut read = 0;
        while read < sub_bytes {
            let (ver, lv, plen) = parse_packet(&data[16 + read..data.len()]);
            version_sum += ver;
            match literal_value {
                None => literal_value = Some(lv),
                Some(curr) => literal_value = Some(op.eval(curr, lv)),
            };
            read += plen;
        }
        assert_eq!(read, sub_bytes);
        (version_sum, literal_value.unwrap(), 16 + read)
    } else {
        let sub_count = bin_to_u64(&data[1..12]) as usize;
        let mut read = 0;
        for _ in 0..sub_count {
            let (ver, lv, plen) = parse_packet(&data[12 + read..data.len()]);
            version_sum += ver;
            match literal_value {
                None => literal_value = Some(lv),
                Some(curr) => literal_value = Some(op.eval(curr, lv)),
            };
            read += plen;
        }
        (version_sum, literal_value.unwrap(), 12 + read)
    }
}

// return (version sum, literal value sum, read bytes)
fn parse_packet(bits: &[char]) -> (u64, u64, usize) {
    let version = bin_to_u64(&bits[0..3]);
    let type_ = bits.into_iter().skip(3).take(3).collect::<String>();
    let data = bits.into_iter().skip(6).map(|x| *x).collect::<Vec<char>>();

    // literal value
    if type_ == "100" {
        let mut bin = "".to_string();
        let mut read = 0;
        loop {
            bin += &data[read + 1..read + 5].into_iter().collect::<String>();
            read += 5;
            if data[read - 5] == '0' {
                break;
            }
        }
        let literal_value_sum = u64::from_str_radix(&bin, 2).unwrap();
        return (version, literal_value_sum, 6 + read);
    }

    // operations
    let (sub_version_sum, literal_value, data_len) = match &type_[..] {
        "000" => process_data(&data, Operation::Sum),
        "001" => process_data(&data, Operation::Mul),
        "010" => process_data(&data, Operation::Min),
        "011" => process_data(&data, Operation::Max),
        "101" => process_data(&data, Operation::Greater),
        "110" => process_data(&data, Operation::Less),
        "111" => process_data(&data, Operation::Equal),
        _ => panic!(),
    };
    (version + sub_version_sum, literal_value, 6 + data_len)
}

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let chars = line
        .trim()
        .chars()
        .flat_map(|hex| {
            format!("{:04b}", hex.to_digit(16).unwrap())
                .to_string()
                .chars()
                .collect::<Vec<char>>()
        })
        .collect::<Vec<char>>();
    let res = parse_packet(&chars);
    println!(
        "version sum: {}, literal value: {}, read: {}",
        res.0, res.1, res.2
    );
}

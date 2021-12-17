use std::io;

fn bin_to_u64(bin: &[char]) -> u64 {
    u64::from_str_radix(&bin.into_iter().collect::<String>(), 2).unwrap()
}

// return (version sum, read bytes)
fn process_data(data: &[char]) -> (u64, usize) {
    let mut version_sum = 0;
    if data[0] == '0' {
        let sub_bytes = bin_to_u64(&data[1..16]) as usize;
        let mut read = 0;
        while read < sub_bytes {
            let (ver, plen) = parse_packet(&data[16 + read..data.len()]);
            version_sum += ver;
            read += plen;
        }
        assert_eq!(read, sub_bytes);
        (version_sum, 16 + read)
    } else {
        let sub_count = bin_to_u64(&data[1..12]) as usize;
        let mut read = 0;
        for _ in 0..sub_count {
            let (ver, plen) = parse_packet(&data[12 + read..data.len()]);
            version_sum += ver;
            read += plen;
        }
        (version_sum, 12 + read)
    }
}

// return (version sum, read bytes)
fn parse_packet(bits: &[char]) -> (u64, usize) {
    let version = bin_to_u64(&bits[0..3]);
    let type_ = bits.into_iter().skip(3).take(3).collect::<String>();
    let data = bits.into_iter().skip(6).map(|x| *x).collect::<Vec<char>>();

    if type_ != "100" {
        let (sub_version_sum, data_len) = process_data(&data);
        (version + sub_version_sum, 6 + data_len)
    } else {
        let mut read = 5;
        while data[read - 5] != '0' {
            read += 5;
        }
        (version, 6 + read)
    }
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
    println!("version sum: {}, read: {}", res.0, res.1);
}

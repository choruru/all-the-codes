use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    // The first birth after X days.
    let mut first_births = line
        .trim()
        .split(",")
        .map(|x| x.parse::<i64>().unwrap() + 1)
        .collect::<Vec<i64>>();

    let mut index = 0;
    loop {
        if index == first_births.len() {
            break;
        }
        let mut first_birth = first_births[index];
        while first_birth <= 80 {
            first_births.push(first_birth + 9);
            first_birth += 7;
        }
        index += 1;
    }
    println!("{}", first_births.len());
}

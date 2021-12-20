use std::io;

fn enhance_image(map: &Vec<char>, image: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let curr_out = image[0][0];
    let next_out = if curr_out == '.' { map[0] } else { map[511] };
    let ylen = image.len();
    let xlen = image[0].len();
    let mut new = vec![vec!['.'; xlen]; ylen];
    for y in 0..image.len() {
        for x in 0..image[0].len() {
            let mut val = vec![];
            for dy in -1..=1 {
                for dx in -1..=1 {
                    let target_y = y as i32 + dy;
                    let target_x = x as i32 + dx;
                    let mut c = curr_out;
                    if target_y >= 0
                        && target_y < image.len() as i32
                        && target_x >= 0
                        && target_x < image[0].len() as i32
                    {
                        c = image[target_y as usize][target_x as usize];
                    }
                    val.push(c);
                }
            }
            let bin = val
                .into_iter()
                .map(|c| if c == '.' { '0' } else { '1' })
                .collect::<String>();
            let index = usize::from_str_radix(&bin, 2).unwrap();
            new[y][x] = map[index];
        }
    }
    make_padded_image(new, next_out == '.')
}

fn make_padded_image(image: Vec<Vec<char>>, dot: bool) -> Vec<Vec<char>> {
    let mut new = vec![];
    let len = &image[0].len();
    let c = if dot { '.' } else { '#' };
    new.push(vec![c; len + 2]);
    for mut line in image.into_iter() {
        line.insert(0, c);
        line.push(c);
        new.push(line);
    }
    new.push(vec![c; len + 2]);
    new
}

fn print_image(image: &Vec<Vec<char>>) {
    for l in image.into_iter() {
        println!("{:?}", l.into_iter().collect::<String>());
    }
    println!();
}

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let map = line.chars().collect::<Vec<char>>();
    io::stdin().read_line(&mut line).unwrap();
    line.clear();

    // image construction
    let mut image = vec![];
    while let Ok(n) = io::stdin().read_line(&mut line) {
        if n == 0 {
            break;
        }
        let l = line.trim().chars().collect::<Vec<char>>();
        image.push(l);
        line.clear();
    }

    image = make_padded_image(image, true);

    for _ in 0..50 {
        image = enhance_image(&map, &image);
    }

    let lit: usize = image
        .into_iter()
        .map(|l| l.into_iter().filter(|c| *c == '#').count())
        .sum();

    println!("{}", lit);
}

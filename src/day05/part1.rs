fn main() {
    let result = solve(include_str!("input")).unwrap();
    println!("{result}");
}

fn solve(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let mut seeds: Vec<u64> = lines
        .next()?
        .split_once(':')?
        .1
        .trim()
        .split(' ')
        .map(|seed| seed.parse::<u64>().unwrap())
        .collect();
    lines.next();
    while let Some(_header) = lines.next() {
        let mut maps = Vec::new();
        while let Some(line) = lines.next() {
            if line.is_empty() {
                break;
            }
            let numbers: Vec<u64> = line
                .split(' ')
                .map(|number| number.parse().unwrap())
                .collect();
            maps.push((numbers[0], numbers[1], numbers[2]));
        }
        for i in 0..seeds.len() {
            let seed = seeds[i];
            for map in &maps {
                let diff = seed as i64 - map.1 as i64;

                if diff >= 0 {
                    let diff = diff as u64;
                    if diff < map.2 {
                        seeds[i] = map.0 + diff;
                    }
                }
            }
        }
    }
    Some(seeds.into_iter().min()?)
}

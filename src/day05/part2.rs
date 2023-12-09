use std::cmp::{max, min};

fn main() {
    let result = solve(include_str!("input")).unwrap();
    println!("{result}");
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Range {
    start: u64,
    length: u64,
}

struct SplitResult {
    mapped: Vec<Range>,
    unmapped: Vec<Range>,
}

impl Range {
    fn end(&self) -> u64 {
        // half-open
        self.start + self.length
    }

    fn overlaps(&self, tested: &Range) -> bool {
        !(tested.end() <= self.start || self.end() <= tested.start)
    }

    fn apply_mapping(&self, mapping: Range, destination: u64) -> SplitResult {
        let mut mapped = Vec::new();
        let mut unmapped = Vec::new();
        let offset = mapping.start as i64 - self.start as i64;
        if offset > 0 {
            unmapped.push(Range {
                start: self.start,
                length: offset.try_into().unwrap(),
            });
        }
        if mapping.end() < self.end() {
            unmapped.push(Range {
                start: mapping.end(),
                length: self.end() - mapping.end(),
            });
        }
        mapped.push(Range {
            start: (destination as i64 - min(0, offset)).try_into().unwrap(),
            length: min(self.end(), mapping.end()) - max(self.start, mapping.start),
        });
        SplitResult { mapped, unmapped }
    }
}

fn solve(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let mut seeds: Vec<Range> = seeds(lines.next()?.split_once(':')?.1);
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
        let mut new_seeds = Vec::new();
        let mut i = 0;
        'seeds: while i < seeds.len() {
            for mapping in &maps {
                let mapping_range = Range {
                    start: mapping.1,
                    length: mapping.2,
                };
                if seeds[i].overlaps(&mapping_range) {
                    let mut mapping_result = seeds[i].apply_mapping(mapping_range, mapping.0);
                    seeds.append(&mut mapping_result.unmapped);
                    new_seeds.append(&mut mapping_result.mapped);
                    i += 1;
                    continue 'seeds;
                }
            }
            new_seeds.push(seeds[i].clone());
            i += 1;
        }
        seeds = new_seeds;
    }
    Some(seeds.into_iter().map(|range| range.start).min()?)
}

fn seeds(line: &str) -> Vec<Range> {
    let mut seeds = Vec::new();
    let numbers = line
        .trim()
        .split(' ')
        .map(|seed| seed.parse::<u64>().unwrap());
    let mut last = None;
    for number in numbers {
        if let Some(last_number) = last {
            seeds.push(Range {
                start: last_number,
                length: number,
            });
            last = None;
        } else {
            last = Some(number);
        }
    }
    seeds
}

#[cfg(test)]
mod test {
    use super::Range;

    const MAPPING: Range = Range {
        start: 20,
        length: 10,
    };

    #[test]
    fn mapping_contains_seeds() {
        let seeds = Range {
            start: 25,
            length: 2,
        };
        let res = seeds.apply_mapping(MAPPING, 100);
        assert_eq!(res.unmapped.len(), 0);
        assert_eq!(res.mapped.len(), 1);
        assert_eq!(
            res.mapped[0],
            Range {
                start: 105,
                length: 2
            }
        );
    }

    #[test]
    fn seeds_contains_mapping() {
        let seeds = Range {
            start: 10,
            length: 30,
        };
        let res = seeds.apply_mapping(MAPPING, 200);
        assert_eq!(res.unmapped.len(), 2);
        assert_eq!(res.mapped.len(), 1);
        assert_eq!(
            res.mapped[0],
            Range {
                start: 200,
                length: 10
            }
        );
        assert_eq!(
            res.unmapped[0],
            Range {
                start: 10,
                length: 10
            }
        );
        assert_eq!(
            res.unmapped[1],
            Range {
                start: 30,
                length: 10
            }
        );
    }

    #[test]
    fn seeds_start_first() {
        let seeds = Range {
            start: 15,
            length: 10,
        };
        let res = seeds.apply_mapping(MAPPING, 300);
        assert_eq!(res.mapped.len(), 1);
        assert_eq!(res.unmapped.len(), 1);
        assert_eq!(
            res.mapped[0],
            Range {
                start: 300,
                length: 5
            }
        );
        assert_eq!(
            res.unmapped[0],
            Range {
                start: 15,
                length: 5
            }
        );
    }

    #[test]
    fn mapping_start_first() {
        let seeds = Range {
            start: 28,
            length: 5,
        };
        let res = seeds.apply_mapping(MAPPING, 400);
        assert_eq!(res.mapped.len(), 1);
        assert_eq!(res.unmapped.len(), 1);
        assert_eq!(
            res.mapped[0],
            Range {
                start: 408,
                length: 2,
            },
        );
        assert_eq!(
            res.unmapped[0],
            Range {
                start: 30,
                length: 3
            }
        );
    }
}

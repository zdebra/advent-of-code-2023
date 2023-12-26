use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let text = std::fs::read_to_string(&args[1]).expect("read failed");

    let chunks = text.split("\n\n").collect::<Vec<&str>>();
    let seed_ranges = chunks[0]
        .split_whitespace()
        .skip(1)
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
        .chunks(2)
        .map(|ch| {
            let range_start = ch[0];
            let len = ch[1];
            SeedRange {
                start: range_start,
                len,
            }
        })
        .collect();

    let maps = &chunks[1..];

    maps.iter().for_each(|m| {
        let ranges = parse_ranges(m);
        seeds = seeds
            .iter()
            .map(|s| {
                let mut new_seed = *s;
                for r in &ranges {
                    if r.in_range(new_seed) {
                        new_seed = r.dest_start + (new_seed - r.src_start);
                        break;
                    }
                }
                new_seed
            })
            .collect();
    });

    // println!("seeds: {:?}", seeds);
    println!("min seeds: {}", seeds.iter().min().unwrap());
}

// destination_range_start source_range_start range_length
// soil-to-fertilizer map:
// 0 15 37
// 37 52 2
// 39 0 15
fn parse_ranges(map_str: &str) -> Vec<Range> {
    let mut ranges = Vec::new();
    let lines = map_str.split("\n").skip(1).collect::<Vec<&str>>();
    for line in lines {
        let splits = line.split_whitespace().collect::<Vec<&str>>();
        let dest_start = splits[0].parse::<u64>().unwrap();
        let src_start = splits[1].parse::<u64>().unwrap();
        let len = splits[2].parse::<u64>().unwrap();
        ranges.push(Range {
            src_start,
            dest_start,
            len,
        });
    }
    ranges
}

struct Range {
    src_start: u64,
    dest_start: u64,
    len: u64,
}

impl Range {
    fn in_range(&self, val: u64) -> bool {
        val >= self.src_start && val < self.src_start + self.len
    }
}

struct SeedRange {
    start: u64,
    len: u64,
}

fn range_match(r1: &SeedRange, r2: &Range) -> Option<Range> {
    if r1.start >= r2.src_start && r1.start < r2.src_start + r2.len {
        let dest_start = r2.dest_start + (r1.start - r2.src_start);
        let len = r1.len;
        Some(Range {
            src_start: r1.start,
            dest_start,
            len,
        })
    } else {
        None
    }
}

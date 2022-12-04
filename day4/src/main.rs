use std::fs;
use std::str::FromStr;

fn parse_range(range: &str) -> (u32, u32) {
    let mut pair = range.splitn(2, "-");

    (
        FromStr::from_str(pair.next().unwrap()).unwrap(),
        FromStr::from_str(pair.next().unwrap()).unwrap(),
    )
}

fn main() {
    let content = fs::read_to_string("input").expect("Should have been able to read the file");

    let result: u32 = content
        .split("\r\n")
        .map(|line| {
            let mut pair = line.splitn(2, ",");
            let left = parse_range(pair.next().unwrap());
            let right = parse_range(pair.next().unwrap());

            // left start is smaller than right start
            if left.0 < right.0 {
                // right end is smaller than left end: contained
                if right.1 <= left.1 {
                    1
                // right end is higher then left end: not contained
                } else {
                    0
                }
            // right start is smaller than left start
            } else if left.0 > right.0 {
                // left end is smaller than right end: contained
                if left.1 <= right.1 {
                    1
                // right end is smaller than left end: not contained
                } else {
                    0
                }
            } else {
                1
            }
        })
        .sum();
    println!("{:?}", result);

    let result: u32 = content
        .split("\r\n")
        .map(|line| {
            let mut pair = line.splitn(2, ",");
            let left = parse_range(pair.next().unwrap());
            let right = parse_range(pair.next().unwrap());

            if left.0 < right.0 {
                if left.1 >= right.0 {
                    1
                } else {
                    0
                }
            } else if right.0 < left.0 {
                if right.1 >= left.0 {
                    1
                } else {
                    0
                }
            } else {
                1
            }
        })
        .sum();
    println!("{:?}", result);
}

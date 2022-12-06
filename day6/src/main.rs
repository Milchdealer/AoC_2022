use itertools::Itertools;
use std::fs;

fn find_first_marker(data: String) -> u64 {
    let mut counter: u64 = 0;
    let mut it = data
        .chars()
        .tuple_windows::<(_, _, _, _)>()
        .skip_while(|chars| {
            counter += 1;
            if chars.0 == chars.1
                || chars.0 == chars.2
                || chars.0 == chars.3
                || chars.1 == chars.2
                || chars.1 == chars.3
                || chars.2 == chars.3
            {
                true
            } else {
                false
            }
        });

    println!("{:?}", it.next());

    counter += 3;
    counter
}

fn find_message_marker(data: String) -> usize {
    let chars: Vec<(usize, char)> = data.clone().chars().enumerate().collect();

    for (idx, _char) in chars.clone() {
        if idx + 14 <= chars.len() {
            let unique = data[idx..idx + 14].chars().into_iter().all_unique();

            if unique {
                println!("{:?}", &data[idx..idx + 14]);
                return idx + 14;
            } else {
                continue;
            }
        }
    }

    0
}

fn main() {
    let content = fs::read_to_string("input").expect("Should have been able to read the file");

    let result = find_first_marker(content.clone());
    println!("{}", result);

    let result = find_message_marker(content.clone());
    println!("{}", result);
}

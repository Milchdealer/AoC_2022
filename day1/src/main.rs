use std::fs;

fn process_elf_calories(elf_items: &str) -> i64 {
    elf_items
        .split("\n")
        .map(|line| line.parse::<i64>().expect("Failed to parse input"))
        .sum()
}

fn process_input_file(content: String) -> Vec<i64> {
    content
        .split("\n\n")
        .map(process_elf_calories)
        .collect::<Vec<_>>()
}

fn get_top_n(mut list: Vec<i64>, n: usize) -> Vec<i64> {
    list.sort();
    list.iter()
        .rev()
        .take(n)
        .map(|x| x.to_owned())
        .collect::<Vec<_>>()
}

fn main() {
    let content = fs::read_to_string("input").expect("Should have been able to read the file");

    let elf_calories = process_input_file(content);
    let max_calories = elf_calories.iter().max().unwrap();
    println!("{:?}", max_calories);

    let top_three: i64 = get_top_n(elf_calories, 3).iter().sum();
    println!("{:?}", top_three);
}

use std::fs;

fn main() {
    day1();
}

fn day1() {
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path).unwrap();
    let lines = contents.split("\n");
    let mut totals = Vec::new();
    let mut total = 0;
    for line in lines {
        if line.is_empty() {
            totals.push(total);
            total = 0;
            continue;
        }
        total += line.parse::<u32>().unwrap();
    }
    if total > 0 {
        totals.push(total);
        total = 0;
    }
    let max = totals.iter().max().unwrap();

    println!("Max: {}", max);

    totals.sort();

    let max_index = totals.len();
    let top_three_sum: u32 = totals.get(max_index - 3 .. max_index).unwrap().iter().sum();

    println!("Top three sum: {}", top_three_sum);
}
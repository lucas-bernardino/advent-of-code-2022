fn main() {
    part1();
}

fn part1() {
    let content = std::fs::read_to_string("input.txt").expect("Missing input.txt");

    let priority_array = content
        .split("\n")
        .filter(|e| e.len() > 0)
        .map(|rucksack| {
            rucksack
                .split_at(rucksack.len() / 2)
                .1
                .chars()
                .filter(|c| {
                    rucksack
                        .split_at(rucksack.len() / 2)
                        .0
                        .contains(*c)
                        .eq(&true)
                })
                .next()
                .unwrap()
        })
        .map(|c| {
            if c.is_lowercase() {
                c as i32 - 96
            } else {
                c as i32 - 38
            }
        })
        .collect::<Vec<_>>();

    let sum = priority_array.iter().sum::<i32>();

    println!("Contents are: {:#?}", priority_array);
    println!("Sum is: {}", sum)
}

fn main() {
    more_idiomatic();
    my_solution();
}

fn my_solution() {
    let content = std::fs::read_to_string("input.txt").unwrap();
    let mut array_sum: Vec<i32> = vec![];

    let content_split: Vec<i32> = content
        .lines()
        .map(|e| e.parse::<i32>().unwrap_or(-1))
        .collect();

    let mut sum = 0;

    for value in content_split {
        if value == -1 {
            array_sum.push(sum);
            sum = 0;
            continue;
        }
        sum += value;
    }

    let max = array_sum.iter().max().unwrap();
    let pos = array_sum.iter().position(|x| x == max).unwrap();

    let mut top_three_sum = array_sum.clone();
    top_three_sum.sort();
    let sum_three: i32 = top_three_sum.iter().rev().take(3).sum();

    println!("MAX: {max}\nPOS: {pos}\nTOP THREE SUM: {sum_three}");
}

fn more_idiomatic() {
    let content = std::fs::read_to_string("input.txt").unwrap();

    let array_sum = content
        .lines()
        .map(|e| e.parse::<i32>().unwrap_or(0))
        .collect::<Vec<i32>>()
        .split(|x| x == &0)
        .map(|vecs| vecs.iter().sum::<i32>())
        .collect::<Vec<i32>>();

    let max = array_sum.iter().max().unwrap();
    let pos = array_sum.iter().position(|x| x == max).unwrap();

    let mut array_cloned = array_sum.clone();
    array_cloned.sort();
    let sum_top_three = array_cloned.iter().rev().take(3).sum::<i32>();
    println!("MAX: {max}\nPOS: {pos}\nTOP THREE SUM: {sum_top_three}");
}

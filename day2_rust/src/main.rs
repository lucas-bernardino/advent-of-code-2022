#[derive(Debug)]
enum Choice {
    Rock,
    Paper,
    Scissors,
    NO_OPTION,
}
impl Choice {
    fn compute_points(&self, other: &Choice) -> i32 {
        let mut points = 0;
        match self {
            Self::Rock => match other {
                Choice::Rock => points = 3,     // draw
                Choice::Paper => points = 6,    // win
                Choice::Scissors => points = 0, // lose
                Self::NO_OPTION => (),
            },
            Self::Paper => match other {
                Choice::Rock => points = 0,     // lose
                Choice::Paper => points = 3,    // draw
                Choice::Scissors => points = 6, // win
                Self::NO_OPTION => (),
            },
            Self::Scissors => match other {
                Choice::Rock => points = 6,     // win
                Choice::Paper => points = 0,    // lose
                Choice::Scissors => points = 3, // draw
                Self::NO_OPTION => (),
            },
            Self::NO_OPTION => (),
        }
        match other {
            Choice::Rock => points += 1,
            Choice::Paper => points += 2,
            Choice::Scissors => points += 3,
            Choice::NO_OPTION => (),
        };
        points
    }
}

enum Options {
    Rock,
    Paper,
    Scissors,
    Lose,
    Draw,
    Win,
    NO_OPTION,
}

impl Options {
    fn compute_points(&self, other: &Options) -> i32 {
        let mut points = 0;
        match self {
            Self::Rock => match other {
                Options::Win => points = 2 + 6,
                Options::Draw => points = 1 + 3,
                Options::Lose => points = 3 + 0,
                _ => (),
            },
            Self::Paper => match other {
                Options::Win => points = 3 + 6,
                Options::Draw => points = 2 + 3,
                Options::Lose => points = 1 + 0,
                _ => (),
            },
            Self::Scissors => match other {
                Options::Win => points = 1 + 6,
                Options::Draw => points = 3 + 3,
                Options::Lose => points = 2 + 0,
                _ => (),
            },
            _ => (),
        }
        points
    }
}

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let content = std::fs::read_to_string("input.txt").expect("Unable to find input.txt");
    let parsed_content: Vec<_> = content
        .split("\n")
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .filter(|x| x.len() > 0)
        .map(|game| {
            game.iter()
                .map(|ch| match ch {
                    &"A" | &"X" => Choice::Rock,
                    &"B" | &"Y" => Choice::Paper,
                    &"C" | &"Z" => Choice::Scissors,
                    &&_ => Choice::NO_OPTION,
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let points_per_game = parsed_content
        .iter()
        .map(|game| game.get(0).unwrap().compute_points(game.get(1).unwrap()))
        .collect::<Vec<_>>();

    let total_points = points_per_game.iter().sum::<i32>();

    println!("Points per game: \n{:?}\n", points_per_game);
    println!("Total points: \n{total_points}\n",);
}

fn part_2() {
    let content = std::fs::read_to_string("input.txt").expect("Unable to find input.txt");
    let parsed_content: Vec<_> = content
        .split("\n")
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .filter(|x| x.len() > 0)
        .map(|game| {
            game.iter()
                .map(|ch| match ch {
                    &"A" => Options::Rock,
                    &"B" => Options::Paper,
                    &"C" => Options::Scissors,
                    &"X" => Options::Lose,
                    &"Y" => Options::Draw,
                    &"Z" => Options::Win,
                    &&_ => Options::NO_OPTION,
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let points_per_game = parsed_content
        .iter()
        .map(|game| game.get(0).unwrap().compute_points(game.get(1).unwrap()))
        .collect::<Vec<_>>();

    let total_points = points_per_game.iter().sum::<i32>();

    println!("Points per game: \n{:?}\n", points_per_game);
    println!("Total points: \n{total_points}\n",);
}

use std::{fs, path::Path};

fn count_trees_from_path(hill: &Vec<Vec<char>>, path: &(usize, usize)) -> i64 {
    let mut current_pos = (0, 0);

    let mut num_trees = 0;
    let width = hill[0].len();

    while current_pos.1 < hill.len() {
        num_trees += if hill[current_pos.1][current_pos.0 % width] == '#' {
            1
        } else {
            0
        };

        current_pos.0 += path.0;
        current_pos.1 += path.1;
    }

    num_trees
}

fn main() {
    let filename = Path::new("./dat/input.dat");
    let contents = fs::read_to_string(filename).unwrap().trim().to_string();

    let lines = contents.split("\n").collect::<Vec<&str>>();
    let mut hill = Vec::new();

    for line in lines {
        hill.push(line.chars().collect::<Vec<char>>());
    }

    // Answer to part 1
    dbg!(count_trees_from_path(&hill, &(3, 1)));

    let mut prod = 1;

    for path in [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)].iter() {
        prod *= count_trees_from_path(&hill, path);
    }

    // Answer to part 2
    dbg!(prod);
}

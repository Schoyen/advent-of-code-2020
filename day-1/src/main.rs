use std::{fs, path::Path};

fn find_sum_index_pair(expenses: &Vec<i32>) -> (usize, usize) {
    let mut first = 0;
    let mut second = 0;

    for i in 0..expenses.len() {
        first = i;

        for j in (i + 1)..expenses.len() {
            second = j;

            if expenses[first] + expenses[second] == 2020 {
                return (first, second);
            }
        }
    }

    (first, second)
}

fn find_sum_index_triplet(expenses: &Vec<i32>) -> (usize, usize, usize) {
    let mut first = 0;
    let mut second = 0;
    let mut third = 0;

    for i in 0..expenses.len() {
        first = i;

        for j in (i + 1)..expenses.len() {
            second = j;

            for k in (j + 1)..expenses.len() {
                third = k;

                if expenses[first] + expenses[second] + expenses[third] == 2020 {
                    return (first, second, third);
                }
            }
        }
    }

    (first, second, third)
}

fn main() {
    let filename = Path::new("./dat/input.dat");
    let contents = fs::read_to_string(filename).unwrap().trim().to_string();

    let split = contents.split("\n");
    let mut expenses = Vec::new();

    for s in split {
        expenses.push(s.parse::<i32>().unwrap());
    }

    let foo = find_sum_index_pair(&expenses);

    // Answer to part 1
    dbg!(expenses[foo.0] * expenses[foo.1]);

    let foo = find_sum_index_triplet(&expenses);

    // Answer to part 2
    dbg!(expenses[foo.0] * expenses[foo.1] * expenses[foo.2]);
}

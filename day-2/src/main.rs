use std::{fs, path::Path};

fn split_policy(pol: &str) -> (usize, usize, char) {
    let s = pol.split(" ").collect::<Vec<&str>>();

    assert!(s[1].len() == 1);

    let pol_char = s[1].chars().nth(0).unwrap();

    let s = s[0].split("-").collect::<Vec<&str>>();

    let min_occ = s[0].parse::<usize>().unwrap();
    let max_occ = s[1].parse::<usize>().unwrap();

    (min_occ, max_occ, pol_char)
}

fn check_if_valid_pwd_part_1(pol_pwd: &str) -> bool {
    let s = pol_pwd.split(": ").collect::<Vec<&str>>();
    let pol = s[0];
    let pwd = s[1];

    let (min_occ, max_occ, pol_char) = split_policy(pol);

    let mut sum_pol_char = 0;

    for p in pwd.chars() {
        sum_pol_char += if p == pol_char { 1 } else { 0 };
    }

    sum_pol_char >= min_occ && sum_pol_char <= max_occ
}

fn check_if_valid_pwd_part_2(pol_pwd: &str) -> bool {
    let s = pol_pwd.split(": ").collect::<Vec<&str>>();
    let pol = s[0];
    let pwd = s[1];

    let (min_occ, max_occ, pol_char) = split_policy(pol);

    let pwd_chars = pwd.chars().collect::<Vec<char>>();

    (pwd_chars[min_occ - 1] == pol_char) ^ (pwd_chars[max_occ - 1] == pol_char)
}

fn main() {
    let filename = Path::new("./dat/input.dat");
    let contents = fs::read_to_string(filename).unwrap().trim().to_string();

    let lines = contents.split("\n").collect::<Vec<&str>>();

    let mut sum_valid = 0;

    for line in &lines {
        sum_valid += if check_if_valid_pwd_part_1(line) {
            1
        } else {
            0
        };
    }

    // Answer to part 1
    dbg!(sum_valid);

    let mut sum_valid = 0;

    for line in lines {
        sum_valid += if check_if_valid_pwd_part_2(line) {
            1
        } else {
            0
        };
    }

    // Answer to part 2
    dbg!(sum_valid);
}

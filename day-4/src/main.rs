use std::{collections::HashMap, fs, path::Path};

fn valid_id_1(id: &HashMap<&str, &str>) -> bool {
    if id.len() < 7 {
        return false;
    }

    if id.len() == 8 {
        return true;
    }

    if !id.contains_key(&"cid") {
        return true;
    }

    false
}

fn valid_yr(yr: &str, low: i32, high: i32) -> bool {
    if yr.chars().collect::<Vec<char>>().len() != 4 {
        return false;
    }

    let _num_yr = match yr.parse::<i32>() {
        Ok(num_yr) => return num_yr >= low && num_yr <= high,
        Err(_e) => return false,
    };
}

fn valid_hgt(hgt: &str) -> bool {
    let hgt_arr: Vec<char> = hgt.chars().collect();
    let type_start = hgt_arr.len() - 2;

    let mut unit = String::with_capacity(2);
    unit.insert(0, hgt_arr[type_start]);
    unit.insert(1, hgt_arr[type_start + 1]);

    let mut hgt = String::new();

    for i in 0..(hgt_arr.len() - 2) {
        hgt.push(hgt_arr[i]);
    }

    let hgt_val = match hgt.parse::<i32>() {
        Ok(num) => num,
        Err(_e) => return false,
    };

    if unit == "cm" {
        return hgt_val >= 150 && hgt_val <= 193;
    } else if unit == "in" {
        return hgt_val >= 59 && hgt_val <= 76;
    }

    false
}

fn valid_hcl(hcl: &str) -> bool {
    let hcl_arr: Vec<char> = hcl.chars().collect();

    if hcl_arr[0] != '#' {
        return false;
    }

    if hcl_arr.len() != 7 {
        return false;
    }

    let valid_chars = vec![
        'a', 'b', 'c', 'd', 'e', 'f', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
    ];

    for val in &hcl_arr[1..] {
        if !valid_chars.iter().any(|&x| x == *val) {
            return false;
        }
    }

    true
}

fn valid_ecl(ecl: &str) -> bool {
    let valid_ecl = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

    return valid_ecl.iter().any(|&x| x == ecl);
}

fn valid_pid(pid: &str) -> bool {
    let pid_arr: Vec<char> = pid.chars().collect();

    if pid_arr.len() != 9 {
        return false;
    }

    return pid_arr.iter().all(|&x| x.is_numeric());
}

fn valid_id_2(id: &HashMap<&str, &str>) -> bool {
    if id.len() < 7 {
        return false;
    }

    if id.len() == 7 && id.contains_key(&"cid") {
        return false;
    }

    if !valid_yr(id[&"byr"], 1920, 2002) {
        return false;
    }

    if !valid_yr(id[&"iyr"], 2010, 2020) {
        return false;
    }

    if !valid_yr(id[&"eyr"], 2020, 2030) {
        return false;
    }

    if !valid_hgt(id[&"hgt"]) {
        return false;
    }

    if !valid_hcl(id[&"hcl"]) {
        return false;
    }

    if !valid_ecl(id[&"ecl"]) {
        return false;
    }

    if !valid_pid(id[&"pid"]) {
        return false;
    }

    return true;
}

fn get_num_valid_ids(
    ids: &Vec<HashMap<&str, &str>>,
    f: &dyn Fn(&HashMap<&str, &str>) -> bool,
) -> usize {
    let mut num_valid_ids = 0;

    for id in ids {
        num_valid_ids += if f(id) { 1 } else { 0 };
    }

    num_valid_ids
}

fn main() {
    let filename = Path::new("./dat/input.dat");
    let contents = fs::read_to_string(filename).unwrap().trim().to_string();

    let entries = contents.split("\n\n").collect::<Vec<&str>>();
    let mut ids = Vec::new();

    for entry in entries {
        let mut id = HashMap::new();

        for field in entry.split_whitespace().collect::<Vec<&str>>() {
            let f_split = field.split(":").collect::<Vec<&str>>();

            id.insert(f_split[0], f_split[1]);
        }

        ids.push(id);
    }

    let num_valid_ids = get_num_valid_ids(&ids, &valid_id_1);

    // Answer to part 1
    dbg!(num_valid_ids);

    let num_valid_ids = get_num_valid_ids(&ids, &valid_id_2);

    // Answer to part 2
    dbg!(num_valid_ids);
}

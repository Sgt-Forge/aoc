use regex::Regex;

fn find_matches<'a>(line: &'a str, re: &Regex) -> Vec<&'a str> {
    let matches = re.find_iter(line)
        .map(|mat| mat.as_str())
        .collect();

    matches
}

fn multiply(eq: &str, re: &Regex) -> i32 {
   let numbers: Vec<i32> = re.find_iter(eq)
        .map(|mat| mat.as_str().parse().unwrap())
        .collect();

    numbers[0] * numbers[1]
}

fn split_on_dont(line: &str) -> Option<(String, String)> {
    println!("split_on_dont\n\n");
    let re = Regex::new(r"don't\(\)").unwrap();

    if let Some(mat) = re.find(line) {
        let (before, after) = line.split_at(mat.start());
        Some((
            before.to_string(),
            after[mat.as_str().len()..].to_string()
        ))
    } else {
        None
    }
}

fn split_on_do(line: &str) -> Option<(String, String)> { 
    println!("split_on_dont\n\n");
    let re = Regex::new(r"do\(\)").unwrap();
    
    if let Some(mat) = re.find(line) {
        let (before, after) = line.split_at(mat.start());
        Some((
            before.to_string(),
            after[mat.as_str().len()..].to_string()
        ))
    } else {
        None
    }
}

fn collect_sums (input: &str) -> i32 { 
    println!("collect_sums\n");
    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let nums_re = Regex::new(r"\d+").unwrap();
    let mut sum = 0;
    let muls = find_matches(&input, &re);
    for mul in muls {
        sum += multiply(&mul, &nums_re);
    } 

    sum
}

fn add_left_of_donts(input: &str) -> i32{
    println!("============================= add_left_of_donts =========================\n\n");
    let mut sum = 0;
    let Some((left, right)) = split_on_dont(&input) else {
        println!("no don'ts found\n\n");
        return collect_sums(&input);
    };
    println!("left:\n\n");
    println!("{left}\n\n");
    println!("right:\n\n");
    println!("{right}");

    sum += collect_sums(&left);

    let Some((_, remainder)) = split_on_do(&right) else {
        println!("no do's found\n\n");
        return sum;
    };
    println!("remainder:\n\n");
    println!("{remainder}");
    sum += add_left_of_donts(&remainder);

    sum 
}

pub fn day03_part1() {
    let input = crate::get_days_input("03").unwrap();
    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let nums_re = Regex::new(r"\d+").unwrap();
    let mut sum = 0;

    for line in input {
        let line = line.unwrap();
        let muls = find_matches(&line, &re);
        for mul in muls {
            sum += multiply(&mul, &nums_re);
        } 
    }

    println!("{sum}");
} 

pub fn day03_part2() {
    let input = crate::get_days_input("03").unwrap();
    let mut sum = 0;
    for line in input {
        println!("============================= line =========================");
        let line = line.unwrap();
        sum += add_left_of_donts(&line);
    }
    

    println!("part 2: {sum}");
}

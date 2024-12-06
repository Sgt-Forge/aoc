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
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|(d)(o)(?:n't)?\(\)").unwrap();
    let mut sum = 0;
    let mut enabled = true;
    for line in input {
        let line = line.unwrap();
        for (s, [x, y]) in re.captures_iter(&line).map(|c| c.extract()) {
            match s {
                "do()" => enabled = true,
                "don't()" => enabled = false,
                _ => sum += x.parse::<i32>().unwrap() * y.parse::<i32>().unwrap() * enabled as i32,
            }
        }
    }

    println!("p03 part 2: {}", sum);
}

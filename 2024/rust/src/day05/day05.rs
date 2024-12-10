use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use regex::Regex;

#[derive(Debug)]
struct Rule {
    left: i32,
    right: i32,
}

#[derive(Debug)]
struct RulesForPage{
    page: i32,
    comes_before: HashMap<i32, i32>,
    comes_after: HashMap<i32, i32>,
}

impl RulesForPage {
    fn new(page: i32) -> Self {
        RulesForPage {
            page,
            comes_before: HashMap::new(),
            comes_after: HashMap::new(),
        }
    }
}


pub fn day05_part1() {
    println!("==========================================================");
    println!("\t\tday05 - part 1");
    println!("==========================================================\n\n");

    let file = File::open("../../../problems/day05/input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.unwrap().to_string())
        .collect();

    let rule_re = Regex::new(r"(\d+)\|(\d+)").unwrap();
    let mut parse_rules = true;
    let mut possible_page_orders: Vec<Vec<i32>> = vec![];
    let mut rules_for_page: HashMap<i32, RulesForPage> = HashMap::new();

    for line in lines {
        if line.is_empty() {
            parse_rules = false;
            continue;
        }

        if parse_rules {
            if let Some(captures) = rule_re.captures(&line) {
                let left: i32 = captures[1].parse().unwrap();
                let right: i32 = captures[2].parse().unwrap();
                println!("line {}, left, {}, right {}", line, left, right);

                let left_rule = rules_for_page.entry(left).or_insert(
                    RulesForPage::new(left) 
                );
                left_rule.comes_before.insert(right, 1);

                let right_rule = rules_for_page.entry(right).or_insert(
                    RulesForPage::new(right) 
                );
                right_rule.comes_after.insert(left, 1);
            }
        } else {
            possible_page_orders.push(line.split(',').map(|item| item.parse().unwrap()).collect());
        }
    }
    println!("possible page orders count: {}", possible_page_orders.len());
    let mut sum = 0;
    

    for page_order in possible_page_orders {
        let mut valid_order = true;
        println!("{:?}", page_order);

        for (split_idx, num) in page_order.iter().enumerate() {
            let rules = rules_for_page.get(num).unwrap();

            for left in 1..split_idx {
                if rules.comes_before.contains_key(&page_order[left]) {
                    println!("num: {}", num);
                    println!("left: {}", &page_order[left]);
                    println!("{:?}", rules.comes_before[&page_order[left]]);
                    valid_order = false;
                    break;
                }
            }
            for right in (split_idx+1)..page_order.len() {
                if rules.comes_after.contains_key(&page_order[right]) {
                    println!("num: {}", num);
                    println!("right: {}", &page_order[right]);
                    println!("{:?}", rules.comes_after[&page_order[right]]);
                    valid_order = false;
                    break;
                }
            }

            if !valid_order {
                break;
            }
        }

        if valid_order {
            let middle = page_order.len() / 2;
            sum += page_order[middle];
        }
    }

    println!("day5 part 1: {}", sum);
}


pub fn day05_part2() {
    println!("==========================================================");
    println!("\t\tday04 - part 2");
    println!("==========================================================\n\n");
}

use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::HashMap;

pub fn day01_both(){
    let filepath = "../../../problems/day01/input.txt";
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);
    let mut nums1: Vec<i32> = vec![];
    let mut nums2: Vec<i32> = vec![];
    let mut sim1 = HashMap::new();
    let mut sim2 = HashMap::new(); 

    for line in reader.lines() {
        let line = line.unwrap(); // Extract the line, propagating any errors
        let numbers: Vec<&str> = line.trim().split_whitespace().collect();

        if numbers.len() == 2 {
            match (numbers[0].parse::<i32>(), numbers[1].parse::<i32>()) {
                (Ok(num1), Ok(num2)) => {
                    nums1.push(num1);
                    nums2.push(num2);
                    
                    let count1 = sim1.entry(num1).or_insert(0);
                    *count1 += 1;
                    let count2 = sim2.entry(num2).or_insert(0);
                    *count2 += 1;
                },
                _ => println!("error"),
            }
        } else {
            println!("line does not contain exactly 2 numbers");
        }
    }
    nums1.sort();
    nums2.sort();
    let mut sum = 0;
    let mut similarity = 0;

    for (i, loc_id) in nums1.iter().enumerate() {
        sum += (nums1[i] -  nums2[i]).abs();
        
        let count = sim2.get(loc_id).copied().unwrap_or(0);
        similarity += loc_id * count;
    }

    println!("the diff is: {}", sum);
    println!("the similarity is: {}", similarity);

}

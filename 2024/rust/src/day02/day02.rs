fn is_safe(numbers: &Vec<i32>) -> bool { 
    let mut is_initial_increasing = true;

    for (i, _) in numbers.iter().enumerate() {
        if i == 0 {
            is_initial_increasing = numbers[1] > numbers[0];
            continue;
        }

        let prev_num = numbers[i-1];
        let diff = (numbers[i] - prev_num).abs();
        let is_current_increasing = numbers[i] > prev_num;


        if diff == 0 || diff > 3 {
            return false;
        } 

        if is_current_increasing != is_initial_increasing {
            return false;
        }
    }

    true
}

fn is_safe_damp(numbers: &Vec<i32>) -> bool {
    let source = numbers.clone(); 
    
    for i in 0..source.len() {
        let mut damp = source.clone();
        damp.remove(i);
        if is_safe(&damp) {
            return true;
        }
    }

    false
}

pub fn day02_both() {
    let input = crate::get_days_input("02").unwrap();
    let reports = crate::parse_text_to_numbers(input);
    let mut safe_reports = 0; 
    let mut safe_damp_reports = 0; 
    for line in reports {
        if is_safe(&line) {
            safe_reports += 1;
        }
        if is_safe_damp(&line) {
            safe_damp_reports += 1;
        }
    }
        
    println!("The number of safe reports without the problem dampener is: {}", safe_reports);
    println!("The number of safe reports with the problem dampener is: {}", safe_damp_reports);
} 

use std::fs::File;
use std::env;
use std::io::{BufRead, BufReader};

fn build_grid() -> Vec<Vec<char>> {
    let file = File::open("../../../problems/day04/input.txt").unwrap();
    let reader = BufReader::new(file);

        let mut grid: Vec<Vec<char>> = reader
            .lines()
            .map(|line| line.unwrap().trim().chars().collect())
            .collect();

    for i in 0..grid.len() {
        grid[i].insert(0, '.');
        grid[i].insert(0, '.');
        grid[i].insert(0, '.');
        grid[i].push('.');
        grid[i].push('.');
        grid[i].push('.');
    }

    let pad: Vec<char> = vec!['.'; grid[0].len()];
    grid.insert(0, pad.clone());
    grid.insert(0, pad.clone());
    grid.insert(0, pad.clone());
    grid.push(pad.clone());
    grid.push(pad.clone());
    grid.push(pad.clone());

    grid
}

pub fn day04_part1() {
    println!("curr dir: {}", env::current_dir().unwrap().display());
    println!("==========================================================");
    println!("\t\tday04 - part 1");
    println!("==========================================================\n\n");
    let grid = build_grid();
    let mut count = 0;

    for r in 3..grid.len()-3 { 
         for c in 3..grid[r].len()-3 {
             if grid[r][c] == 'X' { 

                // up
                if grid[r-1][c] == 'M' && grid[r-2][c] == 'A' && grid[r-3][c] == 'S' {
                    count += 1;
                }
                // down
                if grid[r+1][c] == 'M' && grid[r+2][c] == 'A' && grid[r+3][c] == 'S' {
                    count += 1;
                }
                // left
                if grid[r][c-1] == 'M' && grid[r][c-2] == 'A' && grid[r][c-3] == 'S' {
                    count += 1;
                }
                // right
                if grid[r][c+1] == 'M' && grid[r][c+2] == 'A' && grid[r][c+3] == 'S' {
                    count += 1;
                }
                // diag up-left
                if grid[r-1][c-1] == 'M' && grid[r-2][c-2] == 'A' && grid[r-3][c-3] == 'S' {
                    count += 1;
                }
                // diag up right
                if grid[r-1][c+1] == 'M' && grid[r-2][c+2] == 'A' && grid[r-3][c+3] == 'S' {
                    count += 1;
                }
                // diag down right
                if grid[r+1][c+1] == 'M' && grid[r+2][c+1] == 'A' && grid[r+3][c+1] == 'S' {
                    count += 1;
                }
                // diag down left
                if grid[r+1][c-1] == 'M' && grid[r+2][c-2] == 'A' && grid[r+3][c-3] == 'S' {
                    count += 1;
                }
             }
         }
     }
    println!("day4 part1 count: {}", count);

}

pub fn day04_part2() {
    println!("==========================================================");
    println!("\t\tday04 - part 1");
    println!("==========================================================\n\n");


}

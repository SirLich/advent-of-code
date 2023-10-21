use std::fs;
use std::io::{self, BufRead, BufReader};

const INPUT_PATH : &str = "./input.txt";
const ELF_SNACK_COUNT : u32 = 3;

fn sum_snacks(input : &str) -> Vec<u32> {
    match line {
        Err(_) => {
            sums.push(sum);
            sum = 0;
        },
        Ok(num) => {
            sum = sum + num;
        }
    };
}

fn main() {
    let mut sum : u32 = 0;
    let mut sums : Vec<u32> = vec![];

    for line in fs::read_to_string(INPUT_PATH) {
        let line = line.as_str().parse::<u32>();
        
        match line {
            Err(_) => {
                sums.push(sum);
                sum = 0;
            },
            Ok(num) => {
                sum = sum + num;
            }
        };
    }

    let highest_sum : u32 = {
        sums.sort_unstable();
        sums.reverse();
        let mut ret = 0;
        for i in 0..ELF_SNACK_COUNT{
            ret = ret + sums.get(i as usize).expect("msg");
        };

        ret
    };
    
    println!("{highest_sum}")
}

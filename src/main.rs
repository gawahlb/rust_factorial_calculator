extern crate num;
use num::{BigUint, One};
use std::io;

fn main() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("Welcome to the factorial calculator!\nYou may enter any number of integers you like and I will return their factorials.\nJust type 'stop' when you want to see the results!\n");

    let user_num: Vec<BigUint> = get_input();
    let solution: Vec<BigUint> = convert_to_vec(user_num.clone());

    if solution.len() == 1 {
        for val in solution {
            println!("\nThe factorial for your number is {}.", val);
        }
    } else {
        let mut count: i16 = 0;
        let mut n: i16 = 0;

        println!("\nHere are the results from your given numbers:\n");
        for num in user_num {
            print!("{}", num,);
            for val in &solution {
                if count == n {
                    println!(": {}", val);
                    break;
                } else {
                    n += 1;
                }
            }
            n = 0;
            count += 1;
        }
    }
}

fn fac_calc(user_num: BigUint) -> BigUint {
    let one: BigUint = One::one();
    if user_num >= 1u32.into() {
        let use_num = user_num.clone();
        return user_num * fac_calc(use_num - one);
    } else {
        return 1u32.into();
    }
}

fn convert_to_vec(user_num: Vec<BigUint>) -> Vec<BigUint> {
    let mut solution_vec: Vec<BigUint> = Vec::new();
    let mut value: BigUint;
    for num in user_num {
        value = fac_calc(num);
        solution_vec.push(value);
    }

    return solution_vec;
}

fn get_input() -> Vec<BigUint> {
    let mut input = String::new();
    let mut value_vec = Vec::new();

    println!("Please enter which integer you would like to find the factorial for: ");

    while input != "stop\r\n" {
        input.clear();

        io::stdin().read_line(&mut input).unwrap();

        if input != "stop\r\n" {
            let input: BigUint = input.trim().parse().unwrap();

            value_vec.push(input);
        }
    }

    return value_vec;
}

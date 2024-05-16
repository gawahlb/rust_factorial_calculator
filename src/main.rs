extern crate num;
use num::{BigUint, One};
use std::io;

fn main() {
    println!("Welcome to the factorial calculator! You may enter any integer you like and I will return it's factorial.");

    let user_num: BigUint = get_input();
    let solution: BigUint = fac_calc(user_num);

    println!("The factorial for your number is {}.", solution);
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

fn get_input() -> BigUint {
    let mut input = String::new();

    println!("Please enter which number you would like to find the factorial for: ");

    io::stdin().read_line(&mut input).unwrap();

    let input: BigUint = input.trim().parse().unwrap();

    return input;
}

use std::io;

fn main() {
    println!("Welcome to the factorial calculator! You may enter any number you like and I will return it's factorial.");

    let user_num: i128 = get_input();
    let solution: i128 = fac_calc(user_num);

    println!("The factorial for your number is {}.", solution);
}

fn fac_calc(user_num: i128) -> i128 {
    if user_num >= 1 {
        return user_num * fac_calc(user_num - 1);
    } else {
        return 1;
    }
}

fn get_input() -> i128 {
    let mut input = String::new();

    println!("Please enter which number you would like to find the factorial for: ");

    io::stdin().read_line(&mut input).unwrap();

    let input: i128 = input.trim().parse().unwrap();

    return input;
}

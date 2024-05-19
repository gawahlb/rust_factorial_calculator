extern crate num;
use num::{BigUint, One};
use std::io;

fn main() {
    //Clear the terminal to clean up the code.
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    println!("Welcome to the factorial calculator!\nYou may enter any number of integers you like and I will return their factorials.\nJust type 'stop' when you want to see the results!\n");

    //Redirect the user to the get_input function in order to receive the numbers they wish to find the factorial of.
    let user_num: Vec<BigUint> = get_input();

    //Take the results of the user's input and convert them to a vector.
    let solution: Vec<BigUint> = convert_to_vec(user_num.clone());

    //The code will now print out the results of the users numbers, pulling from the vectors created previously.

    //Whether the user has entered only one value or many values, the verbage in the terminal will reflect that.
    if solution.len() == 1 {
        for val in solution {
            println!("\nThe factorial for your number is {}.", val);
        }
    } else {
        let mut count: i16 = 0;
        let mut n: i16 = 0;

        println!("\nHere are the results from your given numbers:\n");

        //This section of the code will ensure that when the factorials print to the terminal, the user will be able to see the number they input as well as the result listed next to their number.
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

    //This runs the program.
}

//This function is the calculator that calculates the factorial given the users selected number. It uses recursion in order to calculate. That is, it calls itself within itself.
fn fac_calc(user_num: BigUint) -> BigUint {
    //Since numbers of BigUint type cannot be calculated with integers, we need to create a BigUint integer.
    let one: BigUint = One::one();
    if user_num >= 1u32.into() {
        let use_num = user_num.clone();
        return user_num * fac_calc(use_num - one);
    } else {
        return 1u32.into();
    }
}

//This function takes the numbers the user input and calls the fac_calc function, then stores the results in a Vec.
fn convert_to_vec(user_num: Vec<BigUint>) -> Vec<BigUint> {
    let mut solution_vec: Vec<BigUint> = Vec::new();
    let mut value: BigUint;
    for num in user_num {
        //Calling the fac_calc function to calculate the factorials given the users numbers.
        value = fac_calc(num);

        //Adding the calculated numbers to a Vec
        solution_vec.push(value);
    }

    return solution_vec;
}

//This function retrieves the input from the user.
fn get_input() -> Vec<BigUint> {
    let mut input = String::new();
    let mut value_vec = Vec::new();

    //This is what the user will see when as they are entering their numbers.
    println!("Please enter which integer you would like to find the factorial for: ");

    //When the user types "stop," the function will continue to the next step.
    while input != "stop\r\n" {
        input.clear();

        //This reads what number the user has entered and allows it to be stored.
        io::stdin().read_line(&mut input).unwrap();

        if input != "stop\r\n" {
            //This will convert the users given numbers to BigUint type.
            let input: BigUint = input.trim().parse().unwrap();

            //This will take the users given values and store them in a vector.
            value_vec.push(input);
        }
    }

    return value_vec;
}

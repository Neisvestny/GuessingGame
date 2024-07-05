use rand::Rng;
use std::io;

fn main() {
    println!("=== УГАДАЙ ЧИСЛО ===");

    let rand_number = rand::thread_rng().gen_range(1..=100);
    println!("Рандомное число: {rand_number}");

    let mut win = false;
    while !win {
        print!("Введи своё число: ");
        let mut input_number = String::new();

        io::stdin().read_line(&mut input_number).expect("Error");
        let input_number: u32 = input_number.trim().parse().expect("Надо ввести число!");

        println!("Твоё число: {input_number}");
        if rand_number > input_number {
            println!("Я загадал число, которое БОЛЬШЕ твоего")
        } else if rand_number < input_number {
            println!("Я загадал число, которое МЕНЬШЕ твоего")
        } else {
            println!("\nТЫ ПОБЕДИЛ!!!");
            win = !win
        }
    }
}

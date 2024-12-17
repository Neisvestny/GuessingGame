use rand::Rng;
use std::io;
use std::io::Write;

fn main() {
    println!("=== УГАДАЙ ЧИСЛО ===");
    println!("Компьютер загадал число от 1 до 100. Попробуй угадать!\n");

    let rand_number = rand::thread_rng().gen_range(1..=100);
    let mut attempts = 0;

    loop {
        let input_number = match get_user_input() {
            Some(num) => num,
            None => continue,
        };

        attempts += 1;

        if input_number < rand_number {
            println!("Моё число БОЛЬШЕ твоего.\n");
        } else if input_number > rand_number {
            println!("Моё число МЕНЬШЕ твоего.\n");
        } else {
            println!("\n🎉 ТЫ ПОБЕДИЛ!!! 🎉");
            println!("Ты угадал число за {attempts} попыток.");
            break;
        }
    }
}

fn get_user_input() -> Option<u32> {
    print!("Введи своё число: ");
    io::stdout().flush().expect("Ошибка при очистке буфера.");

    let mut input = String::new();
    if let Err(_) = io::stdin().read_line(&mut input) {
        println!("Ошибка при вводе. Попробуй снова.");
        return None;
    }

    match input.trim().parse::<u32>() {
        Ok(num) => Some(num),
        Err(_) => {
            println!("Пожалуйста, введи корректное число!");
            None
        }
    }
}

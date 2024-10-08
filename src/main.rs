use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::io::Write;
use std::time::Instant;

fn main() {
    println!("*********************************************");
    println!("Hi Elise, Welcome to daddy's maths challenge!");
    println!("*********************************************");

    // defines number range
    let range = 12;

    let mut question_number = 1;
    let total_questions = 10;

    // start timer
    let start_time = Instant::now();

    while question_number <= total_questions {
        // Prep the questions
        let lhs = rand::thread_rng().gen_range(0..=range);
        let rhs = rand::thread_rng().gen_range(0..=range);
        let product = lhs * rhs;

        println!("***********");
        println!("Question {}", question_number);
        println!("***********");
        println!();

        let mut result = false;

        while result == false {
            print!("{} x {} = ", lhs, rhs);
            io::stdout().flush().unwrap();

            let answer: u32 = loop {
                let mut answer = String::new();
                io::stdin()
                    .read_line(&mut answer)
                    .expect("Failed to read input");

                match answer.trim().parse() {
                    Ok(num) => break num,
                    Err(_) => {
                        println!("Elise, that's not a valid number!");
                        println!();
                        print!("{} x {} = ", lhs, rhs);
                        io::stdout().flush().unwrap();
                        continue;
                    }
                };
            };

            match answer.cmp(&product) {
                Ordering::Equal => {
                    println!("Well done, Elise!");
                    println!();
                    result = true;
                }
                _ => {
                    println!("Nope, try again.");
                    println!();
                }
            };
        }

        question_number += 1;
    }

    let elapsed_time = start_time.elapsed();
    let elapsed_secs = elapsed_time.as_secs();

    if elapsed_secs >= 60 {
        let minutes = elapsed_secs / 60;
        let seconds = elapsed_secs % 60;

        if minutes == 1 {
            println!(
                "Great work, Elise! You've completed {} challenges in {} minute {} seconds!",
                total_questions, minutes, seconds
            );
        } else {
            println!(
                "Great work, Elise! You've completed {} challenges in {} minutes {} seconds!",
                total_questions, minutes, seconds
            );
        }
    } else {
        println!(
            "Great work, Elise! You've completed {} challenges in {} seconds!",
            total_questions, elapsed_secs
        );
    }
}

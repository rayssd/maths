use colored::Colorize;
use rand::Rng;
use rand_distr::{Distribution, Uniform};
use std::cmp::Ordering;
use std::io;
use std::io::Write;
use std::time::Instant;

struct Question {
    lhs: i32,
    rhs: i32,
    result: i32,
    question_string: String,
}

fn addition(range: i32) -> Question {
    let mut my_question = Question {
        lhs: rand::thread_rng().gen_range(10..=range),
        rhs: rand::thread_rng().gen_range(0..=range),
        result: 0,
        question_string: String::new(),
    };

    my_question.result = my_question.lhs + my_question.rhs;
    my_question.question_string = format!("{} + {} = ", my_question.lhs, my_question.rhs);

    my_question
}

fn subtraction(range: i32) -> Question {
    let mut my_question = Question {
        lhs: rand::thread_rng().gen_range(10..=range),
        rhs: rand::thread_rng().gen_range(0..=range),
        result: 0,
        question_string: String::new(),
    };

    if my_question.rhs > my_question.lhs {
        std::mem::swap(&mut my_question.lhs, &mut my_question.rhs);
    }

    my_question.result = my_question.lhs - my_question.rhs;
    my_question.question_string = format!("{} - {} = ", my_question.lhs, my_question.rhs);

    my_question
}

fn multiplication(range: i32) -> Question {
    let mut my_question = Question {
        lhs: rand::thread_rng().gen_range(0..=range),
        rhs: rand::thread_rng().gen_range(2..=range),
        result: 0,
        question_string: String::new(),
    };

    my_question.result = my_question.lhs * my_question.rhs;
    my_question.question_string = format!("{} x {} = ", my_question.lhs, my_question.rhs);

    my_question
}

fn division(range: i32) -> Question {
    let mut my_question = Question {
        lhs: rand::thread_rng().gen_range(0..=range),
        rhs: rand::thread_rng().gen_range(2..=range),
        result: 0,
        question_string: String::new(),
    };

    my_question.result = my_question.lhs;
    my_question.lhs = my_question.lhs * my_question.rhs;
    my_question.question_string = format!("{} ÷ {} = ", my_question.lhs, my_question.rhs);

    my_question
}

fn main() {
    println!(
        "{}",
        "*********************************************".purple()
    );
    println!(
        "Hi {}, Welcome to daddy's maths challenge!",
        "Elise".bright_magenta()
    );
    println!(
        "{}",
        "*********************************************".purple()
    );

    let mut question_number = 1;
    let total_questions = 20;

    // start timer
    let start_time = Instant::now();

    while question_number <= total_questions {
        // Prep the questions
        let addition_range = 20;
        let subtraction_range = 30;
        let multiplication_range = 12;
        let division_range = 12;

        let question = match Uniform::from(0..=3).sample(&mut rand::thread_rng()) {
            0 => addition(addition_range),
            1 => subtraction(subtraction_range),
            2 => multiplication(multiplication_range),
            3 => division(division_range),
            _ => break,
        };

        println!("{}", "***********".yellow());
        println!("{} {}", "Question".blue(), question_number);
        println!("{}", "***********".yellow());
        println!();

        let mut is_answer_correct = false;

        while is_answer_correct == false {
            print!("{}", question.question_string);
            io::stdout().flush().unwrap();

            let answer: i32 = loop {
                let mut answer = String::new();
                io::stdin()
                    .read_line(&mut answer)
                    .expect("Failed to read input");

                match answer.trim().parse() {
                    Ok(num) => break num,
                    Err(_) => {
                        println!("Elise, that's not a valid number!");
                        println!();
                        print!("{}", question.question_string);
                        io::stdout().flush().unwrap();
                        continue;
                    }
                };
            };

            match answer.cmp(&question.result) {
                Ordering::Equal => {
                    println!("Well done, {}!", "Elise".bright_magenta());
                    println!();
                    is_answer_correct = true;
                }
                _ => {
                    println!("{}", "Nope, try again.".red());
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

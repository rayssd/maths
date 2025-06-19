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

impl Question {
    fn new_addition_and_subtraction(range: i32) -> Self {
        let lhs = rand::thread_rng().gen_range(10..=range);
        let rhs = rand::thread_rng().gen_range(0..=range);
        let result = 0;
        let question_string = String::new();

        Question {
            lhs,
            rhs,
            result,
            question_string,
        }
    }

    fn new_multiplication_and_division(range: i32) -> Self {
        let lhs = rand::thread_rng().gen_range(0..=range);
        let rhs = rand::thread_rng().gen_range(2..=range);
        let result = 0;
        let question_string = String::new();

        Question {
            lhs,
            rhs,
            result,
            question_string,
        }
    }
}

struct SimplePolynomial {
    n1: i32,
    n2: i32,
    n3: i32,
    n4: i32,
    result: i32,
    question_string: String,
}

impl SimplePolynomial {
    fn new(range: i32) -> Self {
        let n1 = rand::thread_rng().gen_range(1..=range);
        let n2 = rand::thread_rng().gen_range(1..=range);
        let n3 = rand::thread_rng().gen_range(1..=range);
        let n4 = rand::thread_rng().gen_range(1..=range);
        let result = 0;
        let question_string = String::new();

        SimplePolynomial {
            n1,
            n2,
            n3,
            n4,
            result,
            question_string,
        }
    }
}

fn simple_algebra(range: i32) -> SimplePolynomial {
    let mut my_question = SimplePolynomial::new(range);

    my_question.n3 = my_question.n1 + my_question.n2;
    (my_question.question_string, my_question.result) =
        match Uniform::from(0..=3).sample(&mut rand::thread_rng()) {
            0 => (
                format!("x - {} = {}\nWhat's x? ", my_question.n1, my_question.n2),
                my_question.n3,
            ),
            1 => (
                format!("{} - x = {}\nWhat's x? ", my_question.n3, my_question.n2),
                my_question.n1,
            ),
            2 => (
                format!("{} + x = {}\nWhat's x? ", my_question.n1, my_question.n3),
                my_question.n2,
            ),
            3 => (
                format!("x + {} = {}\nWhat's x? ", my_question.n2, my_question.n3),
                my_question.n1,
            ),
            _ => (String::from("Error"), 0i32),
        };

    my_question
}
fn simple_polynomial1(range: i32) -> SimplePolynomial {
    // x + {} + {} = {} + {} & {} -x + {} = {} + {} and variations
    let mut my_question = SimplePolynomial::new(range);

    my_question.result = my_question.n4 + my_question.n3 - my_question.n2 - my_question.n1;

    let seed = rand::thread_rng().gen_range(1..=5);

    match my_question.result.cmp(&0) {
        Ordering::Greater => {
            my_question.question_string = match seed {
                1 => format!(
                    "{} + x + {} = {} + {}\nWhat's x? ",
                    my_question.n1, my_question.n2, my_question.n3, my_question.n4
                ),

                2 => format!(
                    "x + {} + {} = {} + {}\nWhat's x? ",
                    my_question.n1, my_question.n2, my_question.n3, my_question.n4
                ),

                3 => format!(
                    "{} + {} + x = {} + {}\nWhat's x? ",
                    my_question.n1, my_question.n2, my_question.n3, my_question.n4
                ),

                4 => format!(
                    "{} + {} = {} + x + {}\nWhat's x? ",
                    my_question.n3, my_question.n4, my_question.n1, my_question.n2
                ),

                _ => format!(
                    "{} + {} = x + {} + {}\nWhat's x? ",
                    my_question.n3, my_question.n4, my_question.n1, my_question.n2
                ),
            };
        }

        _ => {
            my_question.question_string = match seed {
                1 => format!(
                    "{} - x + {} = {} + {}\nWhat's x? ",
                    my_question.n1, my_question.n2, my_question.n3, my_question.n4
                ),

                2 => format!(
                    "{} + {} - x = {} + {}\nWhat's x? ",
                    my_question.n1, my_question.n2, my_question.n3, my_question.n4
                ),

                3 => format!(
                    "{} + {} = {} - x + {}\nWhat's x? ",
                    my_question.n3, my_question.n4, my_question.n1, my_question.n2
                ),

                _ => format!(
                    "{} + {} = {} + {} - x\nWhat's x? ",
                    my_question.n3, my_question.n4, my_question.n1, my_question.n2
                ),
            };

            my_question.result = my_question.result.abs();
        }
    };

    my_question
}

fn simple_polynomial2(range: i32) -> SimplePolynomial {
    // x - {} - {} = {} + {} and variations

    let mut my_question = SimplePolynomial::new(range);

    my_question.result = my_question.n4 + my_question.n3 + my_question.n2 + my_question.n1;

    let seed = rand::thread_rng().gen_range(0..=1);

    my_question.question_string = match seed {
        0 => format!(
            "x - {} - {} = {} + {}\nWhat's x? ",
            my_question.n1, my_question.n2, my_question.n3, my_question.n4
        ),

        _ => format!(
            "{} + {} = x - {} - {}\nWhat's x? ",
            my_question.n3, my_question.n4, my_question.n1, my_question.n2
        ),
    };

    my_question
}

fn simple_polynomial3(range: i32) -> SimplePolynomial {
    // x - {} + {} = {} + {} and variations

    let mut my_question = SimplePolynomial::new(range);

    let max_n2 = my_question.n1 + my_question.n3 + my_question.n4;
    my_question.n2 = rand::thread_rng().gen_range(1..=max_n2);

    my_question.result = my_question.n4 + my_question.n3 - my_question.n2 + my_question.n1;

    let seed = rand::thread_rng().gen_range(0..=1);

    my_question.question_string = match seed {
        0 => format!(
            "x - {} + {} = {} + {}\nWhat's x? ",
            my_question.n1, my_question.n2, my_question.n3, my_question.n4
        ),

        _ => format!(
            "{} + {} = x - {} + {}\nWhat's x? ",
            my_question.n3, my_question.n4, my_question.n1, my_question.n2
        ),
    };

    my_question
}
fn addition(range: i32) -> Question {
    let mut my_question = Question::new_addition_and_subtraction(range);

    my_question.result = my_question.lhs + my_question.rhs;
    my_question.question_string = format!("{} + {} = ", my_question.lhs, my_question.rhs);

    my_question
}

fn subtraction(range: i32) -> Question {
    let mut my_question = Question::new_addition_and_subtraction(range);

    if my_question.rhs > my_question.lhs {
        std::mem::swap(&mut my_question.lhs, &mut my_question.rhs);
    }

    my_question.result = my_question.lhs - my_question.rhs;
    my_question.question_string = format!("{} - {} = ", my_question.lhs, my_question.rhs);

    my_question
}

fn multiplication(range: i32) -> Question {
    let mut my_question = Question::new_multiplication_and_division(range);

    my_question.result = my_question.lhs * my_question.rhs;
    my_question.question_string = format!("{} x {} = ", my_question.lhs, my_question.rhs);

    my_question
}

fn division(range: i32) -> Question {
    let mut my_question = Question::new_multiplication_and_division(range);

    my_question.result = my_question.lhs;
    my_question.lhs = my_question.lhs * my_question.rhs;
    my_question.question_string = format!("{} ÷ {} = ", my_question.lhs, my_question.rhs);

    my_question
}

fn main() {
    let name = "Peach";

    println!(
        "{}",
        "*********************************************".purple()
    );
    println!(
        "Hi {}, Welcome to daddy's maths challenge!",
        name.bright_magenta()
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
        let simple_polynomial_range = 20;
        let simple_algebra_range = 20;

        // let question = match Uniform::from(0..=3).sample(&mut rand::thread_rng()) {
        //     0 => addition(addition_range),
        //     1 => subtraction(subtraction_range),
        //     2 => multiplication(multiplication_range),
        //     3 => division(division_range),
        //     _ => break,
        // };

        let question = match Uniform::from(0..=2).sample(&mut rand::thread_rng()) {
            0 => simple_polynomial1(simple_polynomial_range),
            1 => simple_polynomial2(simple_polynomial_range),
            2 => simple_polynomial3(simple_polynomial_range),
            _ => break,
        };

        //let question = simple_algebra(simple_algebra_range);

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
                        println!("{}, that's not a valid number!", name);
                        println!();
                        print!("{}", question.question_string);
                        io::stdout().flush().unwrap();
                        continue;
                    }
                };
            };

            match answer.cmp(&question.result) {
                Ordering::Equal => {
                    println!("Well done, {}!", name.bright_magenta());
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
                "Great work, {}! You've completed {} challenges in {} minute {} seconds!",
                name, total_questions, minutes, seconds
            );
        } else {
            println!(
                "Great work, {}! You've completed {} challenges in {} minutes {} seconds!",
                name, total_questions, minutes, seconds
            );
        }
    } else {
        println!(
            "Great work, {}! You've completed {} challenges in {} seconds!",
            name, total_questions, elapsed_secs
        );
    }
}

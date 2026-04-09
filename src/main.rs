use clap::{Parser, ValueEnum};
use colored::Colorize;
use rand::seq::SliceRandom;
use rand::Rng;
use std::io::{self, Write};
use std::time::Instant;

// --- Configuration ---

#[derive(ValueEnum, Clone, Copy, Debug, PartialEq)]
enum Difficulty {
    Simple,
    Medium,
    Hard,
}

#[derive(Parser, Debug)]
#[command(author, version, about = "Daddy's Math Challenge")]
struct Args {
    #[arg(short, long, default_value = "Student")]
    name: String,

    /// Global range for all questions
    #[arg(short, long, default_value_t = 20)]
    range: i32,

    /// Number of questions
    #[arg(short, long, default_value_t = 20)]
    questions: usize,

    /// Types: addition, subtraction, multiplication, division, linear, quadratic
    #[arg(short = 't', long, value_parser, num_args = 1.., value_delimiter = ' ')]
    qtype: Vec<String>,

    /// Choose difficulty for Algebra: simple or hard
    #[arg(short, long, value_enum, default_value_t = Difficulty::Simple)]
    difficulty: Difficulty,
}

struct Question {
    text: String,
    results: Vec<i32>,
}

#[derive(Clone, Debug)]
struct Term {
    coeff: i32,
    power: u32,
}

impl Term {
    fn new(coeff: i32, power: u32) -> Self {
        Term { coeff, power }
    }

    fn to_string(&self, is_first: bool) -> String {
        if self.coeff == 0 {
            return String::new();
        }
        let abs_c = self.coeff.abs();

        let sign = if self.coeff < 0 {
            if is_first {
                "-"
            } else {
                " - "
            }
        } else {
            if is_first {
                ""
            } else {
                " + "
            }
        };

        let var = match self.power {
            0 => "".to_string(),
            1 => "x".to_string(),
            2 => "x^2".to_string(),
            p => format!("x^{}", p),
        };

        let num = if abs_c == 1 && self.power > 0 {
            "".to_string()
        } else {
            abs_c.to_string()
        };

        format!("{}{}{}", sign, num, var)
    }
}

// --- Logic Helpers ---

fn format_expression(terms: &mut Vec<Term>) -> String {
    if terms.is_empty() {
        return "0".to_string();
    }

    // Swap rule: -nx + positive constant -> positive constant - nx
    if terms.len() > 1 && terms[0].coeff < 0 && terms[0].power > 0 {
        if let Some(pos_idx) = terms.iter().position(|t| t.coeff > 0 && t.power == 0) {
            terms.swap(0, pos_idx);
        }
    }

    let mut output = String::new();
    let mut first_printed = false;

    for term in terms {
        if term.coeff == 0 {
            continue;
        }
        let s = term.to_string(!first_printed);
        if !s.is_empty() {
            output.push_str(&s);
            first_printed = true;
        }
    }

    if output.is_empty() {
        "0".to_string()
    } else {
        output
    }
}

// --- Question Factory ---

impl Question {
    fn new_arithmetic(op: &str, range: i32) -> Self {
        let mut rng = rand::thread_rng();

        match op {
            "addition" => {
                let a = rng.gen_range(1..range);
                let b = rng.gen_range(1..=(range - a));
                Question {
                    text: format!("{} + {} = ", a, b),
                    results: vec![a + b],
                }
            }
            "subtraction" => {
                let a = rng.gen_range(1..=range);
                let b = rng.gen_range(0..=a);
                Question {
                    text: format!("{} - {} = ", a, b),
                    results: vec![a - b],
                }
            }
            "multiplication" => {
                let a = rng.gen_range(1..=12);
                let b = rng.gen_range(1..=range.min(12));
                Question {
                    text: format!("{} x {} = ", a, b),
                    results: vec![a * b],
                }
            }
            _ => {
                // Division
                let b = rng.gen_range(1..=12);
                let res = rng.gen_range(1..=range.min(12));
                Question {
                    text: format!("{} ÷ {} = ", b * res, b),
                    results: vec![res],
                }
            }
        }
    }

    /*
    fn new_linear(range: i32, diff: Difficulty) -> Self {
        let mut rng = rand::thread_rng();

        // Ensure x is positive in simple mode
        let x_sol = match diff {
            Difficulty::Simple => rng.gen_range(1..=10),
            Difficulty::Hard => rng.gen_range(-10..=10),
        };

        match diff {
            Difficulty::Simple => {
                // ax +/- b = c. Result x is positive.
                let x_sol = rng.gen_range(1..=10);
                let a = rng.gen_range(2..=9);

                // Allow b to be negative (subtraction), but keep result positive
                let mut b = rng.gen_range(1..=range);
                if rng.gen_bool(0.5) {
                    // Check to ensure ax - b results in a positive number
                    if (a * x_sol) > b {
                        b = -b;
                    }
                }

                let total = a * x_sol + b;
                let mut lhs_vec = vec![Term::new(a, 1), Term::new(b, 0)];
                Question {
                    text: format!(
                        "{} = {}\nWhat's x? ",
                        format_expression(&mut lhs_vec),
                        total
                    ),
                    results: vec![x_sol],
                }
            }
            Difficulty::Hard => {
                let a = rng.gen_range(-9..=9);
                let b = rng.gen_range(-range..=range);
                let mut c = rng.gen_range(-9..=9);
                while c == a {
                    c = rng.gen_range(-9..=9);
                }
                let d = (a * x_sol) + b - (c * x_sol);

                let mut lhs_vec = vec![Term::new(a, 1), Term::new(b, 0)];
                let mut rhs_vec = vec![Term::new(c, 1), Term::new(d, 0)];

                Question {
                    text: format!(
                        "{} = {}\nWhat's x? ",
                        format_expression(&mut lhs_vec),
                        format_expression(&mut rhs_vec)
                    ),
                    results: vec![x_sol],
                }
            }
        }
    }
    */
    fn new_linear(range: i32, diff: Difficulty) -> Self {
        let mut rng = rand::thread_rng();

        // Handle Result Constraints
        let x_sol = if diff == Difficulty::Hard {
            rng.gen_range(-10..=10)
        } else {
            rng.gen_range(1..=10)
        };

        match diff {
            Difficulty::Simple => {
                // ax +/- b = c
                let a = rng.gen_range(2..=9);
                let mut b = rng.gen_range(1..=range);
                if rng.gen_bool(0.5) && (a * x_sol) > b {
                    b = -b;
                }
                let total = a * x_sol + b;
                let mut lhs = vec![Term::new(a, 1), Term::new(b, 0)];
                Question {
                    text: format!("{} = {}\nWhat's x? ", format_expression(&mut lhs), total),
                    results: vec![x_sol],
                }
            }
            _ => {
                // Medium & Hard (Polynomial variety)
                let n1 = rng.gen_range(1..=range);
                let n2 = rng.gen_range(1..=range);
                let n3 = rng.gen_range(1..=range);
                let x_coeff = if diff == Difficulty::Hard && rng.gen_bool(0.5) {
                    -1
                } else {
                    1
                };

                let seed = rng.gen_range(0..=5);
                let (mut lhs, mut rhs) = match seed {
                    0 => {
                        // x + n1 + n2 = n3 + n4
                        let n4 = (x_coeff * x_sol) + n1 + n2 - n3;
                        (
                            vec![Term::new(x_coeff, 1), Term::new(n1, 0), Term::new(n2, 0)],
                            vec![Term::new(n3, 0), Term::new(n4, 0)],
                        )
                    }
                    1 => {
                        // n1 + x + n2 = n3 + n4
                        let n4 = n1 + (x_coeff * x_sol) + n2 - n3;
                        (
                            vec![Term::new(n1, 0), Term::new(x_coeff, 1), Term::new(n2, 0)],
                            vec![Term::new(n3, 0), Term::new(n4, 0)],
                        )
                    }
                    2 => {
                        // n3 + n4 = n1 + n2 + x
                        let n4 = n1 + n2 + (x_coeff * x_sol) - n3;
                        (
                            vec![Term::new(n3, 0), Term::new(n4, 0)],
                            vec![Term::new(n1, 0), Term::new(n2, 0), Term::new(x_coeff, 1)],
                        )
                    }
                    3 => {
                        // x - n1 - n2 = n3 + n4
                        let n4 = (x_coeff * x_sol) - n1 - n2 - n3;
                        (
                            vec![Term::new(x_coeff, 1), Term::new(-n1, 0), Term::new(-n2, 0)],
                            vec![Term::new(n3, 0), Term::new(n4, 0)],
                        )
                    }
                    4 => {
                        // n1 - x + n2 = n3 + n4
                        let n4 = n1 - (x_coeff * x_sol) + n2 - n3;
                        (
                            vec![Term::new(n1, 0), Term::new(-x_coeff, 1), Term::new(n2, 0)],
                            vec![Term::new(n3, 0), Term::new(n4, 0)],
                        )
                    }
                    _ => {
                        // n3 + n4 = n1 + n2 - x
                        let n4 = n1 + n2 - (x_coeff * x_sol) - n3;
                        (
                            vec![Term::new(n3, 0), Term::new(n4, 0)],
                            vec![Term::new(n1, 0), Term::new(n2, 0), Term::new(-x_coeff, 1)],
                        )
                    }
                };
                Question {
                    text: format!(
                        "{} = {}\nWhat's x? ",
                        format_expression(&mut lhs),
                        format_expression(&mut rhs)
                    ),
                    results: vec![x_sol],
                }
            }
        }
    }

    fn new_quadratic(diff: Difficulty) -> Self {
        let mut rng = rand::thread_rng();
        let (r1, r2) = if diff == Difficulty::Hard {
            (rng.gen_range(-8..=8), rng.gen_range(-8..=8))
        } else {
            (rng.gen_range(1..=8), rng.gen_range(1..=8))
        };

        let b = -(r1 + r2);
        let c = r1 * r2;

        match diff {
            Difficulty::Simple => {
                // x^2 = c or x^2 + c = bx (no negative signs visible)
                let mut lhs = vec![Term::new(1, 2), Term::new(c, 0)];
                let mut rhs = vec![Term::new(-b, 1)];
                Question {
                    text: format!(
                        "{} = {}\nWhat is x? ",
                        format_expression(&mut lhs),
                        format_expression(&mut rhs)
                    ),
                    results: vec![r1, r2],
                }
            }
            _ => {
                // Medium & Hard
                let (mut lhs, mut rhs) = if rng.gen_bool(0.5) {
                    (
                        vec![Term::new(1, 2), Term::new(b, 1), Term::new(c, 0)],
                        vec![Term::new(0, 0)],
                    )
                } else {
                    (
                        vec![Term::new(1, 2), Term::new(c, 0)],
                        vec![Term::new(-b, 1)],
                    )
                };
                Question {
                    text: format!(
                        "{} = {}\nWhat is x? ",
                        format_expression(&mut lhs),
                        format_expression(&mut rhs)
                    ),
                    results: vec![r1, r2],
                }
            }
        }
    }
}

// --- Main ---

fn main() {
    let args = Args::parse();
    let mut selected_types = args.qtype.clone();
    if selected_types.is_empty() {
        selected_types = vec![
            "addition".into(),
            "subtraction".into(),
            "multiplication".into(),
            "division".into(),
            "linear".into(),
            "quadratic".into(),
        ];
    }

    println!(
        "{}",
        "*********************************************".purple()
    );
    println!(
        "Hi {}, Welcome to daddy's maths challenge!",
        args.name.bright_magenta()
    );
    println!(
        "Mode: {:?} Difficulty | Range: {}",
        args.difficulty, args.range
    );
    println!(
        "{}",
        "*********************************************".purple()
    );

    let mut deck: Vec<String> = (0..args.questions)
        .map(|i| selected_types[i % selected_types.len()].clone())
        .collect();
    deck.shuffle(&mut rand::thread_rng());

    let start_time = Instant::now();

    for (i, q_type) in deck.iter().enumerate() {
        let question = match q_type.as_str() {
            "addition" | "subtraction" | "multiplication" | "division" => {
                Question::new_arithmetic(q_type, args.range)
            }
            "linear" => Question::new_linear(args.range, args.difficulty),
            "quadratic" => Question::new_quadratic(args.difficulty),
            _ => Question::new_arithmetic("addition", args.range),
        };

        println!("\n{} {} ({})", "Question".blue(), i + 1, q_type.cyan());
        let mut is_correct = false;
        while !is_correct {
            print!("{}", question.text);
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read");
            if let Ok(num) = input.trim().parse::<i32>() {
                if question.results.contains(&num) {
                    println!("Well done, {}!", args.name.bright_magenta());
                    is_correct = true;
                } else {
                    println!("{}", "Nope, try again.".red());
                }
            } else {
                println!("Please enter a valid number!");
            }
        }
    }

    let elapsed = start_time.elapsed().as_secs();
    println!(
        "\nGreat work, {}! You finished {} questions in {}m {}s.",
        args.name,
        args.questions,
        elapsed / 60,
        elapsed % 60
    );
}

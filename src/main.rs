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

    /// Lower bound for all questions
    #[arg(long, default_value_t = 1)]
    min: i32,

    /// Upper bound for all questions
    #[arg(long, default_value_t = 20)]
    max: i32,

    /// Number of questions
    #[arg(short, long, default_value_t = 20)]
    questions: usize,

    /// Types: addition, subtraction, multiplication, division, linear, quadratic
    #[arg(short = 't', long, value_parser, num_args = 1.., value_delimiter = ' ')]
    qtype: Vec<String>,

    /// Choose difficulty for Algebra: simple, medium, or hard
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
    fn new_arithmetic(op: &str, min: i32, max: i32) -> Self {
        let mut rng = rand::thread_rng();

        match op {
            "addition" => {
                let a = rng.gen_range(min..=max);
                let b = rng.gen_range(min..=max);
                Question {
                    text: format!("{} + {} = ", a, b),
                    results: vec![a + b],
                }
            }
            "subtraction" => {
                let a = rng.gen_range(min..=max);
                // Ensure b doesn't exceed a (unless min goes into negatives)
                let b_min = min.min(a);
                let b = rng.gen_range(b_min..=a);
                Question {
                    text: format!("{} - {} = ", a, b),
                    results: vec![a - b],
                }
            }
            "multiplication" => {
                let a = rng.gen_range(min..=max);
                let b = rng.gen_range(min..=max);
                Question {
                    text: format!("{} x {} = ", a, b),
                    results: vec![a * b],
                }
            }
            _ => {
                // Division
                let res = rng.gen_range(min..=max);
                // Prevent division by zero
                let divisor_min = if min <= 0 { 1 } else { min };
                let divisor_max = if max < divisor_min { divisor_min } else { max };

                let b = rng.gen_range(divisor_min..=divisor_max);
                Question {
                    text: format!("{} ÷ {} = ", b * res, b),
                    results: vec![res],
                }
            }
        }
    }

    fn new_linear(min: i32, max: i32, diff: Difficulty) -> Self {
        let mut rng = rand::thread_rng();

        // 1. Pick the answer x
        let x_sol = if diff == Difficulty::Simple {
            rng.gen_range(1..=10)
        } else {
            rng.gen_range(-10..=10)
        };

        match diff {
            Difficulty::Simple => {
                // ax +/- b = c (The 2-step basics)
                let a = rng.gen_range(2..=9);
                let mut b = rng.gen_range(min..=max);
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
            Difficulty::Medium => {
                // Bracket style: m(ax + b) = cx + d
                let m = rng.gen_range(2..=5);
                let a = rng.gen_range(1..=4);

                let mut b = rng.gen_range(min..=max);
                if rng.gen_bool(0.5) {
                    b = -b;
                }

                let mut c = rng.gen_range(1..=10);
                if c == m * a {
                    c += 1; // avoid infinite/no solutions (cancelling x)
                }

                // Balance equation based on root
                let lhs_val = m * (a * x_sol + b);
                let d = lhs_val - c * x_sol;

                // Format the LHS string cleanly: "m(ax + b)"
                let a_str = match a {
                    1 => "x".to_string(),
                    _ => format!("{}x", a),
                };
                let b_str = if b < 0 {
                    format!("- {}", b.abs())
                } else if b > 0 {
                    format!("+ {}", b)
                } else {
                    "".to_string()
                };

                let lhs_str = if b == 0 {
                    format!("{}({})", m, a_str)
                } else {
                    format!("{}({} {})", m, a_str, b_str)
                };

                // Format RHS string cleanly: "cx + d"
                let c_str = match c {
                    1 => "x".to_string(),
                    0 => "".to_string(),
                    _ => format!("{}x", c),
                };

                let rhs_str = if c == 0 {
                    d.to_string()
                } else if d < 0 {
                    format!("{} - {}", c_str, d.abs())
                } else if d > 0 {
                    format!("{} + {}", c_str, d)
                } else {
                    c_str
                };

                Question {
                    text: format!("{} = {}\nWhat's x? ", lhs_str, rhs_str),
                    results: vec![x_sol],
                }
            }
            Difficulty::Hard => {
                if rng.gen_bool(0.5) {
                    // 50% chance of an Advanced Bracket style: m(ax + b) + k = cx + d
                    // Multiplier `m` can now be negative, and we add an extra loose term `k`
                    let m = if rng.gen_bool(0.5) {
                        rng.gen_range(2..=5)
                    } else {
                        rng.gen_range(-5..=-2)
                    };
                    let a = rng.gen_range(1..=4);

                    let mut b = rng.gen_range(min..=max);
                    if rng.gen_bool(0.5) {
                        b = -b;
                    }

                    let mut k = rng.gen_range(min..=max);
                    if rng.gen_bool(0.5) {
                        k = -k;
                    }

                    let mut c = rng.gen_range(-5..=5);
                    if c == m * a {
                        c += 1; // Prevent x from cancelling out
                    }

                    // Balance equation based on root
                    let lhs_val = m * (a * x_sol + b) + k;
                    let d = lhs_val - c * x_sol;

                    // Format LHS
                    let a_str = match a {
                        1 => "x".to_string(),
                        _ => format!("{}x", a),
                    };
                    let b_str = if b < 0 {
                        format!("- {}", b.abs())
                    } else if b > 0 {
                        format!("+ {}", b)
                    } else {
                        "".to_string()
                    };

                    let bracket_str = if b == 0 {
                        format!("{}({})", m, a_str)
                    } else {
                        format!("{}({} {})", m, a_str, b_str)
                    };

                    let k_str = if k < 0 {
                        format!(" - {}", k.abs())
                    } else if k > 0 {
                        format!(" + {}", k)
                    } else {
                        "".to_string()
                    };

                    let lhs_str = format!("{}{}", bracket_str, k_str);

                    // Format RHS
                    let c_str = match c {
                        1 => "x".to_string(),
                        -1 => "-x".to_string(),
                        0 => "".to_string(),
                        _ => format!("{}x", c),
                    };

                    let rhs_str = if c == 0 {
                        d.to_string()
                    } else if d < 0 {
                        format!("{} - {}", c_str, d.abs())
                    } else if d > 0 {
                        format!("{} + {}", c_str, d)
                    } else {
                        c_str
                    };

                    Question {
                        text: format!("{} = {}\nWhat's x? ", lhs_str, rhs_str),
                        results: vec![x_sol],
                    }
                } else {
                    // 50% chance of Polynomial style with ax on both sides, multiple constants
                    let mut lhs_terms = Vec::new();
                    let mut rhs_terms = Vec::new();

                    let a1 = rng.gen_range(2..=6);
                    let mut a2 = rng.gen_range(1..=5);
                    if a1 == a2 {
                        a2 += 1;
                    }

                    lhs_terms.push(Term::new(a1, 1));
                    rhs_terms.push(Term::new(a2, 1));

                    let n1 = rng.gen_range(min..=max);
                    lhs_terms.push(Term::new(n1, 0));

                    if rng.gen_bool(0.3) {
                        let n2 = rng.gen_range(min..=max);
                        lhs_terms.push(Term::new(n2, 0));
                    }

                    let m1 = rng.gen_range(min..=max);
                    rhs_terms.push(Term::new(m1, 0));

                    let lhs_val: i32 = lhs_terms
                        .iter()
                        .map(|t| {
                            if t.power == 1 {
                                t.coeff * x_sol
                            } else {
                                t.coeff
                            }
                        })
                        .sum();
                    let rhs_current_val: i32 = rhs_terms
                        .iter()
                        .map(|t| {
                            if t.power == 1 {
                                t.coeff * x_sol
                            } else {
                                t.coeff
                            }
                        })
                        .sum();

                    let d = lhs_val - rhs_current_val;
                    rhs_terms.push(Term::new(d, 0));

                    lhs_terms.shuffle(&mut rng);
                    rhs_terms.shuffle(&mut rng);

                    Question {
                        text: format!(
                            "{} = {}\nWhat's x? ",
                            format_expression(&mut lhs_terms),
                            format_expression(&mut rhs_terms)
                        ),
                        results: vec![x_sol],
                    }
                }
            }
        }
    }

    fn new_quadratic(diff: Difficulty) -> Self {
        let mut rng = rand::thread_rng();
        let (r1, r2) = if diff == Difficulty::Simple {
            (rng.gen_range(1..=8), rng.gen_range(1..=8))
        } else {
            // Both Medium & Hard will use negatives here
            (rng.gen_range(-8..=8), rng.gen_range(-8..=8))
        };

        let b = -(r1 + r2);
        let c = r1 * r2;

        match diff {
            Difficulty::Simple => {
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

    // Fallback if no specific ranges given, ensure min isn't larger than max
    let mut min = args.min;
    let mut max = args.max;
    if min > max {
        std::mem::swap(&mut min, &mut max);
    }

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
        "Mode: {:?} Difficulty | Range: {} to {}",
        args.difficulty, min, max
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
                Question::new_arithmetic(q_type, min, max)
            }
            "linear" => Question::new_linear(min, max, args.difficulty),
            "quadratic" => Question::new_quadratic(args.difficulty),
            _ => Question::new_arithmetic("addition", min, max),
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

#![allow(dead_code)]

fn if_statement() {
    println!("If statement:");
    let temp = 25;
    println!("{}", temp);

    println!("obvious... not supposed to continue.")
}

fn for_loop() {
    println!("For loop:");

    println!("exclusive range:");
    for x in 1..11 {
        println!("x = {}", x);
    }

    println!("inclusive range:");
    for x in 1..=11 {
        println!("x = {}", x);
    }

    println!("enumerate:");
    for (pos, y) in (1..11).enumerate() {
        println!("{}: {}", pos, y);
    }
}

fn match_statement() {
    println!("Match statement:");

    let c_code = 812;

    let c = match c_code {
        812 => "St. Petersburg",
        831 => "Nizhny Novgorod",
        1..=1000 => "unknown",
        _ => "invalid",
    };

    println!("The city with code {} is {}", c_code, c);
}

fn combination_lock() {
    println!("Simple combination lock:");
    use std::io::stdin;

    enum State {
        Locked,
        Failed,
        Unlocked,
    }

    let code = String::from("1234");
    let mut state = State::Locked;
    let mut entry = String::new();

    loop {
        match state {
            State::Locked => {
                let mut input = String::new();
                match stdin().read_line(&mut input) {
                    Ok(_) => {
                        entry.push_str(&input.trim_end());
                    }
                    Err(_) => continue,
                }
                if entry == code {
                    state = State::Unlocked;
                    continue;
                }

                if !code.starts_with(&entry) {
                    state = State::Failed;
                }
                continue;
            }
            State::Failed => {
                println!("FAILED");
                entry.clear();
                state = State::Locked;
                continue;
            }
            State::Unlocked => {
                println!("UNLOCKED");
                return;
            }
        }
    }
}

pub fn show_sections() {
    println!("Section3: Control flow:");
    if_statement();
    for_loop();
    match_statement();
    combination_lock();
}

fn main() {
    show_sections();
}

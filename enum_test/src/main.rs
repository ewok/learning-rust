enum Language {
    English
}

fn greet(language: Language) -> String {
    match language {
        Language::English => "Hello Rust".to_string()
    }
}

fn main() {
    println!("{}", greet(Language::English));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_worked() {
            assert_eq!(greet(Language::English), "Hello Rust".to_string())
    }
}


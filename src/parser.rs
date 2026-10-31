use std::cmp::Ordering;
use std::env;

pub fn parse_base(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut in_single = false;
    let mut in_double = false;
    let mut escaped = false;
    let mut has_token = false;

    for c in input.chars() {
        if escaped {
            current.push(c);
            has_token = true;
            escaped = false;
        } else if c == '\\' && !in_single {
            escaped = true;
        } else if c == '\'' && !in_double {
            in_single = !in_single;
            has_token = true;
        } else if c == '"' && !in_single {
            in_double = !in_double;
            has_token = true;
        } else if c.is_whitespace() && !in_single && !in_double {
            if has_token {
                tokens.push(expand_tilde(current));
                current = String::new();
                has_token = false;
            }
        } else {
            current.push(c);
            has_token = true;
        }
    }

    if has_token {
        tokens.push(expand_tilde(current));
    }

    tokens
}

fn expand_tilde(token: String) -> String {
    if token == "~" {
        env::var("HOME").unwrap_or(token)
    } else if token.starts_with("~/") {
        if let Ok(home) = env::var("HOME") {
            format!("{}{}", home, &token[1..])
        } else {
            token
        }
    } else {
        token
    }
}

pub fn parse_path(input: &str) -> bool {
    for i in input.chars() {
        match i.cmp(&'/'){
            Ordering::Equal => {return true}
            _ => {continue}

        }
    }
    false
}
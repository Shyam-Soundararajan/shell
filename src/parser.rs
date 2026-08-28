use std::cmp::Ordering;
pub fn parse_base(input: &str) -> Vec<&str> {
    input.split_whitespace().collect()
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
use std::cmp::Ordering;
// completely fucking revamp parser
// handle quotes / and escape sequences
// build char by char
// core shell features like arrow up down etc


//doesnt work! atleast not good enough - fuck it up
pub fn parse_base(input: &str) -> Vec<&str> {
    input.split_whitespace().collect()
}












//it works! dont touch!!
pub fn parse_path(input: &str) -> bool {
    for i in input.chars() {
        match i.cmp(&'/'){
            Ordering::Equal => {return true}
            _ => {continue}

        }
    }
    false
}
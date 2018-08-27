
use std::vec::Vec;

pub fn search<'a> (query: &str, text: &'a str) -> Vec<&'a str>{
    let mut result = Vec::new();
    
    for line in text.lines() {
        if line.contains(query){
            result.push(line);
        }
    }

    println!("{:?}",result);
    result
}
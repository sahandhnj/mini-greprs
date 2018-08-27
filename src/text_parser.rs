use std::vec::Vec;

pub fn search<'a>(query: &str, text: &'a str, case_insensetive: bool) -> Vec<&'a str> {
    let mut result = Vec::new();
    let mut line_number = 0;

    if case_insensetive {
        let query = query.to_lowercase();
        for line in text.lines() {
            line_number += 1;

            if line.to_lowercase().contains(&query) {
                println!("{}- {}", line_number, line);
                result.push(line);
            }
        }
    } else {
        for line in text.lines() {
            line_number += 1;

            if line.contains(query) {
                println!("{}- {}", line_number, line);
                result.push(line);
            }
        }
    }

    println!("\n{} lines found", result.len());
    result
}

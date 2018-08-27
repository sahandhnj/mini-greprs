use super::*;

#[test]
fn case_sensetive() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.";

    assert_eq!(vec!["safe, fast, productive."], text_parser::search(query, contents, false));
}

#[test]
fn case_insensetive() {
    let query = "DuCt";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.";

    assert_eq!(vec!["safe, fast, productive."], text_parser::search(query, contents, true));
}

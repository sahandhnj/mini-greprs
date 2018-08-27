pub struct Arguments<'a> {
    pub search_term: &'a String,
    pub file_path: &'a String,
}

impl<'a> Arguments<'a> {
    pub fn new(args: &Vec<String>) -> Result<Arguments, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        Ok(Arguments {
            search_term: &args[1],
            file_path: &args[2],
        })
    }
}

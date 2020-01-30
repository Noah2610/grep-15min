pub struct Args {
    pub file:     String,
    pub patterns: Vec<String>,
}

impl Args {
    pub fn new() -> Self {
        use std::env;

        let mut args = env::args();
        let _ = args.next();
        let file = args.next().expect("First argument has to be a file");
        let patterns = args.collect();

        Self { file, patterns }
    }
}

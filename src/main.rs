struct Args {
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

fn main() {
    use std::fs::File;
    use std::io::Read;
    use std::path::{Path, PathBuf};

    let args = Args::new();

    let mut file: File = File::open(PathBuf::from(&args.file))
        .expect(&format!("Unable to open file {}", &args.file));
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .expect("Failed reading file");

    let mut output: Vec<String> = Vec::new();

    for (line_number, line) in file_contents.split("\n").enumerate() {
        if args.patterns.iter().any(|pattern| line.contains(pattern)) {
            output.push(format!("{}:  {}", line_number, line));
        }
    }

    for output_line in output {
        println!("{}", output_line);
    }
}

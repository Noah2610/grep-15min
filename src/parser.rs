use crate::args::Args;

pub struct Parser {
    args: Args,
}

impl Parser {
    pub fn new(args: Args) -> Self {
        Self { args }
    }

    pub fn get_output(&self) -> String {
        self.file_content()
            .split("\n")
            .enumerate()
            .filter_map(|(line_number, line)| {
                if self
                    .args
                    .patterns
                    .iter()
                    .any(|pattern| line.contains(pattern))
                {
                    Some(format!("{}:  {}\n", line_number, line))
                } else {
                    None
                }
            })
            .collect()
    }

    fn file_content(&self) -> String {
        use std::fs::File;
        use std::io::Read;
        use std::path::PathBuf;

        let mut file: File = File::open(PathBuf::from(&self.args.file))
            .expect(&format!("Unable to open file {}", &self.args.file));
        let mut file_content = String::new();
        file.read_to_string(&mut file_content)
            .expect("Failed reading file");
        file_content
    }
}

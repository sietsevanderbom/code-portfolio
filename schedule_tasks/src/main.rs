use std::fs::File;
use std::io::Write;
use regex::Regex;
use anyhow::{bail, Context, Result};
use clap::{arg, Command};
use std::fs;
use crate::scheduler::build_scheduler;
use nom_supreme::{
    error::ErrorTree,
    final_parser::Location,
};

mod input;
mod scheduler;
mod task;

fn main() -> Result<()> {
    let matches = Command::new("schedule-tasks")
        .version("0.1.0")
        .author("Sietse van der Bom")
        .about("App to schedule tasks.")
        .arg(arg!([INPUT_FILE_NAME]).validator(extract_input_file_name).default_value("./test/example.tasks.in"))
        .after_help(r#"App to schedule tasks based on duration and dependencies.
  Outputs:
    - critical-path,
    - minimum total duration,
    - max-parallelism.

"#).get_matches();

    fn extract_input_file_name(input: &str) -> Result<String> {
        let name = Regex::new(r"^(./)?test/(?P<name>[0-9A-Za-z_-]+)\.?tasks.in$").unwrap();
        let captures = name.captures(input);

        match captures {
            Some(capture) => Ok(capture["name"].to_string()),
            None => bail!("Could not parse input filename, should have format: test/<name>.tasks.in"),
        }
    }

    fn make_output_file(output_text: &str, path: &str) -> Result<()> {
        let mut output = File::create(path)
            .with_context(|| format!("Failed to create file to write with path: {}", path))?;
        write!(output, "{}", output_text).context("Failed to write output-file")?;

        Ok(())
    }

    let input_file_name = matches.value_of("INPUT_FILE_NAME").context("Could not match cli argument")?;
    let output_file_name = "./test/".to_string() + &extract_input_file_name(input_file_name)? + ".sched.out";
    let input = fs::read_to_string(&input_file_name)
        .context("Something went wrong reading the input file")?;

    let tasks = match input::parsers::parse_job(&input) {
        Ok(tasks) => tasks,
        Err(ErrorTree::Stack { base, contexts: _ }) => {
            match *base {
                ErrorTree::Base { location, kind } => {
                    let Location { line, column } = location;
                    eprintln!("Error: row {}, column {}, kind: {}", line, column, kind);
                }
                ErrorTree::Alt(bases) => {
                    let mut bases_sorted = vec![];
                    for base in bases {
                        bases_sorted.insert(0, base);
                    }
                    for base in bases_sorted {
                        if let ErrorTree::Base { location, kind } = base {
                            let Location { line, column, } = location;
                            eprintln!("Error: row {}, column {}, kind: {}", line, column, kind);
                        }
                    }
                    std::process::exit(1)
                }
                _ => {
                    eprintln!("Error");
                    std::process::exit(1)
                }
            }
            std::process::exit(1)
        }
        Err(error) => {
            eprintln!("Error: {:#?}", error);
            std::process::exit(1)
        }
    };

    let scheduler = build_scheduler(tasks);
    let output = scheduler.run();
    eprintln!("{}", output);

    make_output_file(output.as_str(), &output_file_name)
        .with_context(|| format!("Could not make output file {}", &output_file_name))?;

    Ok(())
}

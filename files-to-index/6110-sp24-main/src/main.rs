use std::{error, process::exit};

mod parser;
mod tokenizer;
mod utils;

fn get_writer(output: &Option<std::path::PathBuf>) -> Box<dyn std::io::Write> {
    match output {
        Some(path) => Box::new(std::fs::File::create(path.as_path()).unwrap()),
        None => Box::new(std::io::stdout()),
    }
}

fn execute(
    input: &str,
    writer: &mut Box<dyn std::io::Write>,
    target: utils::cli::CompilerAction,
) -> Result<(), Box<dyn error::Error>> {
    match target {
        utils::cli::CompilerAction::Default => {
            panic!("Invalid target");
        }
        utils::cli::CompilerAction::Scan => {
            let tokens = tokenizer::tokenize(input.to_string())?;
            for token in tokens {
                writeln!(writer, "{token}").unwrap();
            }
        }
        utils::cli::CompilerAction::Parse => {
            let tokens = tokenizer::tokenize(input.to_string())?;
            let program = parser::parse_program(tokens)?;
            writeln!(writer, "{:?}", program).unwrap();
        }
        utils::cli::CompilerAction::Inter => {
            todo!("inter");
        }
        utils::cli::CompilerAction::Assembly => {
            todo!("assembly");
        }
    }
    Ok(())
}

fn main() {
    let args = utils::cli::parse();
    let input = std::fs::read_to_string(&args.input).expect("Filename is incorrect.");

    if args.debug {
        eprintln!(
            "Filename: {:?}\nDebug: {:?}\nOptimizations: {:?}\nOutput File: {:?}\nTarget: {:?}",
            args.input, args.debug, args.opt, args.output, args.target
        );
    }

    // Use writeln!(writer, "template string") to write to stdout ot file.
    let mut writer = get_writer(&args.output);
    match execute(&input, &mut writer, args.target) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("{e}");
            exit(1);
        }
    }
}

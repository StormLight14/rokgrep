use clap::Parser;
use colored::Colorize;
use std::fs;
use std::fs::metadata;
use std::io::ErrorKind;
use walkdir::WalkDir;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    path: String,
    #[arg(short, long)]
    text: String,
    #[arg(short, long, action)]
    recursive: bool,
    #[arg(short, long, action)]
    case_insensitive: bool,
}

fn main() {
    let args = Args::parse();
    let path_metadata = metadata(&args.path).unwrap();

    if path_metadata.is_file() {
        if let Ok(contents) = fs::read_to_string(&args.path) {
            for line in contents.split("\n") {
                if line.contains(args.text.as_str()) && !args.case_insensitive {
                    println!("{}", line);
                } else if line
                    .to_ascii_lowercase()
                    .contains(args.text.to_ascii_lowercase().as_str())
                    && args.case_insensitive
                {
                    println!("{}", line)
                }
            }
        }
    } else if path_metadata.is_dir() {
        if args.recursive {
            for entry in WalkDir::new(&args.path) {
                if let Ok(entry) = entry {
                    if entry.metadata().unwrap().is_file() {
                        check_contents(
                            entry.path().to_str().unwrap(),
                            &args.text,
                            args.case_insensitive,
                        );
                    }
                }
            }
        } else {
            if let Ok(entries) = fs::read_dir(&args.path) {
                for entry in entries {
                    if let Ok(entry) = entry {
                        if entry.metadata().unwrap().is_file() {
                            check_contents(
                                entry.path().to_str().unwrap(),
                                &args.text,
                                args.case_insensitive,
                            );
                        }
                    }
                }
            }
        }
    }
}

fn check_contents(path: &str, text: &str, case_insensitive: bool) {
    let text = match case_insensitive {
        true => text.to_ascii_lowercase(),
        false => text.to_string(),
    };

    if let Ok(contents) = fs::read_to_string(path) {
        if contents.contains(text.as_str()) {
            println!("{}:", path);
        }
        for line in contents.split("\n") {
            if line.contains(text.as_str()) {
                println!(
                    "{}",
                    line.replace(text.as_str(), text.red().to_string().as_str())
                );
            }
        }
    }
}

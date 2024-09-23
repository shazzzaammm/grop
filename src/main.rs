use anyhow::{Context, Ok, Result};
use colored::Colorize;

struct Cli {
    pattern: String,
    path: std::path::PathBuf,
    params: String,
}

fn main() -> Result<()> {
    let search_pattern = std::env::args().nth(1).expect("No Pattern Given");
    let file_path = std::env::args().nth(2).expect("No Path Given");
    let params = match std::env::args().nth(3) {
        Some(p) => p,
        None => String::new(),
    };

    let args = Cli {
        pattern: search_pattern,
        path: std::path::PathBuf::from(file_path),
        params: params,
    };

    if args.params.contains("r") {
        recursive_search(&args.pattern, &args.path).unwrap();
    } else {
        if args.path.is_file() {
            match search_file(&args.pattern, &args.path) {
                Err(e) => println!("{}", e),
                _ => (),
            }
        } else {
            println!("No such file {}", args.path.display());
        }
    }

    Ok(())
}

fn search_file(pattern: &str, path: &std::path::PathBuf) -> Result<()> {
    let contents = std::fs::read_to_string(path)
        .with_context(|| format!("could not read file `{}`", path.display()))?;

    for line in contents.lines() {
        if line.contains(pattern) {
            print!(
                "{}{}",
                path.display().to_string().bright_magenta(),
                ": ".bright_cyan()
            );
            println!(
                "{}",
                line.replace(pattern, &pattern.bright_red().bold().to_string())
            );
        }
    }
    Ok(())
}

fn recursive_search(pattern: &str, path: &std::path::PathBuf) -> Result<()> {
    if path.is_file() {
        match search_file(pattern, path) {
            // Err(e) => eprintln!("{}", e),
            _ => (),
        }
    } else if path.is_dir() {
        for file in std::fs::read_dir(path).unwrap() {
            let file_path = file.unwrap().path();
            match recursive_search(&pattern, &file_path) {
                // Err(e) => eprintln!("{}", e),
                _ => (),
            }
        }
    }
    Ok(())
}

use anyhow::{Context, Ok, Result};
use colored::Colorize;

struct Cli {
    pattern: String,
    path: std::path::PathBuf,
    params: String,
}

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);

    let mut pattern: String = String::new();
    let mut file_path: Option<std::path::PathBuf> = None;
    let mut params: String = String::new();

    while let Some(arg) = args.next() {
        match &arg[..] {
            "-h" | "--help" => params += "help ",
            "-r" | "--recursive" => params += "recursive ",
            "-i" | "--ignore-case" => params += "insensitive ",
            "-x" | "--regex" => params += "regex ",

            _ => {
                if arg.starts_with("-") {
                    println!("Unknown argument '{}'", arg);
                } else if pattern == "" {
                    pattern = arg;
                } else if file_path.is_none() {
                    let path_arg = std::path::PathBuf::from(arg);
                    if path_arg.exists() {
                        file_path = Some(path_arg);
                    } else {
                        println!("'{}' no such file or directory", path_arg.display());
                    }
                }
            }
        }
    }

    if params.contains("help") {
        println!("Usage: grop [PATTERN] [FILE] [PARAMS]");
        println!("Search for PATTERN in each FILE");
        println!("Example: grop \"Hello World\" src/main.rs -i\n");
        println!("-h, --help\t\t Show the help screen");
        println!("-r, --recursive\t\tRecursively search FILE as a directory");
        println!("-i, --ignore-case\t\tMatch pattern in any case");
        println!("-x, --regex\t\tInterpret pattern as a regular expression");
        return Ok(());
    }

    if params.contains("regex") {
        println!("Regex not implemented yet");
        return Ok(());
    }

    let file_path = file_path.get_or_insert(std::path::PathBuf::from("."));

    let args = Cli {
        pattern: pattern,
        path: file_path.to_owned(),
        params: params,
    };

    if args.params.contains("recursive") {
        recursive_search(&args).unwrap();
    } else {
        if args.path.is_file() {
            match search_file(&args) {
                Err(e) => println!("{}", e),
                _ => (),
            }
        } else if args.path.is_dir() {
            println!("'{}' is a directory", args.path.display());
        } else {
            println!("no such file {}", args.path.display());
        }
    }

    Ok(())
}

fn search_file(args: &Cli) -> Result<()> {
    let contents = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    for line in contents.lines() {
        if args.params.contains("insensitive") {
            if line.to_lowercase().contains(&args.pattern.to_lowercase()) {
                print!(
                    "{}{}",
                    args.path.display().to_string().bright_magenta(),
                    ": ".bright_cyan()
                );
                println!(
                    "{}",
                    line.replace(
                        &args.pattern.to_lowercase(),
                        &args.pattern.to_lowercase().bright_red().bold().to_string()
                    )
                    .replace(
                        &args.pattern.to_uppercase(),
                        &args.pattern.to_uppercase().bright_red().bold().to_string()
                    )
                );
            }
        } else {
            if line.contains(&args.pattern) {
                print!(
                    "{}{}",
                    args.path.display().to_string().bright_magenta(),
                    ": ".bright_cyan()
                );
                println!(
                    "{}",
                    line.replace(&args.pattern, &args.pattern.bright_red().bold().to_string())
                );
            }
        }
    }
    Ok(())
}

fn recursive_search(args: &Cli) -> Result<()> {
    if args.path.is_file() {
        match search_file(args) {
            // Err(e) => eprintln!("{}", e),
            _ => (),
        }
    } else if args.path.is_dir() {
        for file in std::fs::read_dir(&args.path).unwrap() {
            let file_path = file.unwrap().path();
            let new_args = Cli {
                pattern: args.pattern.to_owned(),
                path: file_path,
                params: args.params.to_owned(),
            };

            match recursive_search(&new_args) {
                // Err(e) => eprintln!("{}", e),
                _ => (),
            }
        }
    }
    Ok(())
}

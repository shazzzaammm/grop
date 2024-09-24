// Imports
use anyhow::{Context, Ok, Result};
use colored::Colorize;

/// The settings for the command input by the user
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
    params: String,
}

fn main() -> Result<()> {
    // Get the inputs from the user (skip the word "grop")
    let mut args = std::env::args().skip(1);

    // Create variables to store params
    let mut pattern: String = String::new();
    let mut file_path: Option<std::path::PathBuf> = None;
    let mut params: String = String::new();

    // Loop through the user inputs
    while let Some(arg) = args.next() {
        match &arg[..] {
            // Implemented options for command
            // TODO verbose, 
            "-h" | "--help" => params += "help ",
            "-r" | "--recursive" => params += "recursive ",
            "-i" | "--ignore-case" => params += "insensitive ",
            "-x" | "--regex" => params += "regex ",

            _ => {
                // Error handling
                if arg.starts_with("-") {
                    println!("Unknown argument '{}'", arg);
                } 

                // Set the pattern if it doesnt exist
                else if pattern == "" {
                    pattern = arg;
                } 
                
                // Set directory if it exists
                else if file_path.is_none() {
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

    // Print help if requested
    if params.contains("help") {
        print_help();
        return Ok(());
    }

    // TODO implement regex
    if params.contains("regex") {
        println!("Regex not implemented yet");
        return Ok(());
    }

    // Set file path to this directory if no file path is provided
    let file_path = file_path.get_or_insert(std::path::PathBuf::from("."));

    // Store up the arguements for the command
    let args = Cli {
        pattern: pattern,
        path: file_path.to_owned(),
        params: params,
    };

    // Recursive search if requested
    if args.params.contains("recursive") {
        recursive_search(&args).unwrap();
    } 
    else {
        // Search the file for the pattern if it exists
        if args.path.is_file() {
            match search_file(&args) {
                Err(e) => println!("{}", e),
                _ => (),
            }
        }
        // Error due to it being a directory 
        else if args.path.is_dir() {
            println!("'{}' is a directory", args.path.display());
        }
        // Error due to the path not existing 
        else {
            println!("no such file {}", args.path.display());
        }
    }
    // Success!
    Ok(())
}

/// Function to search a file for all instances of a pattern and highlight them 
fn search_file(args: &Cli) -> Result<()> {
    // Read the file
    let contents = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    for line in contents.lines() {
        // Search for upper or lower case if requested
        if args.params.contains("insensitive") {
            if line.to_lowercase().contains(&args.pattern.to_lowercase()) {
                // Print file path
                print!(
                    "{}{}",
                    args.path.display().to_string().bright_magenta(),
                    ": ".bright_cyan()
                );

                // Highlight instances
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
        }
        // Search case sensitively 
        else {
            if line.contains(&args.pattern) {
                // Print file path
                print!(
                    "{}{}",
                    args.path.display().to_string().bright_magenta(),
                    ": ".bright_cyan()
                );
                // Highlight instances
                println!(
                    "{}",
                    line.replace(&args.pattern, &args.pattern.bright_red().bold().to_string())
                );
            }
        }
    }
    // Success!
    Ok(())
}

/// Print the help text (grop -h)
fn print_help() {
    println!("Usage: grop [PATTERN] [FILE] [PARAMS]");
    println!("Search for PATTERN in each FILE");
    println!("Example: grop \"Hello World\" src/main.rs -i\n");
    println!("-h, --help\t\t Show the help screen");
    println!("-r, --recursive\t\tRecursively search FILE as a directory");
    println!("-i, --ignore-case\t\tMatch pattern in any case");
    println!("-x, --regex\t\tInterpret pattern as a regular expression");
}

/// Recursively search directories for instances
fn recursive_search(args: &Cli) -> Result<()> {
    // Search the file 
    if args.path.is_file() {
        match search_file(args) {
            // Err(e) => eprintln!("{}", e),
            _ => (),
        }
    } else if args.path.is_dir() {
        // Loop through all elements of directory
        for file in std::fs::read_dir(&args.path).unwrap() {
            // Update the arguements
            let file_path = file.unwrap().path();
            let new_args = Cli {
                pattern: args.pattern.to_owned(),
                path: file_path,
                params: args.params.to_owned(),
            };

            // Recursion!!!!
            match recursive_search(&new_args) {
                // Err(e) => eprintln!("{}", e),
                _ => (),
            }
        }
    }
    Ok(())
}

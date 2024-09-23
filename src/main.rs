use anyhow::{Context, Ok, Result};
use colored::Colorize;

struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let search_pattern = std::env::args().nth(1).expect("No Pattern Given");
    let file_path = std::env::args().nth(2).expect("No Path Given");

    let args = Cli {
        pattern: search_pattern,
        path: std::path::PathBuf::from(file_path),
    };

    let contents = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    for line in contents.lines() {
        if line.contains(&args.pattern) {
            println!(
                "{}",
                line.replace(
                    &args.pattern,
                    &args.pattern.bright_red().bold().to_string()
                )
            );
        }
    }

    Ok(())
}
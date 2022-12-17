use clap::{App, Arg};
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    // Parse command-line arguments
    let matches = App::new("file-renamer")
        .arg(Arg::with_name("directory").required(true))
        .arg(Arg::with_name("rule").required(true))
        .get_matches();

    // Read directory and rename files
    let directory = matches.value_of("directory").unwrap();
    let rule = matches.value_of("rule").unwrap();
    for entry in fs::read_dir(directory)? {
        let old_path = entry?.path();
        let old_name = old_path.file_name().unwrap().to_str().unwrap();
        let (name, extension) =
            old_name.split_at(old_name.rfind('.').unwrap_or_else(|| old_name.len()));
        let new_name = match rule {
            "uppercase" => name.to_uppercase(),
            "lowercase" => name.to_lowercase(),
            "first-letter-uppercase" => {
                let mut chars = name.chars();
                let first_char = chars.next().unwrap();
                let rest_of_name: String = chars.collect();
                format!("{}{}", first_char.to_uppercase(), rest_of_name)
            }
            "first-letter-lowercase" => {
                let mut chars = name.chars();
                let first_char = chars.next().unwrap();
                let rest_of_name: String = chars.collect();
                format!("{}{}", first_char.to_lowercase(), rest_of_name)
            }
            _ => name.to_owned(),
        };
        let new_path = old_path.with_file_name(format!("{}{}", new_name, extension));
        fs::rename(old_path, new_path)?;
    }

    Ok(())
}


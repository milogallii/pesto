use regex::escape;
use std::env;
use std::process::{Command, Stdio};

fn main() {
    let args: Vec<String> = env::args().collect();

    // Check if we have at least one argument (the delimiter)
    if args.len() < 2 {
        eprintln!("usage: pss DELIM [PATH...]");
        std::process::exit(1);
    }

    let delim = &args[1];
    let escaped_delim = escape(delim);

    // Determine paths to search
    let paths: Vec<&str> = if args.len() > 2 {
        args[2..].iter().map(|s| s.as_str()).collect()
    } else {
        vec!["."]
    };

    let pattern = format!("(?s):{}\\K.*?(?={}:)", escaped_delim, escaped_delim);
    let pesto_files = collect_pesto_files();

    let mut rg_cmd = Command::new("rg");
    rg_cmd
        .arg("--color=always")
        .arg("-nH")
        .arg("-UoP")
        .arg("-g")
        .arg("!*.pesto")
        .arg(&pattern)
        .args(&paths);

    if !pesto_files.is_empty() {
        rg_cmd.args(&pesto_files);
    }

    let status = rg_cmd
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .expect("Failed to execute ripgrep");

    std::process::exit(status.code().unwrap_or(1));
}

fn collect_pesto_files() -> Vec<String> {
    let mut pesto_files = Vec::new();

    if let Ok(entries) = std::fs::read_dir(".") {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(ext) = path.extension() {
                if ext == "pesto" {
                    if let Some(name) = path.file_name() {
                        if let Some(name_str) = name.to_str() {
                            pesto_files.push(name_str.to_string());
                        }
                    }
                }
            }
        }
    }

    pesto_files
}

use std::env::{args, var};
use std::fmt;
use std::fs::read_to_string;
use std::io;
use std::process::Command;

use serde_derive::Deserialize;

#[derive(Deserialize)]
struct Cargo {
    package: Package,
}

#[derive(Deserialize)]
struct Package {
    name: String,
    description: Option<String>,
}

impl fmt::Display for Package {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, " {}", self.name)
    }
}

fn main() -> io::Result<()> {
    // Parse Cargo.toml
    let cargo_toml = read_to_string("Cargo.toml")?;
    let val: Cargo = toml::from_str(&cargo_toml)?;

    // Get max len if any
    let max_len = args().nth(1).map(|arg| arg.parse::<usize>().unwrap_or(100));

    // Create the new pane name
    let mut proj_desc = val.package.to_string();
    match max_len {
        Some(ml) if proj_desc.len() > ml => {
            proj_desc.truncate(ml - 3);
            proj_desc.push_str(" ..");
        }
        _ => {}
    }

    // Rename the tmux pane
    Command::new("tmux")
        .arg("renamew")
        .arg(proj_desc)
        .output()?;

    Ok(())
}

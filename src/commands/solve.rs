use std::process::{Command, Stdio};

use super::Day;

pub fn handle(year: u16, day: Day, release: bool) {
    let mut args = vec![
        "run".to_string(),
        "--bin".to_string(),
        format!("{}-{}", year, day),
    ];

    if release {
        args.push("--release".to_string());
    }

    let mut command = Command::new("cargo")
        .args(&args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

    command.wait().unwrap();
}

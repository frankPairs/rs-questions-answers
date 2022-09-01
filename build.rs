use platforms::*;
use std::{borrow::Cow, process::Command};

/// Generate the `cargo` key output based on the git commit sha
pub fn generate_cargo_keys() {
    let output_result = Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output();

    let commit = match output_result {
        Ok(output) if output.status.success() => {
            let sha = String::from_utf8_lossy(&output.stdout).trim().to_owned();

            Cow::from(sha)
        }
        Ok(output) => {
            println!(
                "cargo:warning=Git command failed with status: {}",
                output.status
            );

            Cow::from("unknown")
        }
        Err(err) => {
            println!("cargo:warning=Failed to execute git command: {}", err);

            Cow::from("unknown")
        }
    };

    println!(
        "cargo:rustc-env=RUST_WEB_DEV_VERSION={}",
        get_version(&commit)
    );
}

fn get_platform() -> String {
    let env_dash = if TARGET_ENV.is_some() { "-" } else { "" };

    format!(
        "{}-{}{}{}",
        TARGET_ARCH.as_str(),
        TARGET_OS.as_str(),
        env_dash,
        TARGET_ENV.map(|x| x.as_str()).unwrap_or("")
    )
}

fn get_version(commit: &str) -> String {
    let commit_dash = if commit.is_empty() { "" } else { "-" };

    format!(
        "{}{}{}-{}",
        std::env::var("CARGO_PKG_VERSION").unwrap_or_default(),
        commit_dash,
        commit,
        get_platform()
    )
}

fn main() {
    generate_cargo_keys();
}

use std::{env, process::Command};

fn main() {
    let version_regex = regex::bytes::Regex::new(r"cargo (?P<version>[^ ]+) \(").unwrap();

    let cargo = env::var_os("CARGO").expect("`CARGO` gets set automatically");

    let version = Command::new(cargo)
        .arg("--version")
        .output()
        .expect("cannot execute cargo");
    if let Some(regex_match) = version_regex.captures(&version.stdout) {
        let version = regex_match
            .name("version")
            .expect("The name is used in the regex")
            .as_bytes();
        let version = String::from_utf8_lossy(&version);
        println!("cargo::rustc-env=RUST_COMPILER_VERSION={version}");
    } else {
        panic!("Could not extract the version info from the `cargo --version` output");
    }
}

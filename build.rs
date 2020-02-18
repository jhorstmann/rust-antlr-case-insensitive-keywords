use std::process::Command;

fn main() {
    Command::new("java")
        .arg("-jar")
        .arg("antlr4-4.8-2-SNAPSHOT-complete.jar")
        .arg("-Dlanguage=Rust")
        .arg("-no-listener")
        .arg("-no-visitor")
        .arg("-o")
        .arg("src/gen")
        .arg("Query.g4")
        .spawn()
        .expect("antlr tool failed to start");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Query.g4");
    println!("cargo:rerun-if-changed=antlr4-4.8-2-SNAPSHOT-complete.jar");
}

use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("./Cargo.toml")
        .expect("Failed to open Cargo.toml!");
    writeln!(file, "[profile.release]\ncodegen-units = 1\nlto = \"fat\"\nopt-level = \"z\"\npanic = \"abort\"\nstrip = \"symbols\"\nincremental = false\ndebug = false").expect("Failed to write to file! (Is it open)");
    println!("Done!");
}

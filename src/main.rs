use std::process::Command;

use totaluptime::{compute_total_uptime, ColoredUnwrap};

fn main() {
    let result = Command::new("last")
        .arg("reboot")
        .output()
        .colored_unwrap("🛑 Failed to query the total uptime of your machine");
    let input = String::from_utf8(result.stdout)
        .colored_unwrap("🛑 Failed to read the total uptime of your machine");

    let total_uptime = compute_total_uptime(&input);
    println!("{total_uptime}")
}

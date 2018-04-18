extern crate python_challenge;

use python_challenge::solve_task;

fn main() {
    let mut args = std::env::args();
    match solve_task(
        args.nth(1)
            .unwrap_or_else(|| String::from("missing argument"))
            .as_ref(),
    ) {
        Ok(()) => (),
        Err(err) => {
            eprintln!("Error: '{}'", err);
            std::process::exit(1)
        }
    }
}

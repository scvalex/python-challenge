fn main() {
    let mut args = std::env::args();
    match args.nth(1)
        .unwrap_or(String::from("missing argument"))
        .as_ref()
    {
        "0" => println!("{}", (2 as i64).pow(38)),
        s => {
            eprintln!("unknown task '{}'", s);
            std::process::exit(1)
        }
    }
}

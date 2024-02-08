use clap::Arg;

fn main() -> std::io::Result<()> {
    let matches = clap::command!()
        .arg(Arg::new("n")
            .required(true)
            .value_name("NUM")
            .value_parser(clap::value_parser!(i128))
            .help("Specify the decimal number to convert")
        )
        .arg(Arg::new("dummy")
            .long("dummy")
            .required(false)
            .conflicts_with("n")
            .action(clap::ArgAction::SetTrue)
            .help("Run the dummy main")
        )
        .get_matches();

    if matches.get_flag("dummy") {
        binary_conversions::main();
        return Ok(());
    }

    let n: i128 = *matches.get_one("n").unwrap();

    binary_conversions::run(n);

    Ok(())
}

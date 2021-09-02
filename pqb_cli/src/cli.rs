use clap::{App, AppSettings, Arg, ArgSettings};

pub fn build_cli() -> App<'static, 'static> {
    let input = Arg::with_name("input")
        // .set(ArgSettings::Required)
        .short("i")
        .long("input")
        .help("Path to models file.")
        .global(true)
        .takes_value(true);

    let output = Arg::with_name("output")
        .short("o")
        .long("output")
        .help("Specifies output path for rust file.")
        .global(true)
        .takes_value(true);

    App::new("pqb_cli")
        .version(env!("CARGO_PKG_VERSION"))
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(input)
        .arg(output)
}

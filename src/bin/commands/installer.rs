use hello_world::util::*;
use hello_world::ArgMatchesExt;

pub fn cli() -> Command {
    subcommand("install")
        .about("Insatll Rust componenet added in `distribution.toml`.")
        .arg_target_dir()
        .after_help("Run `cargo-dist dist --help` for more detailed information. \n")
}

pub fn exec(args: &ArgMatches) {
    let output_dir = args.output_dir();
    
}


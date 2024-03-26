use hello_world::util::*;

pub fn cli() -> Command {
    subcommand("package")
        .about("Build locally and package as component as rustup can identify.")
        .arg_release("Build artifacts in release mode, with optimizations")
        .arg_target_triple("Build for the target triple")
        .arg_target_dir()
        .after_help("Run `cargo-dist dist --help` for more detailed information. \n")
}

pub fn exec(_args: &ArgMatches) {

}


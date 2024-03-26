use hello_world::util::*;

pub fn cli() -> Command {
    subcommand("dist")
        .about("Distribution package manager")
        .arg_release("Build artifacts in release mode, with optimizations")
        .arg_target_triple("Build for the target triple")
        .arg_target_dir()
        .after_help("Run `cargo-dist dist --help` for more detailed information. \n")
}

pub fn exec(args: &ArgMatches) {

}


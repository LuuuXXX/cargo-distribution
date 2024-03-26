pub mod installer;
pub mod package;

use hello_world::util::*;

pub fn builtin() -> Vec<Command> {
    vec![
        installer::cli(),
        package::cli(),
    ]
}

pub type Exec = fn(&ArgMatches);

pub fn builtin_exec(cmd: &str) -> Option<Exec> {
    let f = match cmd {
        "install" => installer::exec,
        "package" => package::exec,
        _ => panic!("Failed to identify command"),
    };
    Some(f)
}
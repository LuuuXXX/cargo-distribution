use clap::ArgMatches;

pub mod core;
pub mod util;

pub trait ArgMatchesExt {
    fn target_triple(&self) -> Option<&str> {
        self._value_of("target")
    }

    fn output_dir(&self) -> Option<&str> {
        self._value_of("target-dir")
    }

    fn manifest_dir(&self) -> Option<&str> {
        self._value_of("manifest-path")
    }

    fn is_release(&self) -> bool {
        if !self.flag("release") {
            return false;
        }
        true
    }

    fn flag(&self, name: &str) -> bool;

    fn _value_of(&self, name: &str) -> Option<&str>;
}

impl<'a> ArgMatchesExt for ArgMatches {
    fn flag(&self, name: &str) -> bool {
        ignore_unknown(self.try_get_one::<bool>(name))
            .copied()
            .unwrap_or(false)
    }

    fn _value_of(&self, name: &str) -> Option<&str> {
        ignore_unknown(self.try_get_one::<String>(name))
            .map(String::as_str)
    }
}

#[track_caller]
pub fn ignore_unknown<T: Default>(r: Result<T, clap::parser::MatchesError>) -> T {
    match r {
        Ok(t) => t,
        Err(clap::parser::MatchesError::UnknownArgument { .. }) => Default::default(),
        Err(e) => {
            panic!("Mismatch between definition and access: {}", e);
        }
    }
}
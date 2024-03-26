use std::fmt::Error;
use std::process::{Command, Stdio};

struct Curl {
    cmd: Command,
    host: String,
}

impl Curl {
    fn new(host: &str) -> Curl {
        let mut cmd = Command::new("curl");
        cmd.arg("-sSf");
        Curl {
            cmd,
            host: host.to_string()
        }
    }

    fn get(&mut self, url: &str) -> Result<String, Error>{
        let url = if url.starts_with("https://") {
            url.to_string()
        } else {
            format!("{}{}", self.host, url)
        };
        let output = self.cmd.arg(&url).stderr(Stdio::inherit()).output().unwrap();
        if output.status.success() {
            Ok(String::from_utf8(output.stdout).unwrap())
        } else {
            panic!("failed to fetch `{}` : {}", url, output.status)
        }
    }
}

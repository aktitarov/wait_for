use std::process::Command;
use std::time::{Duration, Instant};

pub fn exec(sources: Vec<&str>) {
    let mut command = Command::new(sources[0]);
    for arg in sources[1..].iter() {
        command.arg(arg);
    }
    command.spawn().expect("Command executing failed");
}

#[derive(Debug)]
pub struct Service<'a> {
    host: &'a str,
    port: &'a str,
}

impl<'a> Service<'a> {
    pub fn new(host: &'a str, port: &'a str) -> Self {
        Service { host, port }
    }

    fn ping(&self) -> bool {
        let output = Command::new("nc")
            .arg("-z")
            .arg(self.host)
            .arg(self.port)
            .output();
        match output {
            Ok(v) => {
                if let Some(code) = v.status.code() {
                    match code {
                        0 => true,
                        _ => false,
                    }
                } else {
                    false
                }
            }
            Err(_) => false,
        }
    }

    pub fn ping_until_timeout(&self, timeout: &Duration) -> bool {
        let start = Instant::now();
        loop {
            let now = Instant::now();
            if now.duration_since(start) > *timeout {
                break false;
            }
            if self.ping() {
                break true;
            }
        }
    }
}

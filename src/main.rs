use std::time::Duration;

mod cli;
mod utils;

use utils::{exec, Service};

fn main() {
    let matches = cli::build_cli().get_matches();

    let service = Service::new(
        matches.value_of("host").unwrap(),
        matches.value_of("port").unwrap(),
    );

    let timeout = Duration::from_secs(
        matches
            .value_of("timeout")
            .unwrap_or("15")
            .parse::<u64>()
            .expect("Timeout must be a positive number"),
    );

    let command = match matches.values_of("command") {
        Some(v) => v.collect::<Vec<&str>>(),
        None => Vec::new(),
    };

    match service.ping_until_timeout(&timeout) {
        true => exec(command),
        _ => println!("Operation timed out"),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_existing_service_must_be_found() {
        let service = Service::new("google.com", "80");
        let timeout = Duration::from_secs(15);
        let result = service.ping_until_timeout(&timeout);
        assert_eq!(result, true);
    }

    #[test]
    fn test_non_existing_service_must_not_be_found() {
        let service = Service::new("example", "9000");
        let timeout = Duration::from_secs(15);
        let result = service.ping_until_timeout(&timeout);
        assert_eq!(result, false);
    }
}

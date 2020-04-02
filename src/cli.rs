use clap::{App, AppSettings, Arg};

pub fn build_cli() -> App<'static, 'static> {
    let host_arg = Arg::with_name("host")
        .help("Sets host of service to be watched")
        .long("host")
        .short("h")
        .required(true)
        .takes_value(true);

    let port_arg = Arg::with_name("port")
        .help("Sets port of service to be watched")
        .long("port")
        .short("p")
        .required(true)
        .takes_value(true);

    let timeout_arg = Arg::with_name("timeout")
        .help("Sets timeout in secons, zero for no timeout")
        .long("timeout")
        .short("t")
        .takes_value(true);

    let command_arg = Arg::with_name("command")
        .help("Command to execute after service is available")
        .multiple(true);

    App::new("wait for")
        .about("Waits for another service to become available")
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(host_arg)
        .arg(port_arg)
        .arg(timeout_arg)
        .arg(command_arg)
}

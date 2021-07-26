use clap::ArgMatches;

pub fn clap_app_auto_deploy() -> clap::App<'static, 'static> {
    use clap::*;

    SubCommand::with_name("auto-deploy")
        .about("Crafts a \"Deploy\" transaction directly from the SVM SDK output.")
        .arg(
            Arg::with_name("input")
                .help("Reads JSON-formatted transactions from this file")
                .short("i")
                .long("input")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output")
                .help("Writes the binary output to this file")
                .short("o")
                .long("output")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("tx-type")
                .help("The type of input transaction")
                .long("tx-type")
                .required(true)
                .takes_value(true)
                .possible_values(&["spawn", "deploy", "call"]),
        )
}

pub fn subcmd_auto_deploy(_args: &ArgMatches) -> anyhow::Result<()> {
    Ok(())
}

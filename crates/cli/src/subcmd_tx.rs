use clap::ArgMatches;

use std::fs::File;
use std::io::{self, Write};
use std::str::FromStr;

pub fn clap_app_tx() -> clap::App<'static, 'static> {
    use clap::*;

    SubCommand::with_name("tx")
        .about("Crafts a transaction and writes its byte representation to a file")
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .help("Writes the output to a binary file")
                .takes_value(true),
        )
        .subcommand(
            SubCommand::with_name("spawn")
                .about("Crafts a transaction of type `spawn`")
                .arg(
                    Arg::with_name("version")
                        .default_value("0")
                        .short("V")
                        .long("Version")
                        .help("Template version"),
                )
                .arg(
                    Arg::with_name("name")
                        .required(true)
                        .long("name")
                        .help("The name of the `spawn`'ed Account.")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("template")
                        .required(true)
                        .long("template")
                        .takes_value(true)
                        .help("The name of the Account's Template."),
                )
                .arg(
                    Arg::with_name("ctor")
                        .long("ctor")
                        .required(true)
                        .takes_value(true)
                        .help("The `ctor` of the Template"),
                )
                .arg(
                    Arg::with_name("calldata")
                        .long("calldata")
                        .required(true)
                        .takes_value(true)
                        .help("A binary file containing the calldata"),
                ),
        )
        .subcommand(SubCommand::with_name("deploy").about("Crafts a transactino of type`deploy`"))
        .subcommand(SubCommand::with_name("call").about("Crafts a transaction of type `call`"))
}

pub fn subcmd_tx(args: &ArgMatches) -> anyhow::Result<()> {
    match args.subcommand() {
        ("spawn", Some(spawn_args)) => subcmd_tx_spawn(args, spawn_args)?,
        ("call", Some(args)) => unimplemented!("https://github.com/spacemeshos/svm/issues/304"),
        ("deploy", Some(args)) => unimplemented!("https://github.com/spacemeshos/svm/issues/303"),
        (_, _) => unreachable!(),
    }
    Ok(())
}

fn subcmd_tx_spawn(tx_args: &ArgMatches, spawn_args: &ArgMatches) -> anyhow::Result<()> {
    let mut file = File::create(tx_args.value_of("file").unwrap())?;

    let version = {
        let s = spawn_args.value_of("version").unwrap();
        u16::from_str(s)?
    };
    let ctor = spawn_args.value_of("ctor").unwrap();
    let name = spawn_args.value_of("name").unwrap();
    let name = spawn_args.value_of("calldata").unwrap();
    let template_addr = {
        let s = spawn_args.value_of("template").unwrap();
        svm_types::Address::of(s)
    };
    let builder = svm_codec::api::builder::SpawnBuilder::new()
        .with_ctor(ctor)
        .with_version(version)
        .with_template(&template_addr.into())
        .with_name(name);

    file.write_all(&builder.build())?;
    Ok(())
}

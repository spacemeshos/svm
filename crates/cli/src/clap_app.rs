use super::subcmd_tx::clap_app_tx;
use super::subcmd_validate::clap_app_validate;

pub fn clap_app() -> clap::App<'static, 'static> {
    use clap::*;

    // Help messages all use the third person rather than the imperative form,
    // e.g. "prints" rather than "print".

    App::new("svm-cli")
        .version("1.0")
        .author("The Spacemesh team")
        .about("A CLI tool to access SVM internal utilities")
        // The user must provide a valid subcommand, otherwise we don't really
        // know what to do.
        .setting(clap::AppSettings::SubcommandRequired)
        .subcommand(clap_app_validate())
        .subcommand(clap_app_tx())
}

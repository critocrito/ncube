use clap::{App, AppSettings, Arg};

const HELP: &str = "{about}

USAGE:
    {usage}

{all-args}";

const HELP_DELETE: &str = "{about}

USAGE:
    {usage}

FLAGS:
{flags}";

const USAGE_DELETE: &str = "ncubectl delete workspace <workspace> [-y]
    ncubectl delete account <workspace> <email> [-y]";

pub(crate) fn workspace_cli() -> App<'static> {
    App::new("workspace")
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::DisableVersion)
        .help_template(HELP)
        .about("Create a new local workspace.")
        .arg(
            Arg::new("name")
                .value_name("name")
                .required(true)
                .about("The descriptive name for this workspace.")
                .takes_value(true),
        )
        .arg(
            Arg::new("postgres_url")
                .value_name("postgres_url")
                .about("Associate a PostgreSQL database with this workspace.")
                .takes_value(true),
        )
}

pub(crate) fn account_cli() -> App<'static> {
    App::new("account")
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::DisableVersion)
        .help_template(HELP)
        .about("Create a new account for a workspace.")
        .arg(
            Arg::new("workspace")
                .value_name("workspace")
                .required(true)
                .about("The slug of the workspace this account is associated to.")
                .takes_value(true),
        )
        .arg(
            Arg::new("email")
                .value_name("email")
                .required(true)
                .about("The email address of the new account.")
                .takes_value(true),
        )
}

pub(crate) fn state_cli() -> App<'static> {
    App::new("state")
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::DisableVersion)
        .help_template(HELP)
        .about("Print the state of the local Ncube installation.")
        .arg(
            Arg::new("modifier")
                .value_name("modifier")
                .required(true)
                .possible_values(&["workspaces", "accounts", "all"])
                .about("Specify the state to print.")
                .takes_value(true),
        )
}

pub(crate) fn reset_cli() -> App<'static> {
    App::new("reset")
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::DisableVersion)
        .help_template(HELP)
        .about("Reset settings of the local Ncube installation.")
        .arg(
            Arg::new("modifier")
                .value_name("modifier")
                .required(true)
                .possible_values(&["secret"])
                .about("reset the secret key.")
                .takes_value(true),
        )
}

pub(crate) fn migrate_cli() -> App<'static> {
    App::new("migrate")
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::DisableVersion)
        .about("Migrate a workspace to the latest version of Ncube.")
        .arg(
            Arg::new("workspace")
                .value_name("workspace")
                .required(true)
                .about("The slug of the workspace this account is associated to.")
                .takes_value(true),
        )
}

pub(crate) fn connection_cli() -> App<'static> {
    App::new("connection")
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::DisableVersion)
        .help_template(HELP)
        .about("Print the connection details for a workspace account.")
        .arg(
            Arg::new("workspace")
                .value_name("workspace")
                .required(true)
                .about("The slug of the workspace this account is associated to.")
                .takes_value(true),
        )
        .arg(
            Arg::new("email")
                .value_name("email")
                .required(true)
                .about("The email address of the new account.")
                .takes_value(true),
        )
}

pub(crate) fn delete_cli() -> App<'static> {
    App::new("delete")
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::DisableVersion)
        .help_template(HELP_DELETE)
        .about("Delete a workspace or account.")
        .override_usage(USAGE_DELETE)
        .subcommand(
            App::new("workspace")
                .setting(AppSettings::ArgRequiredElseHelp)
                .setting(AppSettings::DisableVersion)
                .arg(
                    Arg::new("workspace")
                        .value_name("workspace")
                        .required(true)
                        .about("The slug of the workspace this account is associated to.")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("assume_yes")
                        .short('y')
                        .about("Assume 'yes' as answer to all prompts.")
                        .required(false)
                        .takes_value(false),
                ),
        )
        .subcommand(
            App::new("account")
                .setting(AppSettings::ArgRequiredElseHelp)
                .setting(AppSettings::DisableVersion)
                .arg(
                    Arg::new("workspace")
                        .value_name("workspace")
                        .required(true)
                        .about("The slug of the workspace this account is associated to.")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("email")
                        .value_name("email")
                        .required(true)
                        .about("The email address of the new account.")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("assume_yes")
                        .short('y')
                        .about("Assume 'yes' as answer to all prompts.")
                        .required(false)
                        .takes_value(false),
                ),
        )
}

pub(crate) fn get_cli() -> App<'static> {
    App::new("get")
        .setting(AppSettings::DisableVersion)
        .about("Show configuration settings.")
}

pub(crate) fn set_cli() -> App<'static> {
    App::new("set")
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::DisableVersion)
        .about("Set a configuration setting.")
        .arg(
            Arg::new("setting")
                .value_name("setting")
                .required(true)
                .about("The name of configuration setting.")
                .takes_value(true),
        )
        .arg(
            Arg::new("value")
                .value_name("value")
                .about("The new value of this configuration setting.")
                .required(true)
                .takes_value(true),
        )
}

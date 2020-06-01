use clap::{App, AppSettings, Arg};

pub(crate) mod create;
pub(crate) mod list;

const HELP: &'static str = "{about}

USAGE:
    {usage}

{all-args}";

const HELP_DELETE: &'static str = "{about}

USAGE:
    {usage}

FLAGS:
{flags}";

const USAGE_DELETE: &'static str = "ncubectl delete workspace <workspace> [-y]
    ncubectl delete account <workspace> <email> [-y]";

pub(crate) fn workspace_cli() -> App<'static> {
    App::new("workspace")
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::DisableVersion)
        .help_template(HELP)
        .about("Create a new local workspace.")
        .arg(
            Arg::with_name("name")
                .value_name("name")
                .required(true)
                .about("The descriptive name for this workspace.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("postgres_url")
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
            Arg::with_name("workspace")
                .value_name("workspace")
                .required(true)
                .about("The slug of the workspace this account is associated to.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("email")
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
            Arg::with_name("modifier")
                .value_name("modifier")
                .required(true)
                .possible_values(&["workspaces", "accounts", "all"])
                .about("Specify the state to print.")
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
            Arg::with_name("workspace")
                .value_name("workspace")
                .required(true)
                .about("The slug of the workspace this account is associated to.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("email")
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
                    Arg::with_name("workspace")
                        .value_name("workspace")
                        .required(true)
                        .about("The slug of the workspace this account is associated to.")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("assume_yes")
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
                    Arg::with_name("workspace")
                        .value_name("workspace")
                        .required(true)
                        .about("The slug of the workspace this account is associated to.")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("email")
                        .value_name("email")
                        .required(true)
                        .about("The email address of the new account.")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("assume_yes")
                        .short('y')
                        .about("Assume 'yes' as answer to all prompts.")
                        .required(false)
                        .takes_value(false),
                ),
        )
}

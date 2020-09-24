use clap::{crate_authors, crate_version, App, AppSettings, Arg};
use directories::ProjectDirs;
use ncube_data::DatabaseRequest;
use ncubed::{Application, ApplicationConfig};
use std::fs::create_dir_all;
use tracing::Level;

mod cli;
mod cmd;
mod types;

const USAGE: &str = "ncubectl [-hV -d database -v]
    ncubectl workspace <name> [<postgres_url>]
    ncubectl account <workspace> <email>
    ncubectl connection <workspace> <email>
    ncubectl state [workspaces|accounts|settings|all]
    ncubectl get
    ncubectl set <setting> <value>
    ncubectl reset [secret]
    ncubectl migrate <workspace>
    ncubectl delete workspace <workspace> [-y]
    ncubectl delete account <workspace> <email> [-y]
";

const HELP: &str = "{bin} - {about}

USAGE:
    {usage}

FLAGS:
{flags}";

macro_rules! _fatal {
    ($fmt:expr) => ({
        eprint!(concat!("ncubectl: ", $fmt, "\n"));
        ::std::process::exit(1);
    });
    ($fmt:expr, $($arg:tt)*) => ({
        eprint!(concat!("ncubectl: ", $fmt, "\n"), $($arg)*);
        ::std::process::exit(1);
    });
}
pub(crate) use _fatal as fatal;

#[tokio::main]
async fn main() {
    let project = ProjectDirs::from("net", "sugarcubetools", "Ncube").unwrap();
    let cfg_dir = project.config_dir();
    let project_dir_db_path = cfg_dir.join("ncube.db");

    let matches = App::new("ncubectl")
        .setting(AppSettings::ArgRequiredElseHelp)
        .override_usage(USAGE)
        .version(crate_version!())
        .author(crate_authors!())
        .about("configure and control the Ncube daemon")
        .help_template(HELP)
        .arg(
            Arg::new("database")
                .short('d')
                .long("database")
                .about("Path to Ncube host database.")
                .required(false)
                .default_value(&project_dir_db_path.to_string_lossy())
                .takes_value(true),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .about("Enable verbose logging. Use multiple times to increase verbosity.")
                .multiple(true)
                .takes_value(false),
        )
        .subcommand(cli::workspace_cli())
        .subcommand(cli::account_cli())
        .subcommand(cli::state_cli())
        .subcommand(cli::get_cli())
        .subcommand(cli::set_cli())
        .subcommand(cli::reset_cli())
        .subcommand(cli::migrate_cli())
        .subcommand(cli::connection_cli())
        .subcommand(cli::delete_cli())
        .get_matches();

    // matches.is_present() returns true since we use a default value.
    let db_path = if matches.occurrences_of("database") == 0 {
        create_dir_all(&cfg_dir).unwrap();
        project_dir_db_path
    } else {
        matches.value_of("database").unwrap().into()
    };
    if matches.occurrences_of("verbose") > 0 {
        let tracing_level = match matches.occurrences_of("verbose") {
            1 => Level::INFO,
            2 => Level::DEBUG,
            3 => Level::TRACE,
            _ => Level::TRACE,
        };

        tracing_subscriber::fmt()
            .with_max_level(tracing_level)
            .init();
    }

    let config = ApplicationConfig {
        host_db: format!("sqlite://{}", db_path.to_str().unwrap()),
        listen: "127.0.0.1:40666".parse().unwrap(),
    };

    let app = Application::new(config);
    app.run_without_http().await.unwrap();

    match matches.subcommand() {
        ("account", Some(matches)) => {
            let workspace = matches.value_of("workspace").unwrap();
            let email = matches.value_of("email").unwrap();

            cmd::account(&workspace, &email).await;
        }
        ("workspace", Some(workspace_matches)) => {
            // FIXME: handle Postgresql database kind
            // let database = if workspace_matches.is_present("postgres_url") {
            //     DatabaseRequest::Postgresql
            // } else {
            //     DatabaseRequest::Sqlite
            // };
            let database = DatabaseRequest::Sqlite;

            let workspace_name = workspace_matches.value_of("name").unwrap();

            cmd::workspace(&workspace_name, database).await;
        }
        ("connection", Some(connection_matches)) => {
            let workspace = connection_matches.value_of("workspace").unwrap();
            let email = connection_matches.value_of("email").unwrap();

            cmd::connection(&workspace, &email).await;
        }
        ("state", Some(state_matches)) => {
            let modifier = state_matches.value_of("modifier").unwrap_or("all");

            match modifier {
                "workspaces" => cmd::state_workspaces().await,
                "accounts" => cmd::state_accounts().await,
                "all" => {
                    println!("WORKSPACES:");
                    cmd::state_workspaces().await;
                    println!("\nACCOUNTS:");
                    cmd::state_accounts().await;
                }
                _ => fatal!("Unknown state modifier."),
            }
        }
        ("reset", Some(state_matches)) => {
            let modifier = state_matches.value_of("modifier").unwrap();

            match modifier {
                "secret" => cmd::reset_secret().await,
                _ => fatal!("Unknown reset modifier."),
            }
        }

        ("migrate", Some(state_matches)) => {
            let workspace = state_matches.value_of("workspace").unwrap();

            cmd::migrate(&workspace).await;
        }

        ("delete", Some(delete_matches)) => match delete_matches.subcommand() {
            ("workspace", Some(delete_workspace_matches)) => {
                let workspace = delete_workspace_matches.value_of("workspace").unwrap();

                println!(
                    "Delete an workspace: {} - {}",
                    workspace,
                    delete_workspace_matches.is_present("assume_yes")
                );
            }
            ("account", Some(delete_account_matches)) => {
                let workspace = delete_account_matches.value_of("workspace").unwrap();
                let email = delete_account_matches.value_of("email").unwrap();
                println!("Delete an account: {}/{}", workspace, email);
            }
            _ => unreachable!(),
        },
        ("get", Some(_)) => {
            cmd::get().await;
        }
        ("set", Some(set_matches)) => {
            let setting = set_matches.value_of("setting").unwrap();
            let value = set_matches.value_of("value").unwrap();

            cmd::set(&setting, &value).await;
        }
        _ => unreachable!(),
    };
}

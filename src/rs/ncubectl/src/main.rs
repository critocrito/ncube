use clap::{crate_authors, crate_version, App, AppSettings, Arg};
use directories::ProjectDirs;
use ncubed::{types::DatabaseRequest, Application, ApplicationConfig};
use std::fs::create_dir_all;
use tracing::Level;

mod cmd;

const USAGE: &'static str = "ncubectl [-hV -d database -v]
    ncubectl workspace <name> [<postgres_url>]
    ncubectl account <workspace> <email>
    ncubectl connection <workspace> <email>
    ncubectl state [workspaces|accounts|settings|all]
    ncubectl reset [secret]
    ncubectl delete workspace <workspace> [-y]
    ncubectl delete account <workspace> <email> [-y]
";

const HELP: &'static str = "{bin} - {about}

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
            Arg::with_name("database")
                .short('d')
                .long("database")
                .about("Path to Ncube host database.")
                .required(false)
                .default_value(&project_dir_db_path.to_string_lossy())
                .takes_value(true),
        )
        .arg(
            Arg::with_name("verbose")
                .short('v')
                .long("verbose")
                .about("Enable verbose logging. Use multiple times to increase verbosity.")
                .required(false)
                .takes_value(false)
                .multiple(true),
        )
        .subcommand(cmd::workspace_cli())
        .subcommand(cmd::account_cli())
        .subcommand(cmd::state_cli())
        .subcommand(cmd::reset_cli())
        .subcommand(cmd::connection_cli())
        .subcommand(cmd::delete_cli())
        .get_matches();

    // matches.is_present() returns true since we use a default value.
    let db_path = if matches.occurrences_of("database") == 0 {
        create_dir_all(&cfg_dir).unwrap();
        project_dir_db_path
    } else {
        matches.value_of("database").unwrap().into()
    };
    if matches.occurrences_of("v") > 0 {
        let tracing_level = match matches.occurrences_of("v") {
            1 => Level::INFO,
            2 => Level::DEBUG,
            3 | _ => Level::TRACE,
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

            cmd::create::account(&workspace, &email).await;
        }
        ("workspace", Some(workspace_matches)) => {
            let database = if workspace_matches.is_present("postgres_url") {
                // FIXME: handle Postgresql database kind
                DatabaseRequest::Sqlite
            } else {
                DatabaseRequest::Sqlite
            };

            let workspace_name = workspace_matches.value_of("name").unwrap();

            cmd::create::workspace(&workspace_name, database).await;
        }
        ("connection", Some(connection_matches)) => {
            let email = connection_matches.value_of("email").unwrap();
            let workspace = connection_matches.value_of("workspace").unwrap();

            println!("Show connection details: {}/{}", email, workspace);
        }
        ("state", Some(state_matches)) => {
            let modifier = state_matches.value_of("modifier").unwrap_or("all");

            match modifier {
                "workspaces" => cmd::list::workspaces().await,
                "accounts" => cmd::list::accounts().await,
                "all" => {
                    println!("WORKSPACES:");
                    cmd::list::workspaces().await;
                    println!("\nACCOUNTS:");
                    cmd::list::accounts().await;
                }
                _ => fatal!("Unknown state modifier."),
            }
        }
        ("reset", Some(state_matches)) => {
            let modifier = state_matches.value_of("modifier").unwrap();

            match modifier {
                "secret" => cmd::reset::secret().await,
                _ => fatal!("Unknown reset modifier."),
            }
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
        _ => unreachable!(),
    };
}

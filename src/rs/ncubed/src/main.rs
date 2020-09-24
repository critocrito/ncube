#![type_length_limit = "1375249"]
use clap::{crate_authors, crate_version, App, AppSettings, Arg};
use directories::ProjectDirs;
use ncubed::{Application, ApplicationConfig};
use std::fs::create_dir_all;
use tracing::Level;

#[tokio::main]
async fn main() {
    let project = ProjectDirs::from("net", "sugarcubetools", "Ncube").unwrap();
    let cfg_dir = project.config_dir();
    let project_dir_db_path = cfg_dir.join("ncube.db");

    let matches = App::new("ncubed")
        .setting(AppSettings::ArgRequiredElseHelp)
        .version(crate_version!())
        .author(crate_authors!())
        .about("The ncube daemon.")
        .arg(
            Arg::new("listen_address")
                .short('l')
                .long("listen")
                .about("Set the listen address and port.")
                .required(false)
                .default_value("127.0.0.1:40666")
                .takes_value(true),
        )
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
        .get_matches();

    // matches.is_present() returns true since we use a default value.
    let db_path = if matches.occurrences_of("database") == 0 {
        create_dir_all(&cfg_dir).unwrap();
        project_dir_db_path
    } else {
        matches.value_of("database").unwrap().into()
    };
    let listen_address = matches.value_of("listen_address").unwrap();

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
        listen: listen_address.parse().unwrap(),
    };

    let app = Application::new(config);
    app.run().await.unwrap();
}

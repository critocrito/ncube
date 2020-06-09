use ncubed::{
    crypto, handlers,
    types::{AccountRequest, DatabaseRequest, WorkspaceKindRequest, WorkspaceRequest},
};
use prettytable::{cell, format::FormatBuilder, row, Table};
use std::io::Write;

use crate::fatal;
use crate::types::ConnectionOut;

pub(crate) async fn account(workspace: &str, email: &str) {
    handlers::account::create_account(
        workspace,
        AccountRequest {
            email: email.to_string(),
            name: None,
        },
    )
    .await
    .unwrap_or_else(|e| fatal!("cannot create account: {}", e.to_string()));
}

pub(crate) async fn workspace(name: &str, database: DatabaseRequest) {
    let request = WorkspaceRequest {
        name: name.to_string(),
        description: None,
        kind: WorkspaceKindRequest::Local,
        database,
    };
    // FIXME: project directory get's created asynchronously, need to
    // wait for that.
    handlers::workspace::create_workspace(request)
        .await
        .unwrap_or_else(|e| fatal!("cannot create workspace: {}", e.to_string()));
}

pub(crate) async fn state_workspaces() {
    let workspaces = handlers::workspace::list_workspaces()
        .await
        .unwrap_or_else(|e| fatal!("failed to list workspaces: {}", e.to_string()));

    let table_format = FormatBuilder::new().padding(0, 2).build();
    let mut table = Table::new();
    table.set_format(table_format);

    table.add_row(row!["SLUG", "NAME"]);
    for workspace in workspaces {
        table.add_row(row![workspace.slug, workspace.name]);
    }
    table.printstd();
}

pub(crate) async fn state_accounts() {
    let accounts = handlers::account::list_accounts()
        .await
        .unwrap_or_else(|e| fatal!("failed to list accounts: {}", e.to_string()));

    let table_format = FormatBuilder::new().padding(0, 2).build();
    let mut table = Table::new();
    table.set_format(table_format);

    table.add_row(row![
        "EMAIL",
        "WORKSPACE",
        "CREATED_AT",
        "UPDATED_AT",
        "OTP"
    ]);

    for account in accounts {
        let otp = if account.is_otp && handlers::account::is_valid_otp(account.updated_at) {
            account.otp.unwrap_or_else(|| "".to_string())
        } else {
            "".to_string()
        };

        table.add_row(row![
            account.email,
            account.workspace,
            account.created_at.to_rfc3339(),
            account.updated_at.to_rfc3339(),
            otp,
        ]);
    }
    table.printstd();
}

pub(crate) async fn connection(workspace: &str, email: &str) {
    let workspace = handlers::workspace::show_workspace(&workspace)
        .await
        .unwrap_or_else(|e| fatal!("failed to show workspace: {}", e.to_string()));
    let account = handlers::account::show_account(&workspace.slug, &email)
        .await
        .unwrap_or_else(|e| fatal!("failed to show account: {}", e.to_string()));
    let endpoint = handlers::config::endpoint()
        .await
        .unwrap_or_else(|e| fatal!("failed to retrieve endpoint setting: {}", e.to_string()));

    let connection = ConnectionOut {
        name: workspace.name,
        workspace: workspace.slug,
        description: workspace.description,
        email: account.email,
        otp: account.otp,
        created_at: account.created_at,
        updated_at: account.updated_at,
        endpoint,
    };

    let json = serde_json::to_string(&connection).unwrap();
    let mut stdout = std::io::stdout();
    stdout.write_all(json.as_bytes()).unwrap();
}

pub(crate) async fn reset_secret() {
    let rng = rand::thread_rng();
    let key = crypto::gen_secret_key(rng);

    handlers::config::insert_config_setting("secret_key", &key)
        .await
        .unwrap_or_else(|e| fatal!("failed to reset secret key: {}", e.to_string()));
}

pub(crate) async fn get() {
    let settings = handlers::config::show_config_all().await.unwrap();

    let table_format = FormatBuilder::new().padding(0, 2).build();
    let mut table = Table::new();
    table.set_format(table_format);

    table.add_row(row!["NAME", "VALUE", "REQUIRED", "RESTRICTED"]);
    for s in settings {
        table.add_row(row![
            s.name,
            s.value.or_else(|| Some("".into())).unwrap(),
            s.required,
            s.restricted
        ]);
    }
    table.printstd();
}

pub(crate) async fn set(setting: &str, value: &str) {
    handlers::config::insert_config_setting(&setting, &value)
        .await
        .unwrap();
}

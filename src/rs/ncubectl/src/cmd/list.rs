use chrono::{DateTime, Utc};
use ncubed::handlers;
use prettytable::{cell, format::FormatBuilder, row, Table};
use serde::Serialize;
use serde_json;
use std::io::Write;

use crate::fatal;

#[derive(Debug, Serialize)]
struct ConnectionOut {
    workspace: String,
    description: Option<String>,
    email: String,
    otp: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

pub(crate) async fn workspaces() {
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

pub(crate) async fn accounts() {
    let accounts = handlers::account::list_accounts()
        .await
        .unwrap_or_else(|e| fatal!("failed to list accounts: {}", e.to_string()));

    let table_format = FormatBuilder::new().padding(0, 2).build();
    let mut table = Table::new();
    table.set_format(table_format);

    table.add_row(row![
        "EMAIL",
        "CREATED_AT",
        "UPDATED_AT",
        "OTP",
        "WORKSPACE"
    ]);

    for account in accounts {
        let otp = match account.otp {
            Some(k) => k,
            None => "".to_string(),
        };

        table.add_row(row![
            account.email,
            account.created_at,
            account.updated_at,
            otp,
            account.workspace,
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

    let connection = ConnectionOut {
        workspace: workspace.slug,
        description: workspace.description,
        email: account.email,
        otp: account.otp,
        created_at: account.created_at,
        updated_at: account.updated_at,
    };

    let json = serde_json::to_string(&connection).unwrap();
    let mut stdout = std::io::stdout();
    stdout.write(json.as_bytes()).unwrap();
}

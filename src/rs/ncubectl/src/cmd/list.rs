use ncubed::handlers;
use prettytable::{cell, format::FormatBuilder, row, Table};

use crate::fatal;

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

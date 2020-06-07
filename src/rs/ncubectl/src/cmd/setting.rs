use ncubed::handlers;
use prettytable::{cell, format::FormatBuilder, row, Table};

pub(crate) async fn get() {
    let settings = handlers::config::show_config_all().await.unwrap();

    let table_format = FormatBuilder::new().padding(0, 2).build();
    let mut table = Table::new();
    table.set_format(table_format);

    table.add_row(row!["NAME", "VALUE", "REQUIRED", "RESTRICTED"]);
    for s in settings {
        table.add_row(row![
            s.name,
            s.value.or(Some("".into())).unwrap(),
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

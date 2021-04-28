use crate::client::globalsettings::GlobalSettings;
use crate::client::utils::OutputStyle;
use crate::transfer::messages::WorkerInfo;
use cli_table::format::Justify;
use cli_table::{print_stdout, Cell, Color, Style, Table};
use std::process::Output;

pub fn print_worker_info(workers: Vec<WorkerInfo>, gsettings: &GlobalSettings) {
    let rows: Vec<_> = workers
        .into_iter()
        .map(|w| {
            vec![
                w.id.cell().justify(Justify::Right),
                match w.ended_at {
                    None => "RUNNING".cell().foreground_color(Some(Color::Green)),
                    Some(_) => "OFFLINE".cell().foreground_color(Some(Color::Red)),
                },
                w.configuration.hostname.cell(),
                w.configuration.n_cpus.cell(),
            ]
        })
        .collect();

    let table = rows
        .table()
        .color_choice(gsettings.color_policy())
        .title(vec![
            "Id".cell(),
            "State".cell().bold(true),
            "Hostname".cell().bold(true),
            "# cpus".cell().bold(true),
        ]);
    assert!(print_stdout(table).is_ok());
}

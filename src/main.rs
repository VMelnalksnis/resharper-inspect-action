use std::env;
use crate::report::parse_report;

mod report;

fn main() {
    let report_filepath = env::args().nth(1).expect("no report filepath specified");
    let file_content: String = std::fs::read_to_string(report_filepath).unwrap();
    let report = parse_report(&file_content);

    let warning_type_ids: Vec<&String> = report.issue_types.issue_types
        .iter()
        .filter(|t| t.severity == "WARNING")
        .map(|t| &t.id)
        .collect();

    let warnings = report.issues.projects
        .iter()
        .flat_map(|p| &p.issues)
        .filter(|i| warning_type_ids.contains(&&i.type_id));

    for warning in warnings {
        write_annotation("warning", &warning.file, &warning.line, &warning.type_id, &warning.message);
    }

    let error_type_ids: Vec<&String> = report.issue_types.issue_types
        .iter()
        .filter(|t| t.severity == "ERROR")
        .map(|t| &t.id)
        .collect();

    let errors = report.issues.projects
        .iter()
        .flat_map(|p| &p.issues)
        .filter(|i| error_type_ids.contains(&&i.type_id));

    for error in errors {
        write_annotation("error", &error.file, &error.line, &error.type_id, &error.message);
    }

    if !error_type_ids.is_empty() {
        std::process::exit(1)
    }
}

fn write_annotation(level: &str, file: &String, line: &u32, title: &String, message: &String) {
    let filename = file.replace("\\", "/");
    println!("::{0} file={1},line={2},title={3}::{4}", level, filename, line, title, message);
}

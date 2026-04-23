use chrono::{NaiveDateTime, TimeZone};
use chrono_tz::Tz;

pub fn convert_time(input_time: &str, abbr: &str, local_tz: &str) -> String {
    let tz_from_abbr: &str = resolve_abbreviation(abbr);
    let source_tz: Tz = tz_from_abbr.parse().expect("Error parsing source timezone");
    let target_tz: Tz = local_tz.parse().expect("Error parsing targeted timezone");

    let source_dt: NaiveDateTime = NaiveDateTime::parse_from_str(input_time, "%d.%m.%Y %H:%M")
        .expect("Error parsing datetime");

    let target_dt = source_tz.from_local_datetime(&source_dt).single();

    let converted_dt = target_dt.unwrap().with_timezone(&target_tz);

    converted_dt.format("%d.%m.%Y %H:%M").to_string()
}

pub fn resolve_abbreviation(input: &str) -> &str {
    match input.trim().to_uppercase().as_str() {
        "EST" | "EDT" => "America/New_York",
        "CST" | "CDT" => "America/Chicago",
        "MST" | "MDT" => "America/Denver",
        "PST" | "PDT" => "America/Los_Angeles",

        "GMT" => "Etc/GMT",
        "UTC" => "Etc/UTC",
        "CET" | "CEST" => "Europe/Paris",
        "IST" => "Asia/Kolkata",
        "AEST" | "AEDT" => "Australia/Sydney",
        &_ => "",
    }
}

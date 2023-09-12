use std::collections::HashMap;
use std::fs::{read_to_string, rename};

use chrono::{Datelike, DateTime, NaiveDateTime, Timelike};
use clap::Parser;
use regex::Regex;
use strfmt::strfmt;

type Vars = HashMap<String, String>;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// File to rename
    file: Option<String>,

    /// Exif filename
    #[arg(short, long)]
    exif: String,

    /// filename pattern
    #[arg(short, long)]
    pattern: String,
}

fn read_lines(filename: &str) -> Vec<String> {
    return read_to_string(filename).unwrap().lines().map(|s| s.to_string()).collect();
}

fn split_exif_line(s: &str) -> Option<(String, String)> {
    let tokens: Vec<&str> = s.splitn(2, ":").collect();
    if tokens.len() == 2 {
        Some((tokens[0].trim().to_string(), tokens[1].trim().to_string()))
    } else {
        None
    }
}

fn read_exif_file(filepath: &str) -> Result<Vars, String> {
    let mut vars: Vars = HashMap::new();
    let lines = read_lines(filepath);
    for line in lines {
        if let Some((key, value)) = split_exif_line(line.as_str()) {
            vars.insert(key, value);
        }
    }
    Ok(vars)
}

fn extend_vars(exif_vars: &Vars) -> Vars {
    let mut vars: Vars = HashMap::new();
    if let Some(create_date) = exif_vars.get("CreateDate") {
        vars.extend(create_vars_from_create_date(create_date));
    }
    if let Some(filename) = exif_vars.get("FileName") {
        vars.extend(create_vars_from_filename(filename));
    }
    if let Some(model) = exif_vars.get("Model") {
        vars.insert("T2".to_string(), model.to_string());
    }
    return vars;
}

fn create_vars_from_create_date(create_date: &str) -> Vars {
    // http://www.breezesys.com/downloads/Downloader_Pro_Manual.pdf
    let mut vars: Vars = HashMap::new();
    if let Some(datetime) = parse_datetime_from_string(create_date) {
        let date = datetime.date();
        let time = datetime.time();

        // Y - 4-digit year
        vars.insert("Y".to_string(), date.year().to_string());
        // y - 2-digit year
        vars.insert("y".to_string(), (date.year() % 100).to_string());
        // m - month (01-12)
        vars.insert("m".to_string(), format!("{:02}", date.month()));
        // D - Day of the month (01-31)
        vars.insert("D".to_string(), format!("{:02}", date.day()));
        // t - time HHMMSS
        vars.insert("t".to_string(), format!("{:02}{:02}{:02}", time.hour(), time.minute(), time.second()));
        // H - hour (01-23)
        vars.insert("H".to_string(), format!("{:02}", time.hour()));
        // h - hour (01-12)
        vars.insert("h".to_string(), format!("{:02}", time.hour12().1));
        // M - minutes (00-59)
        vars.insert("M".to_string(), format!("{:02}", time.minute()));
        // S - seconds (00-59)
        vars.insert("S".to_string(), format!("{:02}", time.second()));
        // W - week number
        vars.insert("W".to_string(), format!("{:02}", date.iso_week().week()));
        // a - Abbreviated weekday name (e.g. Fri)
        vars.insert("a".to_string(), date.weekday().to_string());
    }
    return vars;
}

fn create_vars_from_filename(filename: &str) -> Vars {
    let mut vars: Vars = HashMap::new();

    let regex = Regex::new(r"(.*\D)(\d*)\.([a-zA-Z0-9]+)").unwrap();
    if let Some(groups) = regex.captures(filename) {
        let image_name = groups.get(1).map_or("", |m| m.as_str());
        let image_number = groups.get(2).map_or("", |m| m.as_str());
        let extension = groups.get(3).map_or("", |m| m.as_str());
        vars.insert("f".to_string(), image_name.to_string());
        vars.insert("r".to_string(), image_number.to_string());
        vars.insert("e".to_string(), extension.to_string());
    }

    return vars;
}

fn parse_datetime_from_string(s: &str) -> Option<NaiveDateTime> {
    return match NaiveDateTime::parse_from_str(s, "%Y:%m:%d %H:%M:%S") {
        Ok(dt) => Some(dt),
        Err(_) => {
            match DateTime::parse_from_str(s, "%Y:%m:%d %H:%M:%S%z") {
                Ok(dt) => Some(dt.naive_local()),
                Err(e) => {
                    eprintln!("parse error: {}", e.to_string());
                    None
                },
            }
        }
    };
}

fn format_filename(pattern: &str, vars: &Vars) -> String {
    return strfmt(pattern, vars).unwrap();
}

fn main() {
    let args = Args::parse();

    let exif_filename: String = args.exif;
    let pattern: String = args.pattern;
    let file: Option<String> = args.file;

    match read_exif_file(exif_filename.as_str()) {
        Ok(exif_vars) => {
            let mut vars: Vars = extend_vars(&exif_vars);
            vars.extend(exif_vars);
            let filename = format_filename(pattern.as_str(), &vars);

            if let Some(source_filename) = file {
                match rename(source_filename.clone(), filename.clone()) {
                    Ok(()) => println!("{} -> {}", source_filename, filename),
                    Err(e) => eprintln!("Error: {}", e),
                }
            } else {
                println!("{}", filename)
            }
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }
}

#[test]
fn test_create_vars_from_create_date() {
    let create_date = "2023:09:08 18:56:54";
    let vars = create_vars_from_create_date(create_date);

    let expected = vec![
        ("D", "08"),
        ("H", "18"),
        ("M", "56"),
        ("S", "54"),
        ("W", "36"),
        ("Y", "2023"),
        ("a", "Fri"),
        ("h", "06"),
        ("m", "09"),
        ("t", "185654"),
        ("y", "23"),
    ];

    for (k, v) in expected {
        match vars.get(k) {
            Some(value) => assert_eq!(v, value),
            None => assert_eq!(v, "", "key={}", k)
        }
    }
}

#[test]
fn test_create_vars_from_filename() {
    let filename = "IMG_1234.JPG";
    let vars = create_vars_from_filename(filename);

    let expected = vec![
        ("f", "IMG_"),
        ("r", "1234"),
        ("e", "JPG"),
    ];

    for (k, v) in expected {
        match vars.get(k) {
            Some(value) => assert_eq!(v, value),
            None => assert_eq!(v, "", "key={}", k)
        }
    }
}

#[test]
fn test_format_filename() {
    let exif_vars = HashMap::from(
        [
            ("CreateDate", "2023:09:08 18:56:54"),
            ("FileName", "IMG_9876.JPG"),
            ("Model", "iPhone 14"),
        ].map(|(k, v)| (k.to_string(), v.to_string()))
    );

    let pattern = "{y}{m}{D}_{t}_{T2}_{r}.{e}";
    let expected = "230908_185654_iPhone 14_9876.JPG";

    let mut vars: Vars = extend_vars(&exif_vars);
    vars.extend(exif_vars);
    let actual = format_filename(pattern, &vars);

    assert_eq!(expected, actual)
}

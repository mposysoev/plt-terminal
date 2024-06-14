use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::style::{PointMarker, PointStyle};
use plotlib::view::ContinuousView;
use std::env;
use std::{fs::File, io::{BufRead, BufReader}};

fn get_data_from_file(path_to_file: &str) -> Vec<(f64, f64)> {
    let file = File::open(path_to_file).unwrap();
    let reader = BufReader::new(file);
    let mut values = Vec::new();
    let mut line_number = 0.0;

    for line in reader.lines() {
        let line = line.unwrap();
        if line.starts_with('#') || line.starts_with('%') || line.starts_with('/') {
            continue;
        }
        let mut iter = line.split_ascii_whitespace();
        let x: f64;
        let y: f64;

        if let Some(first_value) = iter.next() {
            let first_value: f64 = first_value.parse().unwrap_or_default();
            if let Some(second_value) = iter.next() {
                // If there's a second value, use the first as x and the second as y
                x = first_value;
                y = second_value.parse().unwrap_or_default();
            } else {
                // If there's only one value, use line number as x and the value as y
                x = line_number;
                y = first_value;
            }
            values.push((x, y));
            line_number += 1.0;
        }
    }
    values
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        return;
    }

    let file_name = &args[1];
    let file_data = get_data_from_file(file_name);

    let s = Plot::new(file_data).point_style(PointStyle::new().marker(PointMarker::Cross));

    let v = ContinuousView::new().add(s);

    println!("{}", Page::single(&v).dimensions(80, 30).to_text().unwrap());
}

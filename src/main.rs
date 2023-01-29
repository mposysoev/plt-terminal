use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::style::{PointMarker, PointStyle};
use plotlib::view::ContinuousView;
use std::env;
use std::{fs::File, io::Read};

fn get_data_from_file(path_to_file: &str) -> Vec<(f64, f64)> {
    let mut data = File::open(path_to_file).unwrap();
    let mut contents = String::new();
    data.read_to_string(&mut contents).unwrap();

    let lines = contents.lines();

    let mut values = Vec::new();

    for line in lines {
        if line.starts_with('#') {
            continue;
        } else {
            let mut iter = line.split_ascii_whitespace();
            let x: f64 = iter.next().unwrap().parse().unwrap();
            let y: f64 = iter.next().unwrap().parse().unwrap();
            values.push((x, y));
        }
    }
    values
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    let file_data = get_data_from_file(file_name);

    let s = Plot::new(file_data).point_style(PointStyle::new().marker(PointMarker::Cross));

    let v = ContinuousView::new().add(s);

    println!("{}", Page::single(&v).dimensions(80, 30).to_text().unwrap());
}

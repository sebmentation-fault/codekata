use std::io::Result;

use kata04::{get_min_spread, read_table};

fn main() {
    let table_path = "./data/weather.dat";
    let header_size = 2;
    let footer_size = 1;
    let ignore_rows = vec![];
    let item_name_col = 0;
    let col1 = 1;
    let col2 = 2;

    let lines = read_table(table_path);
    let min_day = get_min_spread(
        &lines,
        header_size,
        footer_size,
        &ignore_rows,
        item_name_col,
        col1,
        col2,
    );

    if let Some(day) = min_day {
        println!(
            "The minimum temperature spread was {} degrees, which was on day {}.",
            day.spread, day.item
        );
    } else {
        println!("No days were present in the file.");
    }
}

use std::fs::File;
use std::io::Read;

/// read a table, returning lines and words
pub fn read_table(path: &str) -> Vec<Vec<String>> {
    let mut buffer = String::new();
    let mut reader = File::open(path).expect("could not find file");
    reader
        .read_to_string(&mut buffer)
        .expect("could not read file");
    buffer
        .lines()
        .map(|line| line.split_whitespace().map(String::from).collect())
        .collect()
}

/// the table item
pub struct TableItem {
    pub item: String,
    pub spread: isize,
}

/// mininmum spread of goals and difference in temperature is the same mathematics
pub fn get_min_spread(
    table: &[Vec<String>], // table
    header_size: usize,    // header which has info about the cols
    footer_size: usize,    // weather has footer with total, ew
    ignore_rows: &[usize], // the football file has rows which we want to ignore which is the ------- one
    item_name_col: usize,  // the weather day/football team -- the subject of the search
    col1: usize, // the first column of the spread, e,g. temperature high or football goals for
    col2: usize, // the second column of the spread, e.g. temperature low or football goals against
) -> Option<TableItem> {
    let mut min_item: Option<TableItem> = None;
    let mut min_spread: isize = isize::MAX;

    // iterate over rows whilst ignoring header, footer and any other in between
    for (i, row) in table[header_size..table.len() - 1 - footer_size]
        .iter()
        .enumerate()
    {
        if ignore_rows.contains(&(i + header_size)) {
            continue;
        }

        let item = row
            .get(item_name_col)
            .expect("could not get item name")
            .parse::<String>()
            .expect("could not parse item");
        let col1_data = row
            .get(col1)
            .expect("could not get first column item")
            .replace('*', "")
            .parse::<isize>()
            .expect("could not parse temp");
        let col2_data = row
            .get(col2)
            .expect("could not get second column item")
            .replace('*', "")
            .parse::<isize>()
            .expect("could not parse temp");
        let spread = (col1_data - col2_data).abs();

        if spread < min_spread {
            min_item = Some(TableItem { item, spread });
            min_spread = spread;
        }
    }

    min_item
}

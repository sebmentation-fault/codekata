use kata04::{get_min_spread, read_table};

fn main() {
    let table_path = "./data/football.dat";
    let header_size = 1;
    let footer_size = 0;
    let ignore_rows = vec![18];
    let item_name_col = 1;
    let col1 = 6;
    let col2 = 8;

    let lines = read_table(table_path);
    let min_team = get_min_spread(
        &lines,
        header_size,
        footer_size,
        &ignore_rows,
        item_name_col,
        col1,
        col2,
    );

    if let Some(team) = min_team {
        println!(
            "The minimum goal spread was {} goals, which was by team {}.",
            team.spread, team.item
        );
    } else {
        println!("No teams were present in the file.");
    }
}

use std::io::Read;
use std::io::Result;

fn main() -> Result<()> {
    // 1. read ./data/football.dat
    let mut reader =
        std::fs::File::open("./data/football.dat").expect("could not find football file");
    let buffer: &mut String = &mut String::new();
    reader.read_to_string(buffer)?;

    // 2. split each line into words from each space between
    let lines: Vec<&str> = buffer.split('\n').collect();
    let mut min_team: Option<&str> = None;
    let mut min_goal_diff: isize = isize::MAX;

    // 2.1. start on 2 as to ignore the header and the blank line
    for line in &lines[2..lines.len() - 1] {
        let words: Vec<&str> = line.split_whitespace().collect();
        // 2.2. ignore lines that are just the hyphen
        if words.len() == 1 {
            continue;
        }

        // 3. look only at 2nd, 6th and 8th elements
        let team = words.get(1).expect("team was not present");
        let f = words
            .get(6)
            .expect("temp was not present")
            .parse::<isize>()
            .expect("could not parse temp");
        let a = words
            .get(8)
            .expect("temp was not present")
            .parse::<isize>()
            .expect("could not parse temp");
        let spread = (f - a).abs();

        // 4. keep track of min 2nd and which day it was for
        if spread < min_goal_diff {
            min_team = Some(*team);
            min_goal_diff = spread;
        }
    }

    // 5. return min day or none if there were no days present
    if let Some(team) = min_team {
        println!(
            "The minimum difference in for and against goal difference was {}, which was by team {}.",
            min_goal_diff, team
        );
    } else {
        println!("No teams were present in the file.");
    }
    Ok(())
}

use std::io::Read;
use std::io::Result;

fn main() -> Result<()> {
    // 1. read ./data/weather.dat
    let mut reader =
        std::fs::File::open("./data/weather.dat").expect("could not find weather file");
    let buffer: &mut String = &mut String::new();
    reader.read_to_string(buffer)?;

    // 2. split each line into words from each space between
    let lines: Vec<&str> = buffer.split('\n').collect();
    let mut min_day: Option<usize> = None;
    let mut min_tmp_spread: isize = isize::MAX;

    // 2.1. start on 2 as to ignore the header and the blank line
    // 2.2. end before the last line as the last line is special and not a day
    for line in &lines[2..lines.len() - 2] {
        let words: Vec<&str> = line.split_whitespace().collect();

        // 3. look only at 0th and 2nd elements
        let day = words
            .first()
            .expect("day was not present")
            .parse::<usize>()
            .expect("could not parse day");
        // 3.1. sometimes temperates have a * next to them for some reason
        let hi = words
            .get(1)
            .expect("temp was not present")
            .replace('*', "")
            .parse::<isize>()
            .expect("could not parse temp");
        let lo = words
            .get(2)
            .expect("temp was not present")
            .replace('*', "")
            .parse::<isize>()
            .expect("could not parse temp");
        let spread = hi - lo;

        // 4. keep track of min 2nd and which day it was for
        if spread < min_tmp_spread {
            min_day = Some(day);
            min_tmp_spread = spread;
        }
    }

    // 5. return min day or none if there were no days present
    if let Some(day) = min_day {
        println!(
            "The minimum temperature spread was {} degrees, which was on day {}.",
            min_tmp_spread, day
        );
    } else {
        println!("No days were present in the file.");
    }
    Ok(())
}

use std::time::Instant;

mod common;
mod solutions;

fn main() {
    let _ = dotenvy::dotenv();

    let args = std::env::args().collect::<Vec<_>>();

    let day = args
        .get(1)
        .and_then(|n| n.parse::<u8>().ok())
        .expect("day is not a number");

    let part = args
        .get(2)
        .and_then(|n| n.chars().next())
        .expect("part is not a letter");

    let Some(solution) = solutions::SOLUTIONS.get((day - 1) as usize) else {
        eprintln!("[-] Day {} not implemented", day);
        return;
    };

    let input = load(day).unwrap();

    println!(
        "[*] Running: {} ({}-{})",
        solution.name(),
        day,
        part.to_uppercase()
    );

    let start = Instant::now();
    let out = match part.to_ascii_lowercase() {
        'a' => solution.part_a(&input),
        'b' => solution.part_b(&input),
        _ => return eprintln!("[-] Invalid Part {}", part),
    };

    let time = start.elapsed().as_nanos();
    println!("[*] Out: {} (took {})", out, format_time(time));
}

fn load(day: u8) -> Result<String, Box<dyn std::error::Error>> {
    let token = std::env::var("TOKEN").expect("TOKEN is not set");

    Ok(
        ureq::get(format!("https://adventofcode.com/2023/day/{day}/input").as_str())
            .set("Cookie", format!("session={token}").as_str())
            .call()?
            .into_string()?
            .trim()
            .replace('\r', ""),
    )
}

fn format_time(nanos: u128) -> String {
    const TIME_UNITS: &[&str] = &["ns", "μs", "ms", "s"];

    let mut time = nanos;
    for i in TIME_UNITS {
        if time < 1000 {
            return format!("{}{}", time, i);
        }
        time /= 1000;
    }

    format!("{}{}", time, TIME_UNITS.last().unwrap())
}

fn main() {
    println!("Enter `mm:ss` lines. Ctrl-Z to finish");

    let mut total = 0;
    use std::io::BufRead as _;
    let sum = std::io::stdin()
        .lock()
        .lines()
        .flatten()
        .flat_map(|s| {
            let s = s.trim();
            if s.is_empty() {
                return None;
            }

            let mut t = s.split(':').flat_map(|s| s.parse::<u64>().ok());
            let (mins, secs) = (t.next()?, t.next()?);
            total += 1;
            Some(secs + (mins * 60))
        })
        .sum::<u64>();

    println!("\ntotal entries: {}", total);

    let (hours, minutes, seconds) = (sum / (60 * 60), (sum / 60) % 60, sum % 60);
    if hours > 0 {
        println!("{}h {}m {}s", hours, minutes, seconds)
    } else {
        println!("{}m {}s", minutes, seconds)
    }
}

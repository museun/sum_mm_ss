fn main() {
    println!("Enter `mm:ss` lines. Ctrl-Z to finish");

    use std::io::BufRead as _;
    let (total, sum) = std::io::stdin()
        .lock()
        .lines()
        .flatten()
        .flat_map(|s| {
            Some(s.trim()).filter(|s| !s.is_empty()).and_then(|s| {
                let mut t = s.split(':').flat_map(|s| s.parse::<u64>().ok());
                let (mins, secs) = (t.next()?, t.next()?);
                Some(secs + (mins * 60))
            })
        })
        .fold((0, 0), |(total, sum), c| (total + 1, sum + c));

    println!("\ntotal entries: {}", total);

    let (hours, minutes, seconds) = (sum / (60 * 60), (sum / 60) % 60, sum % 60);
    if hours > 0 {
        println!("{}h {}m {}s", hours, minutes, seconds)
    } else {
        println!("{}m {}s", minutes, seconds)
    }
}

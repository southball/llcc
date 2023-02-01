pub fn display_error(source: &str, position: usize, message: &str) {
    let mut line_number = 1usize;
    let mut accumulated = 0usize;
    for line in source.lines() {
        if position <= accumulated + line.len() {
            println!("Error:");
            println!("{} | {}", line_number, line);
            println!(
                "{} | {}^ {}",
                " ".repeat(line_number.to_string().len()),
                " ".repeat(position - accumulated),
                message
            );
        }
        line_number += 1;
        accumulated += line.len() + 1;
    }
}

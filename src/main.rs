fn reverse(str: String) -> String {
    str.chars().rev().collect()
}

fn main() {
    let mut entry = String::new();
    println!("Please enter a string to be reversed.");
    match std::io::stdin().read_line(&mut entry) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("{:?}", e);
            std::process::exit(1);
        }
    }
    let output = reverse(entry);
    println!("Reverse order : {}", output);
}

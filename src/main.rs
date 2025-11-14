const ADDRESS: &str = "8.8.8.8";

fn main() {
    match ping::new(ADDRESS.parse().unwrap()).send() {
        Ok(_) => print!("Successfully pinged {ADDRESS}\n"),
        Err(e) => print!("Could not ping {ADDRESS} => {e}\n"),
    }
}

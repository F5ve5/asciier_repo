pub fn retrieve() -> u8{
    let choice: u8 = std::env::args()
    .nth(1)
    .expect("No argument provided")
    .parse()
    .expect("Invalid input");

    choice
}
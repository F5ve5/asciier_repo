pub fn retrieve() -> u8{
    (
    std::env::args()
        .nth(1)
        .and_then(|arg| arg.parse::<i32>().ok())
        .unwrap_or(0)
    ) as u8
}
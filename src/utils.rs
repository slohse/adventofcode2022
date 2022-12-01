
pub fn i64_or_bust(input: &str) -> i64 {
    let num = i64::from_str_radix(input, 10);
    match num {
        Err(msg) => println!("found non-number {} in input, error: {}", input, msg),
        Ok(d) => return d,
    }
    return 0;
}



pub fn u8_to_sight_name(sight: u8) -> String {
    match sight {
        0 => String::from("Iron Sights"),
        1 => String::from("Any 1.0x Sight variant"),
        2 => String::from("Scope 1.5x"),
        3 => String::from("Scope 2.0x"),
        4 => String::from("Scope 2.5x"),
        5 => String::from("Scope 3.0x"),
        _ => format!("Unkown sight: {}", sight)
    }
}
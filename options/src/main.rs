fn main() {
    let ice_creams = maybe_ice_cream(0).unwrap();

    println!("{}", ice_creams);
}

fn maybe_ice_cream(hour_of_day: u16) -> Option<u16> {
    match hour_of_day {
        0..=21 => Some(5),
        22..=23 => Some(0), // needed equals sign here to make inclusive ugh
        _ => None,
    }
}
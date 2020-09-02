use rustler::TermType;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

// pub struct DateTime {
//     sign: Sign,
//     year: u8,
//     month: u8,
//     day: u8,
//     hour: u8,
//     minute: u8,
//     second: Decimal,
//     offset: OffSet,
//     string_value: String,
// }

// pub struct OffSet {
//     sign: Sign,
//     hour: u8,
//     min: u8,
// }

// pub struct Decimal {
//     int: i64,
//     scf: u32,
// }

// pub enum Sign {
//     Plus,
//     Minus,
// }

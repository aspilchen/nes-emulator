pub struct Sweep {
    enabled: bool,
    divider: u8,
    period: u8, // divider reload value
    negate: bool,
    shift: u8,
    reload: bool, // set on register write
}

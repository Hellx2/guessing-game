pub fn read_i32(x: &String) -> i32 {
    let mut buf: i32 = 0;
    for i in x.as_bytes() {
        if (0x30u8..0x40u8).contains(i) {
            buf *= 10;
            buf += i32::from(i - 0x30u8);
        } else if i != &10u8 {
            eprintln!("\u{001b}[31;1mError:\u{001b}[0m Invalid character '{}'.", i);
            std::process::exit(1);
        }
    }
    buf
}

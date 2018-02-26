#[macro_use]
extern crate bitflags;

bitflags! {
    struct Flags: u32 {
        const X = 0b00000001;
        const Y = 0b00000010;
    }
}

pub trait Format {
    fn decimal(&self);
}

impl Format for Flags {
    fn decimal(&self) {
        println!("Decimal: {}", self.bits());
    }
}

fn main() {
    let flags = Flags::X | Flags::Y;
    flags.decimal();
    (Flags::X | Flags::Y).decimal();
    (Flags::Y).decimal();

    println!("Current state: {:?}", Flags::all());
    println!("Contains X? {:?}", flags.contains(Flags::X));
}

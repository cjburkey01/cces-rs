#[macro_use]
extern crate num_derive;

use enum_iterator::IntoEnumIterator;
use num_traits::{FromPrimitive, ToPrimitive};
use std::convert::{TryFrom, TryInto};

#[derive(Debug, Copy, Clone, Eq, PartialEq, FromPrimitive, ToPrimitive, IntoEnumIterator)]
enum Rotation {
    /// Clockwise from the top
    Right = 0b0000001,

    /// Counter-clockwise from the top
    Left = 0b00000010,

    /// 180 rotation
    Double = 0b00000011,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, FromPrimitive, ToPrimitive, IntoEnumIterator)]
/// The most significant four bits represent the unique ID of the instruction,
/// the least significant four bits represent the count of arguments for the instruction.
#[repr(u8)]
enum Instruction {
    /// Do nothing this tick
    None = 0b00010000,

    /// Move forward 1 or 2 tiles depending on the argument.
    Move = 0b00100001,

    /// Rotate according to the value in the next instruction byte.
    /// The value is mapped to a `Rotation` and nothing will be
    /// performed if the value is invalid.
    Rotate = 0b00110001,
}

impl Instruction {
    /// When 1, the highest bit will instruct the parser to pop
    /// the next byte as it is the argument for that instruction.
    const ARG_BITS: u8 = 0b1111;

    pub fn get_args(&self) -> usize {
        let inst: u8 = (*self).try_into().expect("failed to get instruction bits");
        (inst & Self::ARG_BITS).into()
    }
}

macro_rules! impl_converts {
    ($type:ty) => {
        impl TryFrom<$type> for u8 {
            type Error = ();

            fn try_from(value: $type) -> Result<Self, Self::Error> {
                value.to_u8().map_or(Err(()), |val| Ok(val))
            }
        }
        impl TryFrom<u8> for $type {
            type Error = ();

            fn try_from(value: u8) -> Result<Self, Self::Error> {
                <$type>::from_u8(value).map_or(Err(()), |val| Ok(val))
            }
        }
    };
}

impl_converts!(Rotation);
impl_converts!(Instruction);

fn main() {
    println!("Instructions: {{");

    for instruction in Instruction::into_enum_iter() {
        println!(
            "  [{:04b}] [{}] {:?}",
            (instruction.to_u8().unwrap_or(0b00000000) >> 4),
            instruction.get_args(),
            instruction,
        );
    }

    println!("}}");
}

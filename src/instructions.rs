use enum_iterator::IntoEnumIterator;
use num_traits::{FromPrimitive, ToPrimitive};
use std::convert::{TryFrom, TryInto};

/// Represents an instruction that a creature tries to execute.
///
/// The most significant six bits represent the unique ID of the instruction and the least
/// significant two bits represent the count of arguments for the instruction. Therefore,
/// instructions may take 0, 1, 2, or 3 arguments (00, 01, 10, 11).
///
/// There may also be different variants of the same instruction, such as move with no argument
/// being different than a move with one argument.
#[derive(Debug, Copy, Clone, Eq, PartialEq, FromPrimitive, ToPrimitive, IntoEnumIterator)]
#[repr(u8)]
pub enum Instruction {
    //  Movement
    /// Do nothing this tick
    None = 0b000000_00,
    /// Move forward 1 tile (will not move if unsuccessful).
    Move = 0b000001_00,
    /// Try to forward 2 tiles (will only move 1 if unsuccessful).
    Jump = 0b000010_00,
    /// Rotate clockwise (relative to looking down).
    RotateCW = 0b000100_00,
    /// Rotate counter-clockwise (relative to looking down).
    RotateCCW = 0b000110_00,

    //  Logic
    /// Set mem A to 0.
    ClearA = 0b001000_00,
    /// Set mem B to 0.
    ClearB = 0b001001_00,
    /// Jump to the byte of DNA provided as an argument (loops back to start above length).
    Goto = 0b010000_01,
    /// If mem A is greater than the value in mem B, perform a jump to the argument.
    GotoCondAGtB = 0b011000_01,
    /// If mem B is equal to mem A, perform a jump to the argument.
    GotoCondEq = 0b011001_01,
    /// Switch mem A and mem B.
    SwapAB = 0b010011_00,
    /// Copy and clear the value of mem A into mem TMP.
    StoreATmp = 0b010100_00,
    /// Copy and clear the value of mem B into mem TMP.
    StoreBTmp = 0b010101_00,
    /// Copy and clear the value of mem TMP into mem A.
    LoadTmpA = 0b010110_00,
    /// Copy and clear the value of mem TMP into mem B.
    LoadTmpB = 0b010111_00,

    /// Load current health (0-max) into mem TMP.
    StoreHealthTmp = 0b010000_00,
    /// Load current fullness (0-max) into mem TMP.
    StoreHungerTmp = 0b010001_00,
    /// Load line of sight color hex (3 bytes) into mem TMP.
    StoreLOSCTmp = 0b010010_00,

    //  Math
    /// Add the provided unsigned byte integer to signed mem A.
    UAddA = 0b100000_01,
    /// Add the provided unsigned byte integer to signed mem B.
    UAddB = 0b100001_01,
    /// Add the provided signed byte integer to signed mem A.
    IAddA = 0b100010_01,
    /// Add the provided signed byte integer to signed mem B.
    IAddB = 0b100011_01,
    /// Bitwise AND with A and the argument put into A.
    BitAndATmp = 0b110000_01,
    /// Bitwise AND with B and the argument put into B.
    BitAndBTmp = 0b110001_01,
    /// Bitwise AND with A and the argument put into A.
    BitOrATmp = 0b111000_01,
    /// Bitwise AND with B and the argument put into B.
    BitOrBTmp = 0b111001_01,
    /// Bitwise X-OR with A and the argument put into A.
    BitXorATmp = 0b111100_01,
    /// Bitwise X-OR with B and the argument put into B.
    BitXorBTmp = 0b111101_01,
}

impl Instruction {
    /// When 1, the highest bit will instruct the parser to pop the next byte as it is the argument
    /// for that instruction.
    const ARG_BIT_MASK: u8 = 0b000000_11;

    // `Instruction` implements copy so the value is not moved with this invocation
    pub fn get_args(self) -> usize {
        // Get the instruction bytes
        let inst = self.try_into().unwrap_or(0u8);

        // Get the last two bits of the instruction.
        // These two bits represent the 0-3 byte argument requirement
        (inst & Self::ARG_BIT_MASK).into()
    }
}

/// Implements the converters to and from bytes into the given enum type
macro_rules! impl_converts {
    ($type:ty) => {
        // Converts a byte to an instruction
        impl TryFrom<$type> for u8 {
            type Error = ();

            fn try_from(value: $type) -> Result<Self, Self::Error> {
                value.to_u8().map_or(Err(()), |val| Ok(val))
            }
        }

        // Converts an instruction into a byte
        impl TryFrom<u8> for $type {
            type Error = ();

            fn try_from(value: u8) -> Result<Self, Self::Error> {
                <$type>::from_u8(value).map_or(Err(()), |val| Ok(val))
            }
        }
    };
}

// Implement enum<->byte conversions
impl_converts!(Instruction);

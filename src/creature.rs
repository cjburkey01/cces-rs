use crate::processor::ProcessorMemory;
use enum_iterator::IntoEnumIterator;
use num_traits::{FromPrimitive, ToPrimitive};
use specs::prelude::*;
use specs::Component;
use std::convert::{TryFrom, TryInto};

/// Creates a creature with the given instructions
pub macro create_creature($($inst: ident),*) {
    crate::creature::Creature::new_with(vec![
        $(crate::creature::Instruction::$inst.try_into().unwrap(),)*
    ])
}

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
    RotateCW = 0b000011_00,
    /// Rotate counter-clockwise (relative to looking down).
    RotateCCW = 0b000100_00,

    //  Logic
    /// Set mem A to 0.
    ClearA = 0b000101_00,
    /// Set mem B to 0.
    ClearB = 0b000110_00,
    /// Jump to the byte of DNA provided as an argument (loops back to start above length).
    Goto = 0b000111_01,
    /// If mem A is greater than the value in mem B, perform a jump to the argument.
    GotoCondAGtB = 0b001000_01,
    /// If mem B is equal to mem A, perform a jump to the argument.
    GotoCondEq = 0b001001_01,
    /// Switch mem A and mem B.
    SwapAB = 0b001010_00,
    /// Copy the value of mem A into mem TMP (but don't clear A)
    CopyATmp = 0b001011_00,
    /// Copy and clear the value of mem TMP into mem A.
    LoadTmpA = 0b001100_00,
    /// Copy and clear the value of mem TMP into mem B.
    LoadTmpB = 0b001101_00,

    // Creature
    /// Load current health (0-max) into mem TMP.
    StoreHealthTmp = 0b001110_00,
    /// Load current fullness (0-max) into mem TMP.
    StoreHungerTmp = 0b001111_00,
    /// Load current fullness (0-max) into mem TMP.
    StoreWasteTmp = 0b010110_00,
    /// Load line of sight color hex (3 bytes) into mem TMP.
    StoreLOSCTmp = 0b010000_00,

    //  Math
    /// Add the provided unsigned byte integer to signed mem A.
    UAdd = 0b010001_01,
    /// Add the provided signed byte integer to signed mem A.
    IAdd = 0b010010_01,
    /// Bitwise AND with A and B put into A.
    BitAndB = 0b010011_00,
    /// Bitwise AND with A and the argument put into A.
    BitAnd = 0b010011_01,
    /// Bitwise AND with A and B put into A.
    BitOrB = 0b010100_00,
    /// Bitwise AND with A and the argument put into A.
    BitOr = 0b010100_01,
    /// Bitwise X-OR with A and B put into A.
    BitXorB = 0b010101_00,
    /// Bitwise X-OR with A and the argument put into A.
    BitXor = 0b010101_01,
}

impl Instruction {
    /// Bitmask to get the arguments from the instruction
    const ARG_BIT_MASK: u8 = 0b000000_11;
}

impl crate::processor::Instruction<u8> for Instruction {
    // `Instruction` implements copy so the value is not moved with this invocation
    fn get_args(self) -> usize {
        // Get the instruction byte
        let inst = self.try_into().unwrap_or(0u8);

        // Get the last two bits of the instruction.
        // These two bits represent the 0-3 byte argument requirement
        (inst & Self::ARG_BIT_MASK).into()
    }
}

// Converts a byte to an instruction
impl TryFrom<Instruction> for u8 {
    type Error = ();

    fn try_from(value: Instruction) -> Result<Self, Self::Error> {
        value.to_u8().map_or(Err(()), |val| Ok(val))
    }
}

// Converts an instruction into a byte
impl TryFrom<u8> for Instruction {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value).map_or(Err(()), |val| Ok(val))
    }
}

/// Represents the values in the creature's memory
#[derive(Debug, Clone, PartialEq, Eq, Component)]
#[storage(VecStorage)]
pub struct CreatureMemory {
    /// Mem A
    a: u64,

    /// Mem B
    b: u64,

    /// Mem TMP
    tmp: u64,
}

impl CreatureMemory {
    #[inline(always)]
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for CreatureMemory {
    fn default() -> Self {
        Self { a: 0, b: 0, tmp: 0 }
    }
}

impl ProcessorMemory<u64> for CreatureMemory {
    fn get_mem_a(&self) -> u64 {
        self.a
    }

    fn get_mem_b(&self) -> u64 {
        self.b
    }

    fn get_mem_tmp(&self) -> u64 {
        self.tmp
    }

    fn set_mem_a(&mut self, value: u64) {
        self.a = value;
    }

    fn set_mem_b(&mut self, value: u64) {
        self.b = value;
    }

    fn set_mem_tmp(&mut self, value: u64) {
        self.tmp = value;
    }
}

#[derive(Debug, Clone, Component)]
pub struct Creature {
    cycle: u64,
    current: u64,
    instructions: Vec<u8>,
}

impl Creature {
    #[inline(always)]
    pub fn new() -> Self {
        Self::new_with(vec![])
    }

    #[inline(always)]
    pub fn new_with(instructions: Vec<u8>) -> Self {
        Self {
            cycle: 0,
            current: 0,
            instructions,
        }
    }

    pub fn get_instructions(&self) -> &Vec<u8> {
        &self.instructions
    }

    pub fn get_instructions_mut(&mut self) -> &mut Vec<u8> {
        &mut self.instructions
    }
}

impl Default for Creature {
    fn default() -> Self {
        Self::new()
    }
}

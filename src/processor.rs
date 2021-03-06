use num_traits::PrimInt;
use std::convert::{TryFrom, TryInto};
use std::fmt::Debug;
use std::marker::PhantomData;

/// A struct that will make structs with fields of this type constructable only within this module.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct _Private;

/// Contains instruction execution memory data.
/// Copy is not required to be implemented because this type is passed as a reference.
pub trait ProcessorMemory<MemType: PrimInt>: Debug {
    /// Returns the value of mem A.
    fn get_mem_a(&self) -> MemType;

    /// Returns the value of mem B.
    fn get_mem_b(&self) -> MemType;

    /// Returns the value of mem TMP.
    fn get_mem_tmp(&self) -> MemType;

    /// Updates the value of mem A.
    fn set_mem_a(&mut self, value: MemType);

    /// Updates the value of mem B.
    fn set_mem_b(&mut self, value: MemType);

    /// Updates the value of mem TMP.
    fn set_mem_tmp(&mut self, value: MemType);
}

/// Represents a single instruction
pub trait Instruction<Prim: PrimInt>: Debug + Copy + TryInto<Prim> + TryFrom<Prim> {
    /// Returns the number of arguments that this instruction expects
    fn get_args(self) -> usize;
}

/// Represents a call to an instruction with optional supplied arguments.
/// This *should not* be constructed without using one of the `new` functions.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct InstructionCall<
    InstType: PrimInt,
    Inst: Instruction<InstType>,
    ArgType: Clone + PartialEq,
> {
    /// The instruction to be executed
    instruction: Inst,

    /// The optional arguments passed to the caller for this instruction
    args: (Option<ArgType>, Option<ArgType>, Option<ArgType>),

    /// Dummy data
    _inst_type: PhantomData<InstType>,

    /// Keeps this struct instantiatable only within this module.
    _private: _Private,
}

impl<InstType: PrimInt, Inst: Instruction<InstType>, ArgType: Clone + PartialEq>
    InstructionCall<InstType, Inst, ArgType>
{
    /// Creates an instruction call _without_ verifying the argument count with the instruction
    /// requirements.
    #[inline(always)]
    fn new_raw(
        instruction: Inst,
        arg1: Option<ArgType>,
        arg2: Option<ArgType>,
        arg3: Option<ArgType>,
    ) -> Self {
        Self {
            instruction,
            args: (arg1, arg2, arg3),
            _inst_type: PhantomData,
            _private: _Private,
        }
    }

    /// Wraps an instruction with 3 arguments.
    /// Returns `Err(())` if the instruction has a different number of arguments required.
    #[inline(always)]
    pub fn new_3_arg(
        instruction: Inst,
        arg1: ArgType,
        arg2: ArgType,
        arg3: ArgType,
    ) -> Result<Self, ()> {
        if instruction.get_args() != 3 {
            return Err(());
        }
        Ok(Self::new_raw(
            instruction,
            Some(arg1),
            Some(arg2),
            Some(arg3),
        ))
    }

    /// Wraps an instruction with 2 arguments.
    /// Returns `Err(())` if the instruction has a different number of arguments required.
    #[inline(always)]
    pub fn new_2_arg(instruction: Inst, arg1: ArgType, arg2: ArgType) -> Result<Self, ()> {
        if instruction.get_args() != 2 {
            return Err(());
        }
        Ok(Self::new_raw(instruction, Some(arg1), Some(arg2), None))
    }

    /// Wraps an instruction with 1 argument.
    /// Returns `Err(())` if the instruction has a different number of arguments required.
    #[inline(always)]
    pub fn new_1_arg(instruction: Inst, arg1: ArgType) -> Result<Self, ()> {
        if instruction.get_args() != 1 {
            return Err(());
        }
        Ok(Self::new_raw(instruction, Some(arg1), None, None))
    }

    /// Wraps an instruction with no arguments.
    /// Returns `Err(())` if the instruction has a different number of arguments required.
    #[inline(always)]
    pub fn new_0_arg(instruction: Inst) -> Result<Self, ()> {
        if instruction.get_args() != 0 {
            return Err(());
        }
        Ok(Self::new_raw(instruction, None, None, None))
    }
}

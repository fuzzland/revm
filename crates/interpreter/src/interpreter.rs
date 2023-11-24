pub mod analysis;
mod contract;
pub(crate) mod memory;
mod stack;

pub use analysis::BytecodeLocked;
pub use contract::Contract;
pub use memory::Memory;
pub use stack::Stack;

use crate::primitives::{Bytes, Spec};
use crate::{
    instructions::{eval, InstructionResult},
    Gas, Host,
};
use core::ops::Range;

pub const STACK_LIMIT: u64 = 1024;
pub const CALL_STACK_LIMIT: u64 = 1024;

// This is a carefully considered value. Eg. very complex tx:
// https://bscscan.com/tx/0xae8ca9dc8258ae32899fe641985739c3fa53ab1f603973ac74b424e165c66ccf whose instruction count is 4_752_748
// https://bscscan.com/tx/0x3b472f87431a52082bae7d8524b4e0af3cf930a105646259e1249f2218525607 whose instruction count is 2_230_341
pub const MAX_INSTRUCTION_SIZE: usize = 800_000;

/// EIP-170: Contract code size limit
/// By default limit is 0x6000 (~25kb)
pub const MAX_CODE_SIZE: usize = 0x6000;
/// EIP-3860: Limit and meter initcode
pub const MAX_INITCODE_SIZE: usize = 2 * MAX_CODE_SIZE;

pub struct Interpreter {
    /// Instruction pointer.
    pub instruction_pointer: *const u8,
    /// Return is main control flag, it tell us if we should continue interpreter or break from it
    pub instruction_result: InstructionResult,
    /// left gas. Memory gas can be found in Memory field.
    pub gas: Gas,
    /// Memory.
    pub memory: Memory,
    /// Stack.
    pub stack: Stack,
    /// After call returns, its return data is saved here.
    pub return_data_buffer: Bytes,
    /// Return value.
    pub return_range: Range<usize>,
    /// Is interpreter call static.
    pub is_static: bool,
    /// Contract information and invoking data
    pub contract: Contract,
    /// Memory limit. See [`crate::CfgEnv`].
    #[cfg(feature = "memory_limit")]
    pub memory_limit: u64,
}

impl Interpreter {
    /// Current opcode
    pub fn current_opcode(&self) -> u8 {
        unsafe { *self.instruction_pointer }
    }

    /// Create new interpreter
    pub fn new(contract: Contract, gas_limit: u64, is_static: bool) -> Self {
        #[cfg(not(feature = "memory_limit"))]
        {
            Self {
                instruction_pointer: contract.bytecode.as_ptr(),
                return_range: Range::default(),
                memory: Memory::new(),
                stack: Stack::new(),
                return_data_buffer: Bytes::new(),
                contract,
                instruction_result: InstructionResult::Continue,
                is_static,
                gas: Gas::new(gas_limit),
            }
        }

        #[cfg(feature = "memory_limit")]
        {
            Self::new_with_memory_limit(contract, gas_limit, is_static, u64::MAX)
        }
    }

    #[cfg(feature = "memory_limit")]
    pub fn new_with_memory_limit(
        contract: Contract,
        gas_limit: u64,
        is_static: bool,
        memory_limit: u64,
    ) -> Self {
        Self {
            instruction_pointer: contract.bytecode.as_ptr(),
            return_range: Range::default(),
            memory: Memory::new(),
            stack: Stack::new(),
            return_data_buffer: Bytes::new(),
            contract,
            instruction_result: InstructionResult::Continue,
            is_static,
            gas: Gas::new(gas_limit),
            memory_limit,
        }
    }

    pub fn contract(&self) -> &Contract {
        &self.contract
    }

    pub fn gas(&self) -> &Gas {
        &self.gas
    }

    /// Reference of interpreter memory.
    pub fn memory(&self) -> &Memory {
        &self.memory
    }

    /// Reference of interpreter stack.
    pub fn stack(&self) -> &Stack {
        &self.stack
    }

    /// Return a reference of the program counter.
    pub fn program_counter(&self) -> usize {
        // Safety: this is just subtraction of pointers, it is safe to do.
        unsafe {
            self.instruction_pointer
                .offset_from(self.contract.bytecode.as_ptr()) as usize
        }
    }

    /// Execute next instruction
    #[inline(always)]
    pub fn step<T, H: Host<T>, SPEC: Spec>(&mut self, host: &mut H, additional_data: &mut T) {
        // step.
        let opcode = unsafe { *self.instruction_pointer };
        // Safety: In analysis we are doing padding of bytecode so that we are sure that last
        // byte instruction is STOP so we are safe to just increment program_counter bcs on last instruction
        // it will do noop and just stop execution of this contract
        self.instruction_pointer = unsafe { self.instruction_pointer.offset(1) };
        eval::<T, H, SPEC>(opcode, self, host, additional_data);
    }

    /// loop steps until we are finished with execution
    pub fn run<T, H: Host<T>, SPEC: Spec>(
        &mut self,
        host: &mut H,
        additional_data: &mut T,
    ) -> InstructionResult {
        while self.instruction_result == InstructionResult::Continue {
            self.step::<T, H, SPEC>(host, additional_data);
        }
        self.instruction_result
    }

    /// loop steps until we are finished with execution
    pub fn run_inspect<T, H: Host<T>, SPEC: Spec>(
        &mut self,
        host: &mut H,
        additional_data: &mut T,
    ) -> InstructionResult {
        for _count in 0..MAX_INSTRUCTION_SIZE {
            if self.instruction_result == InstructionResult::Continue {
                host.step(self, additional_data);
                self.step::<T, H, SPEC>(host, additional_data);
            } else {
                break;
            }
        }
        self.instruction_result
    }

    /// Copy and get the return value of the interpreter, if any.
    pub fn return_value(&self) -> Bytes {
        // if start is usize max it means that our return len is zero and we need to return empty
        if self.return_range.start == usize::MAX {
            Bytes::new()
        } else {
            Bytes::copy_from_slice(self.memory.get_slice(
                self.return_range.start,
                self.return_range.end - self.return_range.start,
            ))
        }
    }
}

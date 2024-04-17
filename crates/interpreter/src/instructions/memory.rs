use std::cmp::max;

use crate::{gas, interpreter::Interpreter, primitives::U256, Host, InstructionResult};

pub fn mload<T>(interpreter: &mut Interpreter, _host: &mut dyn Host<T>) {
    gas!(interpreter, gas::VERYLOW);
    pop!(interpreter, index);
    let index = as_usize_or_fail!(interpreter, index, InstructionResult::InvalidOperandOOG);
    memory_resize!(interpreter, index, 32);
    push!(
        interpreter,
        U256::from_be_bytes::<{ U256::BYTES }>(
            interpreter.memory.get_slice(index, 32).try_into().unwrap()
        )
    );
}

pub fn mstore<T>(interpreter: &mut Interpreter, _host: &mut dyn Host<T>) {
    gas!(interpreter, gas::VERYLOW);
    pop!(interpreter, index, value);
    let index = as_usize_or_fail!(interpreter, index, InstructionResult::InvalidOperandOOG);
    memory_resize!(interpreter, index, 32);
    interpreter.memory.set_u256(index, value);
}

pub fn mstore8<T>(interpreter: &mut Interpreter, _host: &mut dyn Host<T>) {
    gas!(interpreter, gas::VERYLOW);
    pop!(interpreter, index, value);
    let index = as_usize_or_fail!(interpreter, index, InstructionResult::InvalidOperandOOG);
    memory_resize!(interpreter, index, 1);
    let value = value.as_le_bytes()[0];
    // Safety: we resized our memory two lines above.
    unsafe { interpreter.memory.set_byte(index, value) }
}

pub fn msize<T>(interpreter: &mut Interpreter, _host: &mut dyn Host<T>) {
    gas!(interpreter, gas::BASE);
    push!(interpreter, U256::from(interpreter.memory.effective_len()));
}

// EIP-5656: MCOPY - Memory copying instruction
pub fn mcopy<T>(interpreter: &mut Interpreter, _host: &mut dyn Host<T>) {
    // check!(interpreter, SPEC::enabled(CANCUN));
    pop!(interpreter, dst, src, len);

    // into usize or fail
    let len = as_usize_or_fail!(interpreter, len);
    // deduce gas
    gas_or_fail!(interpreter, gas::verylowcopy_cost(len as u64));
    if len == 0 {
        return;
    }

    let dst = as_usize_or_fail!(interpreter, dst);
    let src = as_usize_or_fail!(interpreter, src);
    // resize memory
    memory_resize!(interpreter, max(dst, src), len);
    // copy memory in place
    // interpreter.shared_memory.copy(dst, src, len);
    interpreter.memory.copy(dst, src, len);
}

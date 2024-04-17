#[macro_use]
mod macros;
mod arithmetic;
mod bitwise;
mod control;
mod host;
mod host_env;
mod i256;
mod memory;
pub mod opcode;
mod stack;
mod system;

use crate::{interpreter::Interpreter, primitives::Spec, Host};
pub use opcode::{OpCode, OPCODE_JUMPMAP};

pub use crate::{return_ok, return_revert, InstructionResult};
pub fn return_stop<T>(interpreter: &mut Interpreter, _host: &mut dyn Host<T>) {
    interpreter.instruction_result = InstructionResult::Stop;
}
pub fn return_invalid<T>(interpreter: &mut Interpreter, _host: &mut dyn Host<T>) {
    interpreter.instruction_result = InstructionResult::InvalidFEOpcode;
}

pub fn return_not_found<T>(interpreter: &mut Interpreter, _host: &mut dyn Host<T>) {
    interpreter.instruction_result = InstructionResult::OpcodeNotFound;
}

#[inline(always)]
pub fn eval<T, H: Host<T>, S: Spec>(
    opcode: u8,
    interp: &mut Interpreter,
    host: &mut H,
    additional_data: &mut T,
) {
    match opcode {
        opcode::STOP => return_stop(interp, host),
        opcode::ADD => arithmetic::wrapped_add(interp, host),
        opcode::MUL => arithmetic::wrapping_mul(interp, host),
        opcode::SUB => arithmetic::wrapping_sub(interp, host),
        opcode::DIV => arithmetic::div(interp, host),
        opcode::SDIV => arithmetic::sdiv(interp, host),
        opcode::MOD => arithmetic::rem(interp, host),
        opcode::SMOD => arithmetic::smod(interp, host),
        opcode::ADDMOD => arithmetic::addmod(interp, host),
        opcode::MULMOD => arithmetic::mulmod(interp, host),
        opcode::EXP => arithmetic::eval_exp::<T, S>(interp, host),
        opcode::SIGNEXTEND => arithmetic::signextend(interp, host),
        opcode::LT => bitwise::lt(interp, host),
        opcode::GT => bitwise::gt(interp, host),
        opcode::SLT => bitwise::slt(interp, host),
        opcode::SGT => bitwise::sgt(interp, host),
        opcode::EQ => bitwise::eq(interp, host),
        opcode::ISZERO => bitwise::iszero(interp, host),
        opcode::AND => bitwise::bitand(interp, host),
        opcode::OR => bitwise::bitor(interp, host),
        opcode::XOR => bitwise::bitxor(interp, host),
        opcode::NOT => bitwise::not(interp, host),
        opcode::BYTE => bitwise::byte(interp, host),
        opcode::SHL => bitwise::shl::<T, S>(interp, host),
        opcode::SHR => bitwise::shr::<T, S>(interp, host),
        opcode::SAR => bitwise::sar::<T, S>(interp, host),
        opcode::SHA3 => system::sha3(interp, host),
        opcode::ADDRESS => system::address(interp, host),
        opcode::BALANCE => host::balance::<T, S>(interp, host),
        opcode::SELFBALANCE => host::selfbalance::<T, S>(interp, host),
        opcode::CODESIZE => system::codesize(interp, host),
        opcode::CODECOPY => system::codecopy(interp, host),
        opcode::CALLDATALOAD => system::calldataload(interp, host),
        opcode::CALLDATASIZE => system::calldatasize(interp, host),
        opcode::CALLDATACOPY => system::calldatacopy(interp, host),
        opcode::POP => stack::pop(interp, host),
        opcode::MLOAD => memory::mload(interp, host),
        opcode::MSTORE => memory::mstore(interp, host),
        opcode::MSTORE8 => memory::mstore8(interp, host),
        opcode::JUMP => control::jump(interp, host),
        opcode::JUMPI => control::jumpi(interp, host),
        opcode::PC => control::pc(interp, host),
        opcode::MSIZE => memory::msize(interp, host),
        opcode::JUMPDEST => control::jumpdest(interp, host),
        opcode::PUSH0 => stack::push0::<T, S>(interp, host),
        opcode::PUSH1 => stack::push::<T, 1>(interp, host),
        opcode::PUSH2 => stack::push::<T, 2>(interp, host),
        opcode::PUSH3 => stack::push::<T, 3>(interp, host),
        opcode::PUSH4 => stack::push::<T, 4>(interp, host),
        opcode::PUSH5 => stack::push::<T, 5>(interp, host),
        opcode::PUSH6 => stack::push::<T, 6>(interp, host),
        opcode::PUSH7 => stack::push::<T, 7>(interp, host),
        opcode::PUSH8 => stack::push::<T, 8>(interp, host),
        opcode::PUSH9 => stack::push::<T, 9>(interp, host),
        opcode::PUSH10 => stack::push::<T, 10>(interp, host),
        opcode::PUSH11 => stack::push::<T, 11>(interp, host),
        opcode::PUSH12 => stack::push::<T, 12>(interp, host),
        opcode::PUSH13 => stack::push::<T, 13>(interp, host),
        opcode::PUSH14 => stack::push::<T, 14>(interp, host),
        opcode::PUSH15 => stack::push::<T, 15>(interp, host),
        opcode::PUSH16 => stack::push::<T, 16>(interp, host),
        opcode::PUSH17 => stack::push::<T, 17>(interp, host),
        opcode::PUSH18 => stack::push::<T, 18>(interp, host),
        opcode::PUSH19 => stack::push::<T, 19>(interp, host),
        opcode::PUSH20 => stack::push::<T, 20>(interp, host),
        opcode::PUSH21 => stack::push::<T, 21>(interp, host),
        opcode::PUSH22 => stack::push::<T, 22>(interp, host),
        opcode::PUSH23 => stack::push::<T, 23>(interp, host),
        opcode::PUSH24 => stack::push::<T, 24>(interp, host),
        opcode::PUSH25 => stack::push::<T, 25>(interp, host),
        opcode::PUSH26 => stack::push::<T, 26>(interp, host),
        opcode::PUSH27 => stack::push::<T, 27>(interp, host),
        opcode::PUSH28 => stack::push::<T, 28>(interp, host),
        opcode::PUSH29 => stack::push::<T, 29>(interp, host),
        opcode::PUSH30 => stack::push::<T, 30>(interp, host),
        opcode::PUSH31 => stack::push::<T, 31>(interp, host),
        opcode::PUSH32 => stack::push::<T, 32>(interp, host),
        opcode::DUP1 => stack::dup::<T, 1>(interp, host),
        opcode::DUP2 => stack::dup::<T, 2>(interp, host),
        opcode::DUP3 => stack::dup::<T, 3>(interp, host),
        opcode::DUP4 => stack::dup::<T, 4>(interp, host),
        opcode::DUP5 => stack::dup::<T, 5>(interp, host),
        opcode::DUP6 => stack::dup::<T, 6>(interp, host),
        opcode::DUP7 => stack::dup::<T, 7>(interp, host),
        opcode::DUP8 => stack::dup::<T, 8>(interp, host),
        opcode::DUP9 => stack::dup::<T, 9>(interp, host),
        opcode::DUP10 => stack::dup::<T, 10>(interp, host),
        opcode::DUP11 => stack::dup::<T, 11>(interp, host),
        opcode::DUP12 => stack::dup::<T, 12>(interp, host),
        opcode::DUP13 => stack::dup::<T, 13>(interp, host),
        opcode::DUP14 => stack::dup::<T, 14>(interp, host),
        opcode::DUP15 => stack::dup::<T, 15>(interp, host),
        opcode::DUP16 => stack::dup::<T, 16>(interp, host),

        opcode::SWAP1 => stack::swap::<T, 1>(interp, host),
        opcode::SWAP2 => stack::swap::<T, 2>(interp, host),
        opcode::SWAP3 => stack::swap::<T, 3>(interp, host),
        opcode::SWAP4 => stack::swap::<T, 4>(interp, host),
        opcode::SWAP5 => stack::swap::<T, 5>(interp, host),
        opcode::SWAP6 => stack::swap::<T, 6>(interp, host),
        opcode::SWAP7 => stack::swap::<T, 7>(interp, host),
        opcode::SWAP8 => stack::swap::<T, 8>(interp, host),
        opcode::SWAP9 => stack::swap::<T, 9>(interp, host),
        opcode::SWAP10 => stack::swap::<T, 10>(interp, host),
        opcode::SWAP11 => stack::swap::<T, 11>(interp, host),
        opcode::SWAP12 => stack::swap::<T, 12>(interp, host),
        opcode::SWAP13 => stack::swap::<T, 13>(interp, host),
        opcode::SWAP14 => stack::swap::<T, 14>(interp, host),
        opcode::SWAP15 => stack::swap::<T, 15>(interp, host),
        opcode::SWAP16 => stack::swap::<T, 16>(interp, host),

        opcode::RETURN => control::ret(interp, host),
        opcode::REVERT => control::revert::<T, S>(interp, host),
        opcode::INVALID => return_invalid(interp, host),
        opcode::BASEFEE => host_env::basefee::<T, S>(interp, host),
        opcode::ORIGIN => host_env::origin(interp, host),
        opcode::CALLER => system::caller(interp, host),
        opcode::CALLVALUE => system::callvalue(interp, host),
        opcode::GASPRICE => host_env::gasprice(interp, host),
        opcode::EXTCODESIZE => host::extcodesize::<T, S>(interp, host),
        opcode::EXTCODEHASH => host::extcodehash::<T, S>(interp, host),
        opcode::EXTCODECOPY => host::extcodecopy::<T, S>(interp, host),
        opcode::RETURNDATASIZE => system::returndatasize::<T, S>(interp, host),
        opcode::RETURNDATACOPY => system::returndatacopy::<T, S>(interp, host),
        opcode::BLOCKHASH => host::blockhash(interp, host),
        opcode::COINBASE => host_env::coinbase(interp, host),
        opcode::TIMESTAMP => host_env::timestamp(interp, host),
        opcode::NUMBER => host_env::number(interp, host),
        opcode::DIFFICULTY => host_env::difficulty::<T, H, S>(interp, host),
        opcode::GASLIMIT => host_env::gaslimit(interp, host),
        opcode::SLOAD => host::sload::<T, S>(interp, host),
        opcode::SSTORE => host::sstore::<T, S>(interp, host),
        opcode::TLOAD => host::tload::<T, S>(interp, host),
        opcode::TSTORE => host::tstore::<T, S>(interp, host),
        opcode::MCOPY => memory::mcopy(interp, host),
        opcode::GAS => system::gas(interp, host),
        opcode::LOG0 => host::log::<T, 0>(interp, host),
        opcode::LOG1 => host::log::<T, 1>(interp, host),
        opcode::LOG2 => host::log::<T, 2>(interp, host),
        opcode::LOG3 => host::log::<T, 3>(interp, host),
        opcode::LOG4 => host::log::<T, 4>(interp, host),
        opcode::SELFDESTRUCT => host::selfdestruct::<T, S>(interp, host),
        opcode::CREATE => host::create::<T, false, S>(interp, host, additional_data), //check
        opcode::CREATE2 => host::create::<T, true, S>(interp, host, additional_data), //check
        opcode::CALL => host::call::<T, S>(interp, host, additional_data),            //check
        opcode::CALLCODE => host::call_code::<T, S>(interp, host, additional_data),   //check
        opcode::DELEGATECALL => host::delegate_call::<T, S>(interp, host, additional_data), //check
        opcode::STATICCALL => host::static_call::<T, S>(interp, host, additional_data), //check
        opcode::CHAINID => host_env::chainid::<T, S>(interp, host),
        _ => return_not_found(interp, host),
    }
}

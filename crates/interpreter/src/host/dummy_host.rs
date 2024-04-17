use crate::primitives::{hash_map::Entry, Bytecode, Bytes, HashMap, U256};
use crate::{
    primitives::{Env, Log, B160, B256, KECCAK_EMPTY},
    BytecodeLocked, CallInputs, CreateInputs, Gas, Host, InstructionResult, Interpreter,
    SelfDestructResult,
};
use alloc::vec::Vec;
use std::sync::Arc;

pub struct DummyHost {
    pub env: Env,
    pub storage: HashMap<U256, U256>,
    pub transient_storage: HashMap<U256, U256>,
    pub log: Vec<Log>,
}

impl DummyHost {
    pub fn new(env: Env) -> Self {
        Self {
            env,
            storage: HashMap::new(),
            transient_storage: HashMap::new(),
            log: Vec::new(),
        }
    }
    pub fn clear(&mut self) {
        self.storage.clear();
        self.log.clear();
    }
}

impl Host<u32> for DummyHost {
    fn step(&mut self, _interp: &mut Interpreter, _: &mut u32) -> InstructionResult {
        InstructionResult::Continue
    }

    fn step_end(
        &mut self,
        _interp: &mut Interpreter,
        _ret: InstructionResult,
        _: &mut u32,
    ) -> InstructionResult {
        InstructionResult::Continue
    }

    fn env(&mut self) -> &mut Env {
        &mut self.env
    }

    fn load_account(&mut self, _address: B160) -> Option<(bool, bool)> {
        Some((true, true))
    }

    fn block_hash(&mut self, _number: U256) -> Option<B256> {
        Some(B256::zero())
    }

    fn balance(&mut self, _address: B160) -> Option<(U256, bool)> {
        Some((U256::ZERO, false))
    }

    fn code(&mut self, _address: B160) -> Option<(Arc<BytecodeLocked>, bool)> {
        Some((Arc::new(BytecodeLocked::default()), false))
    }

    fn code_hash(&mut self, __address: B160) -> Option<(B256, bool)> {
        Some((KECCAK_EMPTY, false))
    }

    fn sload(&mut self, __address: B160, index: U256) -> Option<(U256, bool)> {
        match self.storage.entry(index) {
            Entry::Occupied(entry) => Some((*entry.get(), false)),
            Entry::Vacant(entry) => {
                entry.insert(U256::ZERO);
                Some((U256::ZERO, true))
            }
        }
    }

    fn sstore(
        &mut self,
        _address: B160,
        index: U256,
        value: U256,
    ) -> Option<(U256, U256, U256, bool)> {
        let (present, is_cold) = match self.storage.entry(index) {
            Entry::Occupied(mut entry) => (entry.insert(value), false),
            Entry::Vacant(entry) => {
                entry.insert(value);
                (U256::ZERO, true)
            }
        };

        Some((U256::ZERO, present, value, is_cold))
    }

    #[inline]
    fn tload(&mut self, _address: B160, index: U256) -> U256 {
        self.transient_storage
            .get(&index)
            .copied()
            .unwrap_or_default()
    }

    #[inline]
    fn tstore(&mut self, _address: B160, index: U256, value: U256) {
        self.transient_storage.insert(index, value);
    }

    fn log(&mut self, address: B160, topics: Vec<B256>, data: Bytes) {
        self.log.push(Log {
            address,
            topics,
            data,
        })
    }

    fn selfdestruct(&mut self, _address: B160, _target: B160) -> Option<SelfDestructResult> {
        panic!("Selfdestruct is not supported for this host")
    }

    fn create(
        &mut self,
        _inputs: &mut CreateInputs,
        _: &mut u32,
    ) -> (InstructionResult, Option<B160>, Gas, Bytes) {
        panic!("Create is not supported for this host")
    }

    fn call(
        &mut self,
        _input: &mut CallInputs,
        _: &mut Interpreter,
        output_info: (usize, usize),
        _: &mut u32,
    ) -> (InstructionResult, Gas, Bytes) {
        panic!("Call is not supported for this host")
    }
}

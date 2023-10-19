#![warn(unreachable_pub)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![deny(unused_must_use, rust_2018_idioms)]
#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
extern crate alloc;

pub mod db;
mod evm;
mod evm_context;
mod evm_impl;
pub mod handler;
mod inspector;
mod journaled_state;

#[cfg(feature = "optimism")]
pub mod optimism;

#[cfg(all(feature = "with-serde", not(feature = "serde")))]
compile_error!("`with-serde` feature has been renamed to `serde`.");

pub(crate) const USE_GAS: bool = !cfg!(feature = "no_gas_measuring");

pub type DummyStateDB = InMemoryDB;
use db::{CacheDB, EmptyDB};
#[cfg(feature = "std")]
pub use db::{
    CacheState, DBBox, State, StateBuilder, StateDBBox, TransitionAccount, TransitionState,
};
pub use db::{Database, DatabaseCommit, DatabaseRef, InMemoryDB};
pub use evm::{evm_inner, new, EVM};
pub use evm_context::EVMData;
pub use evm_impl::{EVMImpl, Transact, CALL_STACK_LIMIT};
use interpreter::{DummyHost, Interpreter};
use interpreter::opcode::make_instruction_table;
pub use interpreter::{Contract, SharedMemory, DummyContract};
pub use journaled_state::{is_precompile, JournalCheckpoint, JournalEntry, JournaledState};

use primitives::SpecId::SHANGHAI;
use primitives::{Env, ShanghaiSpec};
use revm_precompile::Precompiles;

// reexport `revm_precompiles`
#[doc(inline)]
pub use revm_precompile as precompile;

// reexport `revm_interpreter`
#[doc(inline)]
pub use revm_interpreter as interpreter;

// reexport `revm_primitives`
#[doc(inline)]
pub use revm_interpreter::primitives;

// reexport inspector implementations
pub use inspector::inspectors;
pub use inspector::{inspector_instruction, Inspector};

// export Optimism types, helpers, and constants
#[cfg(feature = "optimism")]
pub use optimism::{L1BlockInfo, BASE_FEE_RECIPIENT, L1_BLOCK_CONTRACT, L1_FEE_RECIPIENT};

pub use handler::Handler;


pub fn test_harness(contract: DummyContract) {
    let mut env: Env = Default::default(); 
    let mut init_state = CacheDB::new(EmptyDB::default());
    let mut host = DummyHost::new(env);
    let instruction_table = make_instruction_table::<DummyHost, ShanghaiSpec>();
    let mut shared_mem = SharedMemory::new_with_memory_limit(1024 * 1024);


    // println!("contract: {:?}", contract);

    let mut interpreter = Interpreter::new(
        Box::new(contract.to_contract()),
        100000,
        false,
        &mut shared_mem,
    );

    let res = interpreter.run(&instruction_table, &mut host);
}

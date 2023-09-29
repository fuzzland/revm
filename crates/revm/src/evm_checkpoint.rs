use crate::evm_impl::{ExecutionContext, Resume};
use crate::primitives::{specification::*, Env, SpecId};
use crate::{Database, EVMData, EVMImpl, Inspector, JournaledState};
use revm_precompile::Precompiles;

#[derive(Clone)]
pub struct EvmCheckpoint {
    pub journaled_state: JournaledState,
    pub precompiles: Precompiles,
    pub execution_contexts: Vec<ExecutionContext>,
}

impl EvmCheckpoint {
    pub fn new<DB: Database>(data: &EVMData<'_, DB>) -> EvmCheckpoint {
        EvmCheckpoint {
            journaled_state: data.journaled_state.clone(),
            precompiles: data.precompiles.clone(),
            execution_contexts: data.execution_contexts.clone(),
        }
    }

    pub fn recover<'a, DB: Database, const INSPECT: bool>(
        &self,
        env: &'a mut Env,
        db: &'a mut DB,
        inspector: &'a mut dyn Inspector<DB>,
    ) -> Box<dyn Resume<DB::Error> + 'a> {
        macro_rules! create_resumable_evm {
            ($spec:ident) => {
                Box::new(EVMImpl::<'a, $spec, DB, INSPECT> {
                    data: EVMData {
                        env,
                        db,

                        journaled_state: self.journaled_state.clone(),
                        precompiles: self.precompiles.clone(),
                        execution_contexts: self.execution_contexts.clone(),
                        error: None,
                    },
                    inspector,
                    _phantomdata: Default::default(),
                }) as Box<dyn Resume<DB::Error> + 'a>
            };
        }

        match env.cfg.spec_id {
            SpecId::FRONTIER | SpecId::FRONTIER_THAWING => create_resumable_evm!(FrontierSpec),
            SpecId::HOMESTEAD | SpecId::DAO_FORK => create_resumable_evm!(HomesteadSpec),
            SpecId::TANGERINE => create_resumable_evm!(TangerineSpec),
            SpecId::SPURIOUS_DRAGON => create_resumable_evm!(SpuriousDragonSpec),
            SpecId::BYZANTIUM => create_resumable_evm!(ByzantiumSpec),
            SpecId::PETERSBURG | SpecId::CONSTANTINOPLE => create_resumable_evm!(PetersburgSpec),
            SpecId::ISTANBUL | SpecId::MUIR_GLACIER => create_resumable_evm!(IstanbulSpec),
            SpecId::BERLIN => create_resumable_evm!(BerlinSpec),
            SpecId::LONDON | SpecId::ARROW_GLACIER | SpecId::GRAY_GLACIER => {
                create_resumable_evm!(LondonSpec)
            }
            SpecId::MERGE => create_resumable_evm!(MergeSpec),
            SpecId::SHANGHAI => create_resumable_evm!(ShanghaiSpec),
            SpecId::CANCUN => create_resumable_evm!(CancunSpec),
            SpecId::LATEST => create_resumable_evm!(LatestSpec),
        }
    }
}

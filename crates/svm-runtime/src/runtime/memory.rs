use std::cell::RefCell;
use std::rc::Rc;

use svm_common::{Address, State};
use svm_kv::memory::MemKVStore;

use svm_contract::{
    env::{ContractEnv, ContractEnvTypes},
    memory::{MemContractStore, MemoryEnv},
};

use svm_storage::{
    memory::{MemContractPageCache, MemContractPages},
    ContractStorage,
};

use crate::opts::Opts;
use crate::runtime::Runtime;

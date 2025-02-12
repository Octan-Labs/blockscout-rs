#![allow(dead_code, unused)]

use std::collections::BTreeMap;

pub struct TestInput {
    pub contract_name: &'static str,
    pub compiler_version: &'static str,
    pub evm_version: &'static str,
    pub optimization_runs: Option<usize>,
    pub contract_libraries: BTreeMap<String, String>,
    pub has_constructor_args: bool,
    pub is_yul: bool,

    /// If None, the input would be read from the corresponding file
    pub source_code: Option<String>,
    /// If None, the input would be read from the corresponding file
    pub creation_tx_input: Option<String>,
    /// If None, the bytecode would be read from the corresponding file
    pub deployed_bytecode: Option<String>,
}

impl TestInput {
    pub fn new(contract_name: &'static str, compiler_version: &'static str) -> Self {
        Self {
            contract_name,
            compiler_version,
            evm_version: "default",
            optimization_runs: None,
            contract_libraries: Default::default(),
            has_constructor_args: false,
            is_yul: false,

            source_code: None,
            creation_tx_input: None,
            deployed_bytecode: None,
        }
    }

    pub fn with_evm_version(mut self, evm_version: &'static str) -> Self {
        self.evm_version = evm_version;
        self
    }

    pub fn with_optimization_runs(mut self, runs: usize) -> Self {
        self.optimization_runs = Some(runs);
        self
    }

    pub fn with_contract_libraries(mut self, libraries: BTreeMap<String, String>) -> Self {
        self.contract_libraries = libraries;
        self
    }

    pub fn has_constructor_args(mut self) -> Self {
        self.has_constructor_args = true;
        self
    }

    pub fn is_yul(mut self) -> Self {
        self.is_yul = true;
        self
    }

    pub fn with_source_code(mut self, source_code: String) -> Self {
        self.source_code = Some(source_code);
        self
    }

    pub fn with_deployed_bytecode(mut self, deployed_bytecode: String) -> Self {
        self.deployed_bytecode = Some(deployed_bytecode);
        self
    }

    pub fn with_creation_tx_input(mut self, creation_tx_input: String) -> Self {
        self.creation_tx_input = Some(creation_tx_input);
        self
    }
}

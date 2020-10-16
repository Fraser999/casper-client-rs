use casper_execution_engine::core::engine_state::ExecutableDeployItem;
use casper_types::{bytesrepr::ToBytes, ContractPackageHash, RuntimeArgs};

use crate::error::Result;

/// Extention trait for ExecutableDeploysItem, contains convenience constructors.
pub trait ExecutableDeployItemExt {
    /// Helper for creating an `ExecutableDeployItem::StoredContractByName`.
    fn stored_contract_by_name(
        name: String,
        entry_point: String,
        args: RuntimeArgs,
    ) -> Result<ExecutableDeployItem>;

    /// Helper to create a `ExecutableDeployItem::StoredContractByHash`.
    fn stored_contract_by_hash(
        hash: ContractPackageHash,
        entry_point: String,
        args: RuntimeArgs,
    ) -> Result<ExecutableDeployItem>;

    /// Helper to create a `ExecutableDeployItem::StoredVersionedContractByHash`.
    fn stored_versioned_contract_by_hash(
        hash: ContractPackageHash,
        version: u32,
        entry_point: String,
        args: RuntimeArgs,
    ) -> Result<ExecutableDeployItem>;

    /// Helper to create a `ExecutableDeployItem::StoredVersionedContractByName`.
    fn stored_versioned_contract_by_name(
        name: String,
        version: u32,
        entry_point: String,
        args: RuntimeArgs,
    ) -> Result<ExecutableDeployItem>;

    /// Helper to create a `ExecutableDeployItem::ModuleBytes`.
    fn from_module_bytes(module_bytes: Vec<u8>, args: RuntimeArgs) -> Result<ExecutableDeployItem>;
}

impl ExecutableDeployItemExt for ExecutableDeployItem {
    fn stored_contract_by_name(
        name: String,
        entry_point: String,
        args: RuntimeArgs,
    ) -> Result<ExecutableDeployItem> {
        Ok(ExecutableDeployItem::StoredContractByName {
            name,
            entry_point,
            args: args.to_bytes()?,
        })
    }

    fn stored_contract_by_hash(
        hash: ContractPackageHash,
        entry_point: String,
        args: RuntimeArgs,
    ) -> Result<ExecutableDeployItem> {
        Ok(ExecutableDeployItem::StoredContractByHash {
            hash,
            entry_point,
            args: args.to_bytes()?,
        })
    }

    fn stored_versioned_contract_by_name(
        name: String,
        version: u32,
        entry_point: String,
        args: RuntimeArgs,
    ) -> Result<ExecutableDeployItem> {
        Ok(ExecutableDeployItem::StoredVersionedContractByName {
            name,
            version: Some(version), // defaults to highest enabled version
            entry_point,
            args: args.to_bytes()?,
        })
    }

    fn stored_versioned_contract_by_hash(
        hash: ContractPackageHash,
        version: u32,
        entry_point: String,
        args: RuntimeArgs,
    ) -> Result<ExecutableDeployItem> {
        Ok(ExecutableDeployItem::StoredVersionedContractByHash {
            hash,
            version: Some(version), // defaults to highest enabled version
            entry_point,
            args: args.to_bytes()?,
        })
    }

    fn from_module_bytes(module_bytes: Vec<u8>, args: RuntimeArgs) -> Result<ExecutableDeployItem> {
        Ok(ExecutableDeployItem::ModuleBytes {
            module_bytes,
            args: args.to_bytes()?,
        })
    }
}
//! Standalone crate for MEV's node configuration and builder types.
//!
//! This contains mainly two types, [`MEVNode`](node::MEVNode) and
//! [`MEVEvmConfig`](evm::MEVEvmConfig).
//!
//! The [`MEVNode`](node::MEVNode) type implements the
//! [`NodeTypes`](reth_node_builder::NodeTypes) trait, and configures the engine types required for
//! the optimism engine API.
//!
//! The [`MEVEvmConfig`](evm::MEVEvmConfig) type implements the
//! [`ConfigureEvm`](reth_node_api::ConfigureEvm) and
//! [`ConfigureEvmEnv`](reth_node_api::ConfigureEvmEnv) traits, configuring the custom MEV
//! precompiles and instructions.

#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![warn(unused_crate_dependencies)]

pub mod broadcaster;
pub mod chainspec;
pub mod evm;
pub mod forwarder;
pub mod node;
pub mod rpc;

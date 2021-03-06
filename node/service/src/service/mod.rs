// This file is part of Darwinia.
//
// Copyright (C) 2018-2021 Darwinia Network
// SPDX-License-Identifier: GPL-3.0
//
// Darwinia is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Darwinia is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Darwinia. If not, see <https://www.gnu.org/licenses/>.

macro_rules! impl_runtime_apis {
	($($extra_apis:path),*) => {
		/// A set of APIs that darwinia-like runtimes must implement.
		pub trait RuntimeApiCollection:
			sp_api::ApiExt<Block>
			+ sp_api::Metadata<Block>
			+ sp_authority_discovery::AuthorityDiscoveryApi<Block>
			+ sp_block_builder::BlockBuilder<Block>
			+ sp_consensus_babe::BabeApi<Block>
			+ sp_finality_grandpa::GrandpaApi<Block>
			+ sp_offchain::OffchainWorkerApi<Block>
			+ sp_session::SessionKeys<Block>
			+ sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block>
			+ frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Nonce>
			+ pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance>
			+ darwinia_balances_rpc_runtime_api::BalancesApi<Block, AccountId, Balance>
			+ darwinia_header_mmr_rpc_runtime_api::HeaderMMRApi<Block, Hash>
			+ darwinia_staking_rpc_runtime_api::StakingApi<Block, AccountId, Power>
			$(+ $extra_apis),*
		where
			<Self as sp_api::ApiExt<Block>>::StateBackend: sp_api::StateBackend<BlakeTwo256>,
		{
		}
		impl<Api> RuntimeApiCollection for Api
		where
			Api: sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block>
				+ sp_api::ApiExt<Block>
				+ sp_api::Metadata<Block>
				+ sp_authority_discovery::AuthorityDiscoveryApi<Block>
				+ sp_block_builder::BlockBuilder<Block>
				+ sp_consensus_babe::BabeApi<Block>
				+ sp_finality_grandpa::GrandpaApi<Block>
				+ sp_offchain::OffchainWorkerApi<Block>
				+ sp_session::SessionKeys<Block>
				+ frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Nonce>
				+ pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance>
				+ darwinia_balances_rpc_runtime_api::BalancesApi<Block, AccountId, Balance>
				+ darwinia_header_mmr_rpc_runtime_api::HeaderMMRApi<Block, Hash>
				+ darwinia_staking_rpc_runtime_api::StakingApi<Block, AccountId, Power>
				$(+ $extra_apis),*,
			<Self as sp_api::ApiExt<Block>>::StateBackend: sp_api::StateBackend<BlakeTwo256>,
		{
		}
	};
}

pub mod crab;
pub mod darwinia;

// --- std ---
use std::sync::Arc;
// --- substrate ---
use sc_keystore::LocalKeystore;
use sc_service::{config::PrometheusConfig, ChainSpec, Configuration, Error as ServiceError};
use sp_runtime::traits::BlakeTwo256;
use substrate_prometheus_endpoint::Registry;
// --- darwinia ---
use darwinia_primitives::{AccountId, Balance, Hash, Nonce, OpaqueBlock as Block, Power};

type FullBackend = sc_service::TFullBackend<Block>;
type FullSelectChain = sc_consensus::LongestChain<FullBackend, Block>;
type FullClient<RuntimeApi, Executor> = sc_service::TFullClient<Block, RuntimeApi, Executor>;
type FullGrandpaBlockImport<RuntimeApi, Executor> = sc_finality_grandpa::GrandpaBlockImport<
	FullBackend,
	Block,
	FullClient<RuntimeApi, Executor>,
	FullSelectChain,
>;
type LightBackend = sc_service::TLightBackendWithHash<Block, BlakeTwo256>;
type LightClient<RuntimeApi, Executor> =
	sc_service::TLightClientWithBackend<Block, RuntimeApi, Executor, LightBackend>;

pub trait RuntimeExtrinsic: codec::Codec + Send + Sync + 'static {}
impl<E> RuntimeExtrinsic for E where E: codec::Codec + Send + Sync + 'static {}

/// Can be called for a `Configuration` to check if it is a configuration for the `Crab` network.
pub trait IdentifyVariant {
	/// Returns if this is a configuration for the `Crab` network.
	fn is_crab(&self) -> bool;

	/// Returns if this is a configuration for the `Darwinia` network.
	fn is_darwinia(&self) -> bool;
}
impl IdentifyVariant for Box<dyn ChainSpec> {
	fn is_crab(&self) -> bool {
		self.id().starts_with("crab")
	}

	fn is_darwinia(&self) -> bool {
		self.id().starts_with("darwinia")
	}
}

// If we're using prometheus, use a registry with a prefix of `darwinia`.
fn set_prometheus_registry(config: &mut Configuration) -> Result<(), ServiceError> {
	if let Some(PrometheusConfig { registry, .. }) = config.prometheus_config.as_mut() {
		*registry = Registry::new_custom(Some("darwinia".into()), None)?;
	}

	Ok(())
}

fn remote_keystore(_url: &String) -> Result<Arc<LocalKeystore>, &'static str> {
	// FIXME: here would the concrete keystore be built,
	//        must return a concrete type (NOT `LocalKeystore`) that
	//        implements `CryptoStore` and `SyncCryptoStore`
	Err("Remote Keystore not supported.")
}

// This file is part of Substrate.

// Copyright (C) 2017-2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! This API exposes the specification of the substrate's chain.
//!
//! # Note
//!
//! Methods are prefixed by `ChainSpec`.

use jsonrpsee::{core::RpcResult, proc_macros::rpc};
use sc_chain_spec::Properties;

#[rpc(client, server)]
pub trait ChainSpecApi {
	/// Get the specification of the chain.
	#[method(name = "chainSpec_unstable_properties", blocking)]
	fn chainspec_unstable_properties(&self) -> RpcResult<Properties>;

	/// Get the chain name.
	#[method(name = "chainSpec_unstable_chainName", blocking)]
	fn chainspec_unstable_chain_name(&self) -> RpcResult<String>;

	/// Get the chain's genesis hash.
	#[method(name = "chainSpec_unstable_genesisHash", blocking)]
	fn chainspec_unstable_genesis_hash(&self) -> RpcResult<String>;
}

// Copyright 2015-2018 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

//! Authority params deserialization.

use super::ValidatorSet;
use uint::Uint;

/// Authority params deserialization.
#[derive(Debug, PartialEq, Deserialize)]
pub struct BasicAuthorityParams {
	/// Block duration.
	#[serde(rename = "durationLimit")]
	pub duration_limit: Uint,
	/// Valid authorities
	pub validators: ValidatorSet,
}

/// Authority engine deserialization.
#[derive(Debug, PartialEq, Deserialize)]
pub struct BasicAuthority {
	/// Ethash params.
	pub params: BasicAuthorityParams,
}

#[cfg(test)]
mod tests {
	use ethereum_types::{H160, U256};
	use hash::Address;
	use serde_json;
	use spec::basic_authority::BasicAuthority;
	use spec::validator_set::ValidatorSet;
	use uint::Uint;

	#[test]
	fn basic_authority_deserialization() {
		let s = r#"{
			"params": {
				"durationLimit": "0x0d",
				"validators" : {
					"list": ["0xc6d9d2cd449a754c494264e1809c50e34d64562b"]
				}
			}
		}"#;

		let deserialized: BasicAuthority = serde_json::from_str(s).unwrap();

		assert_eq!(deserialized.params.duration_limit, Uint(U256::from(0x0d)));
		let vs = ValidatorSet::List(vec![Address(H160::from(
			"0xc6d9d2cd449a754c494264e1809c50e34d64562b",
		))]);
		assert_eq!(deserialized.params.validators, vs);
	}
}

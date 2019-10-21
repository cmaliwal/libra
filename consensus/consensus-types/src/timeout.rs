// Copyright (c) The Libra Core Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::{block::Block, common::Round};
use libra_crypto::hash::{CryptoHash, CryptoHasher, HashValue, TimeoutHasher};
use serde::{Deserialize, Serialize};

/// This structure contains all the information necessary to construct a signature
/// on the equivalent of a timeout message
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Timeout {
    /// Epoch number corresponds to the set of validators that are active for this round.
    epoch: u64,
    /// The consensus protocol executes proposals (blocks) in rounds, which monotically increase per epoch.
    round: Round,
}

impl Timeout {
    pub fn new(epoch: u64, round: Round) -> Self {
        Self { epoch, round }
    }

    pub fn from_block<T>(block: &Block<T>) -> Self {
        Self {
            epoch: block.epoch(),
            round: block.round(),
        }
    }

    pub fn epoch(&self) -> u64 {
        self.epoch
    }

    pub fn round(&self) -> Round {
        self.round
    }
}

impl CryptoHash for Timeout {
    type Hasher = TimeoutHasher;

    fn hash(&self) -> HashValue {
        let bytes = lcs::to_bytes(self).expect("Timeout serialization failed");
        let mut state = Self::Hasher::default();
        state.write(bytes.as_ref());
        state.finish()
    }
}
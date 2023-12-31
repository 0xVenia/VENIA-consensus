// consensus_state.rs
// Manages and maintains the state of the consensus process in the VENIA blockchain.

use std::collections::HashMap;
use crate::stake_manager::StakeManager;
use crate::validator_set::ValidatorSet;
use crate::block_proposal::Block;
use crate::utilities::crypto_utils;

/// Represents the consensus state of the blockchain at any given point.
pub struct ConsensusState {
    /// Current height of the blockchain.
    current_height: u64,

    /// Current set of validators.
    validators: ValidatorSet,

    /// Map of validator stakes.
    stake_manager: StakeManager,

    /// The last finalized block in the chain.
    last_finalized_block: Block,

    /// Tracks the current round of consensus.
    current_round: u64,

    /// Current leader or proposer in the consensus process.
    current_leader: String,
}

impl ConsensusState {
    /// Creates a new consensus state.
    pub fn new() -> ConsensusState {
        ConsensusState {
            current_height: 0,
            validators: ValidatorSet::new(),
            stake_manager: StakeManager::new(),
            last_finalized_block: Block::default(),
            current_round: 0,
            current_leader: String::new(),
        }
    }

    /// Updates the state with a new block.
    pub fn update_with_block(&mut self, block: &Block) {
        self.last_finalized_block = block.clone();
        self.current_height = block.height;
        self.update_round_and_leader();
    }

    /// Updates the current round and leader based on the latest block.
    fn update_round_and_leader(&mut self) {
        self.current_round += 1;
        self.current_leader = self.select_new_leader();
    }

    /// Selects a new leader for the next round of consensus.
    fn select_new_leader(&self) -> String {
        let leader = self.validators.get_next_leader();
        leader
    }

    /// Returns the current height of the blockchain.
    pub fn get_current_height(&self) -> u64 {
        self.current_height
    }

    /// Returns the current consensus round.
    pub fn get_current_round(&self) -> u64 {
        self.current_round
    }

    /// Returns the current leader.
    pub fn get_current_leader(&self) -> &str {
        &self.current_leader
    }

    // Additional methods related to consensus state management can be added here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_state() {
        let state = ConsensusState::new();
        assert_eq!(state.get_current_height(), 0);
        assert_eq!(state.get_current_round(), 0);
        assert!(state.get_current_leader().is_empty());
    }

}

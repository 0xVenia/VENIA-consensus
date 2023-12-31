// consensus_state.rs
// Manages the state of the consensus process in the VENIA blockchain.

use std::collections::HashMap;
use crate::stake_manager::StakeManager;
use crate::validator_set::ValidatorSet;
use crate::block_proposal::BlockProposal;
use serde::{Serialize, Deserialize};

/// Represents the current state of the consensus mechanism.
#[derive(Serialize, Deserialize, Debug)]
pub struct ConsensusState {
    /// The current set of validators.
    validators: ValidatorSet,

    /// Current stake information.
    stake_manager: StakeManager,

    /// The current highest block that has reached consensus.
    current_block: BlockProposal,

    /// Mapping of validator IDs to their last agreed block.
    validator_block_map: HashMap<String, BlockProposal>,

    /// Timestamp of the last state update.
    last_updated: u64,
}

impl ConsensusState {
    /// Initializes a new ConsensusState with the given components.
    pub fn new(validators: ValidatorSet, stake_manager: StakeManager, initial_block: BlockProposal) -> Self {
        ConsensusState {
            validators,
            stake_manager,
            current_block: initial_block,
            validator_block_map: HashMap::new(),
            last_updated: get_current_timestamp(),
        }
    }

    /// Updates the state based on a new block proposal.
    /// This function is called when a new block is proposed by a validator.
    pub fn update_state_with_block(&mut self, block: BlockProposal) {
        // Check if the block is valid and can be added to the blockchain
        if self.is_block_valid(&block) {
            self.current_block = block.clone();
            self.validator_block_map.insert(block.proposer_id.clone(), block);

            // Update the stake manager with any stake changes in the new block
            self.stake_manager.update_stakes(&block);

            // Update the timestamp
            self.last_updated = get_current_timestamp();

            // TODO: Add logic for handling block rewards and penalties.
        }
    }

    /// Validates whether the proposed block can be added to the blockchain.
    fn is_block_valid(&self, block: &BlockProposal) -> bool {
        // TODO: Implement block validation logic, including signature verification,
        //       transaction validity, and consensus rules.

        // Placeholder logic for illustration
        block.transactions.iter().all(|tx| tx.is_valid())
    }

    /// Returns the current highest block in the consensus state.
    pub fn get_current_block(&self) -> &BlockProposal {
        &self.current_block
    }

    // TODO: Implement additional functionalities as needed, such as state recovery,
    //       fork resolution, and validator set updates.

    // TODO: Integrate network communication mechanisms to propagate state changes
    //       to other nodes in the network.
}

/// Returns the current timestamp.
/// This is a utility function for getting the current time in a format suitable for the blockchain.
fn get_current_timestamp() -> u64 {
    // TODO: Implement a more robust and accurate time-fetching mechanism, possibly synchronized with network time.
    use std::time::{SystemTime, UNIX_EPOCH};

    let start = SystemTime::now();
    start.duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs()
}

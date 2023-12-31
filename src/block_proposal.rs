// block_proposal.rs
// Logic for proposing new blocks in the VENIA blockchain

use crate::{
    stake_manager::StakeManager,
    transaction_verifier::TransactionVerifier,
    consensus_state::ConsensusState,
    utilities::{time_utils, crypto_utils},
};

use blockchain_types::{
    Block, Transaction, Validator, BlockHeader,
};

pub struct BlockProposal {
    stake_manager: StakeManager,
    transaction_verifier: TransactionVerifier,
    consensus_state: ConsensusState,
}

impl BlockProposal {
    /// Creates a new instance of the BlockProposal module.
    pub fn new(stake_manager: StakeManager, transaction_verifier: TransactionVerifier, consensus_state: ConsensusState) -> Self {
        BlockProposal {
            stake_manager,
            transaction_verifier,
            consensus_state,
        }
    }

    /// Main function to propose a block. It orchestrates the block creation process.
    pub fn propose_block(&self, transactions: Vec<Transaction>, validator: &Validator) -> Block {
        let mut block = self.prepare_empty_block(validator);
        for transaction in transactions {
            if self.transaction_verifier.verify(&transaction) {
                block.add_transaction(transaction);
            }
        }
        self.finalize_block(&mut block);
        block
    }

    /// Prepares an empty block with the current validator's signature and other metadata.
    fn prepare_empty_block(&self, validator: &Validator) -> Block {
        let timestamp = time_utils::current_timestamp();
        let previous_hash = self.consensus_state.get_latest_block_hash();
        let block_header = BlockHeader::new(timestamp, previous_hash, validator.public_key.clone());
        
        Block::new(block_header, Vec::new())
    }

    /// Finalizes the block by computing the consensus-related metadata.
    fn finalize_block(&self, block: &mut Block) {
        let state_root_hash = self.consensus_state.compute_state_root();
        let block_hash = crypto_utils::hash_block(block);
        block.set_state_root(state_root_hash);
        block.set_hash(block_hash);

        // Calculate and assign the block's proof-of-stake-related attributes
        let stake_snapshot = self.stake_manager.get_current_stake_snapshot();
        let validator_reward = self.calculate_validator_reward(&stake_snapshot, block);
        block.set_validator_reward(validator_reward);
    }

     /// Calculates the reward for the validator proposing the block.
    fn calculate_validator_reward(&self, stake_snapshot: &StakeSnapshot, block: &Block) -> u64 {
        // Base reward determined by the network's predefined parameters
        let base_reward = self.consensus_state.get_base_reward();
    
        // Additional reward based on the number of transactions in the block
        let tx_reward = self.calculate_transaction_reward(block.transactions.len());
    
        // The validator's stake proportion in the network
        let validator_stake_proportion = self.calculate_stake_proportion(stake_snapshot, &block.validator_public_key);
    
        // Total reward considering base, transaction, and stake proportion
        base_reward + tx_reward + (validator_stake_proportion * base_reward)
    }
    
    /// Calculates additional rewards based on the number of transactions in the block.
    fn calculate_transaction_reward(&self, transaction_count: usize) -> u64 {
        // Define a reward rate per transaction
        let reward_per_transaction = self.consensus_state.get_reward_per_transaction();
    
        // Calculate the total transaction reward
        (transaction_count as u64) * reward_per_transaction
    }
    
    /// Calculates the proportion of stake the validator holds in the network.
    fn calculate_stake_proportion(&self, stake_snapshot: &StakeSnapshot, validator_public_key: &PublicKey) -> u64 {
        let validator_stake = stake_snapshot.get_stake(validator_public_key);
        let total_network_stake = stake_snapshot.get_total_stake();
    
        // Calculate the stake proportion (as a percentage of the total network stake)
        (validator_stake * 100) / total_network_stake
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    // Unit tests for BlockProposal module
    // ...
}

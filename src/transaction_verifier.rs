// transaction_verifier.rs

use crate::blockchain::Transaction;
use crate::crypto::signature::{verify_signature, PublicKey};
use crate::state::BlockchainState;

pub struct TransactionVerifier {
    blockchain_state: BlockchainState,
}

impl TransactionVerifier {
    /// Constructs a new TransactionVerifier
    pub fn new(blockchain_state: BlockchainState) -> Self {
        TransactionVerifier { blockchain_state }
    }

    /// Verifies a transaction for inclusion in a block
    pub fn verify_transaction(&self, transaction: &Transaction) -> Result<(), VerificationError> {
        self.check_signature(transaction)?;
        self.check_nonce(transaction)?;
        self.check_sufficient_balance(transaction)?;
        Ok(())
    }

    /// Checks the signature of the transaction
    fn check_signature(&self, transaction: &Transaction) -> Result<(), VerificationError> {
        let public_key = PublicKey::from_bytes(&transaction.sender);
        verify_signature(
            &transaction.signature,
            &transaction.payload,
            &public_key,
        )
        .map_err(|_| VerificationError::InvalidSignature)
    }

    /// Checks if the nonce is valid (i.e., matches the sender's current state)
    fn check_nonce(&self, transaction: &Transaction) -> Result<(), VerificationError> {
        let expected_nonce = self.blockchain_state.get_nonce(&transaction.sender);
        if transaction.nonce != expected_nonce {
            return Err(VerificationError::InvalidNonce);
        }
        Ok(())
    }

    /// Verifies that the sender has a sufficient balance to perform the transaction
    fn check_sufficient_balance(&self, transaction: &Transaction) -> Result<(), VerificationError> {
        let balance = self.blockchain_state.get_balance(&transaction.sender);
        if transaction.value > balance {
            return Err(VerificationError::InsufficientBalance);
        }
        Ok(())
    }
}

/// Errors that can occur during transaction verification
#[derive(Debug)]
pub enum VerificationError {
    InvalidSignature,
    InvalidNonce,
    InsufficientBalance,
    // TODO: Consider additional error types such as network-related errors.
}

// TODO: Implement additional checks such as transaction format validation,
//       double-spending prevention, and compliance with smart contract rules.

// TODO: Enhance error handling to provide more detailed information about
//       the cause of the verification failure.

// Unit tests for transaction verification
#[cfg(test)]
mod tests {
    // TODO: Write comprehensive unit tests covering various scenarios including
    //       valid transactions, invalid signatures, nonces, and insufficient balances.
}

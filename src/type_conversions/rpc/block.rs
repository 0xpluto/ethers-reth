use std::ops::Deref;

use crate::type_conversions::{ToEthers, ToReth};

use ethers::types::{
    Block as EthersBlock, OtherFields as EthersOtherFields, Transaction as EthersTransaction, Withdrawal as EthersWithdrawal, H256 as EthersH256
};
use reth_rpc_types::{other::OtherFields, Block, BlockTransactions, Header, Rich, Withdrawal};

/// EthersBlock<EthersH256> (ethers) -> Rich<Block> (reth)
impl ToReth<Rich<Block>> for EthersBlock<EthersH256> {
    fn into_reth(self) -> Rich<Block> {
        let txs = self.transactions;
        let block = Block {
            header: Header {
                hash: self.hash.into_reth(),
                parent_hash: self.parent_hash.into_reth(),
                uncles_hash: self.uncles_hash.into_reth(),
                miner: self.author.into_reth().unwrap(),
                state_root: self.state_root.into_reth(),
                transactions_root: self.transactions_root.into_reth(),
                receipts_root: self.receipts_root.into_reth(),
                logs_bloom: self.logs_bloom.into_reth().unwrap(),
                difficulty: self.difficulty.into_reth(),
                number: self.number.into_reth(),
                gas_limit: self.gas_limit.into_reth(),
                gas_used: self.gas_used.into_reth(),
                timestamp: self.timestamp.into_reth(),
                extra_data: self.extra_data.into_reth(),
                mix_hash: self.mix_hash.into_reth(),
                nonce: self.nonce.into_reth(),
                base_fee_per_gas: self.base_fee_per_gas.into_reth(),
                withdrawals_root: self.withdrawals_root.into_reth(),
                blob_gas_used: self.blob_gas_used.into_reth(),
                excess_blob_gas: self.excess_blob_gas.into_reth(),
                parent_beacon_block_root: self.parent_beacon_block_root.into_reth(),
                total_difficulty: self.total_difficulty.into_reth(),
            },
            uncles: self.uncles.into_reth(),
            transactions: txs.into_reth(),
            size: self.size.into_reth(),
            withdrawals: self.withdrawals.map(|w| w.into_reth()),
            other: OtherFields::default(),
        };
        Rich::from(block)
    }
}

/// EthersBlock<EthersTransaction> (ethers) -> Rich<Block> (reth)
impl ToReth<Rich<Block>> for EthersBlock<EthersTransaction> {
    fn into_reth(self) -> Rich<Block> {
        let txs = self.transactions;
        let block = Block {
            header: Header {
                hash: self.hash.into_reth(),
                parent_hash: self.parent_hash.into_reth(),
                uncles_hash: self.uncles_hash.into_reth(),
                miner: self.author.into_reth().unwrap(),
                state_root: self.state_root.into_reth(),
                transactions_root: self.transactions_root.into_reth(),
                receipts_root: self.receipts_root.into_reth(),
                logs_bloom: self.logs_bloom.into_reth().unwrap(),
                difficulty: self.difficulty.into_reth(),
                number: self.number.into_reth(),
                gas_limit: self.gas_limit.into_reth(),
                gas_used: self.gas_used.into_reth(),
                timestamp: self.timestamp.into_reth(),
                extra_data: self.extra_data.into_reth(),
                mix_hash: self.mix_hash.map(|h| h.into_reth()),
                nonce: self.nonce.into_reth(),
                base_fee_per_gas: self.base_fee_per_gas.into_reth(),
                withdrawals_root: self.withdrawals_root.into_reth(),
                blob_gas_used: self.blob_gas_used.into_reth(),
                excess_blob_gas: self.excess_blob_gas.into_reth(),
                parent_beacon_block_root: self.parent_beacon_block_root.into_reth(),
                total_difficulty: self.total_difficulty.into_reth(),
            },
            uncles: self.uncles.into_reth(),
            transactions: txs.into_reth(),
            size: self.size.into_reth(),
            withdrawals: self.withdrawals.into_reth(),
            other: self.other.into_reth(),
        };
        Rich::from(block)
    }
}

// ---------------------------------------

/// Rich<Block> (reth) -> EthersBlock<EthersTransaction> (ethers)
impl ToEthers<EthersBlock<EthersH256>> for Rich<Block> {
    fn into_ethers(self) -> EthersBlock<EthersH256> {
        let txs = match &self.transactions {
            BlockTransactions::Hashes(hashes) => hashes.into_ethers(),
            _ => vec![],
        };
        EthersBlock {
            hash: self.header.hash.into_ethers(),
            parent_hash: self.header.parent_hash.into_ethers(),
            uncles_hash: self.header.uncles_hash.into_ethers(),
            author: Some(self.header.miner.into_ethers()),
            state_root: self.header.state_root.into_ethers(),
            transactions_root: self.header.transactions_root.into_ethers(),
            receipts_root: self.header.receipts_root.into_ethers(),
            number: self.header.number.into_ethers(),
            gas_used: self.header.gas_used.into_ethers(),
            gas_limit: self.header.gas_limit.into_ethers(),
            extra_data: self.header.extra_data.clone().into_ethers(),
            logs_bloom: Some(self.header.logs_bloom.into_ethers()),
            timestamp: self.header.timestamp.into_ethers(),
            difficulty: self.header.difficulty.into_ethers(),
            total_difficulty: self.header.total_difficulty.into_ethers(),
            mix_hash: self.header.mix_hash.into_ethers(),
            nonce: self.header.nonce.into_ethers(),
            base_fee_per_gas: self.header.base_fee_per_gas.into_ethers(),
            withdrawals_root: self.header.withdrawals_root.into_ethers(),
            blob_gas_used: self.header.blob_gas_used.into_ethers(),
            excess_blob_gas: self.header.excess_blob_gas.into_ethers(),
            parent_beacon_block_root: self.header.parent_beacon_block_root.into_ethers(),
            seal_fields: vec![],
            other: self.inner.other.into_ethers(),
            uncles: self.inner.uncles.clone().into_ethers(),
            transactions: txs,
            size: self.inner.size.into_ethers(),
            withdrawals: self.inner.withdrawals.into_ethers(),
        }
    }
}

/// Rich<Block> (reth) -> EthersBlock<EthersTransaction> (ethers)
impl ToEthers<EthersBlock<EthersTransaction>> for Rich<Block> {
    fn into_ethers(self) -> EthersBlock<EthersTransaction> {
        let txs = match &self.transactions {
            BlockTransactions::Full(txs) => txs.into_ethers(),
            _ => vec![],
        };
        EthersBlock {
            hash: self.header.hash.into_ethers(),
            parent_hash: self.header.parent_hash.into_ethers(),
            uncles_hash: self.header.uncles_hash.into_ethers(),
            author: Some(self.header.miner.into_ethers()),
            state_root: self.header.state_root.into_ethers(),
            transactions_root: self.header.transactions_root.into_ethers(),
            receipts_root: self.header.receipts_root.into_ethers(),
            number: self.header.number.into_ethers(),
            gas_used: self.header.gas_used.into_ethers(),
            gas_limit: self.header.gas_limit.into_ethers(),
            extra_data: self.header.extra_data.clone().into_ethers(),
            logs_bloom: Some(self.header.logs_bloom.into_ethers()),
            timestamp: self.header.timestamp.into_ethers(),
            difficulty: self.header.difficulty.into_ethers(),
            total_difficulty: self.header.total_difficulty.into_ethers(),
            mix_hash: self.header.mix_hash.into_ethers(),
            nonce: self.header.nonce.into_ethers(),
            base_fee_per_gas: self.header.base_fee_per_gas.into_ethers(),
            withdrawals_root: self.header.withdrawals_root.into_ethers(),
            blob_gas_used: self.header.blob_gas_used.into_ethers(),
            excess_blob_gas: self.header.excess_blob_gas.into_ethers(),
            parent_beacon_block_root: self.header.parent_beacon_block_root.into_ethers(),
            seal_fields: vec![],
            uncles: self.inner.uncles.clone().into_ethers(),
            transactions: txs,
            size: self.inner.size.into_ethers(),
            withdrawals: self.inner.withdrawals.into_ethers(),
            other: self.inner.other.into_ethers(),
        }
    }
}

// -----------------------------------------------

/// Withdrawal (ethers) -> (reth)
impl ToReth<Withdrawal> for EthersWithdrawal {
    fn into_reth(self) -> Withdrawal {
        Withdrawal {
            index: self.index.as_u64(),
            validator_index: self.index.as_u64(),
            address: self.address.into_reth(),
            amount: self.amount.as_u64(),
        }
    }
}

/// Withdrawal (reth) -> (ethers)
impl ToEthers<EthersWithdrawal> for Withdrawal {
    fn into_ethers(self) -> EthersWithdrawal {
        EthersWithdrawal {
            index: self.index.into(),
            validator_index: self.index.into(),
            address: self.address.into_ethers(),
            amount: self.amount.into(),
        }
    }
}

impl ToReth<OtherFields> for EthersOtherFields {
    fn into_reth(self) -> OtherFields {
        OtherFields::new(self.deref().clone())
    }
}

impl ToEthers<EthersOtherFields> for OtherFields {
    fn into_ethers(self) -> EthersOtherFields {
        let val = self.deref().clone();
        let map = val.into_iter().collect();
        serde_json::from_value(serde_json::Value::Object(map)).unwrap()
    }
}

/// EthersBlock<H256> (ethers) -> BlockTransactions (reth)
impl ToReth<BlockTransactions> for Vec<EthersH256> {
    fn into_reth(self) -> BlockTransactions {
        BlockTransactions::Hashes(self.into_reth())
    }
}

/// EthersBlock<Transaction> (ethers) -> BlockTransactions (reth)
impl ToReth<BlockTransactions> for Vec<EthersTransaction> {
    fn into_reth(self) -> BlockTransactions {
        BlockTransactions::Full(self.into_reth())
    }
}

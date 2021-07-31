
#[macro_use]
extern crate jsonrpc_client_core;

extern crate serde_json;

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
pub type Address = String;
pub type StringDoaGddGA = String;
/// Keccak
///
/// Hex representation of a Keccak 256 hash
///
pub type Keccak = String;
/// Null
///
/// Null
///
pub type Null = serde_json::Value;
#[derive(Serialize, Deserialize)]
pub enum BlockHashOrNull {
    Keccak,
    Null
}
/// BlockNumber
///
/// The hex representation of the block's height
///
pub type BlockNumber = String;
#[derive(Serialize, Deserialize)]
pub enum BlockNumberOrNull {
    BlockNumber,
    Null
}
/// From
///
/// The sender of the transaction
///
pub type From = String;
/// TransactionGas
///
/// The gas limit provided by the sender in Wei
///
pub type TransactionGas = String;
/// TransactionGasPrice
///
/// The gas price willing to be paid by the sender in Wei
///
pub type TransactionGasPrice = String;
/// TransactionHash
///
/// Keccak 256 Hash of the RLP encoding of a transaction
///
pub type TransactionHash = String;
/// TransactionInput
///
/// The data field sent with the transaction
///
pub type TransactionInput = String;
/// TransactionNonce
///
/// The total number of prior transactions made by the sender
///
pub type TransactionNonce = String;
#[derive(Serialize, Deserialize)]
pub enum To {
    Address,
    Null
}
/// Integer
///
/// Hex representation of the integer
///
pub type Integer = String;
#[derive(Serialize, Deserialize)]
pub enum TransactionIndex {
    Integer,
    Null
}
/// TransactionValue
///
/// Value of Ether being transferred in Wei
///
pub type TransactionValue = String;
/// TransactionSigV
///
/// ECDSA recovery id
///
pub type TransactionSigV = String;
/// TransactionSigR
///
/// ECDSA signature r
///
pub type TransactionSigR = String;
/// TransactionSigS
///
/// ECDSA signature s
///
pub type TransactionSigS = String;
/// BlockNumberTag
///
/// The optional block height description
///
#[derive(Serialize, Deserialize)]
pub enum BlockNumberTag {
    #[serde(rename = "earliest")]
    Earliest,
    #[serde(rename = "latest")]
    Latest,
    #[serde(rename = "pending")]
    Pending,
}
pub type NumberHo1ClIqD = f64;
#[derive(Serialize, Deserialize)]
pub struct ObjectOfStringDoaGddGANumberHo1ClIqDLjEorPOj {
    pub(crate) txID: Option<StringDoaGddGA>,
    pub(crate) outputIndex: Option<NumberHo1ClIqD>,
}
pub type UnorderedSetOfObjectOfStringDoaGddGANumberHo1ClIqDLjEorPOjZ3FrnfAP = Vec<ObjectOfStringDoaGddGANumberHo1ClIqDLjEorPOj>;
/// AnyCVPkZfkL
///
/// specifies the format for the returned UTXOs. Can be either "cb58" or "hex" and defaults to "cb58".
///
pub type AnyCVPkZfkL = serde_json::Value;
pub type UnorderedSetOfStringDoaGddGADvj0XlFa = Vec<StringDoaGddGA>;
#[derive(Serialize, Deserialize)]
pub struct ObjectOfStringDoaGddGAAddressAJNaKQBQ {
    pub(crate) address: Option<Address>,
    pub(crate) utxo: Option<StringDoaGddGA>,
}
/// NumberJCGGoG2P
///
/// is the total balance, in nAVAX.
///
pub type NumberJCGGoG2P = f64;
/// NumberP3FC9PHt
///
/// is the unlocked balance, in nAVAX.
///
pub type NumberP3FC9PHt = f64;
/// NumberL6Q4SN4S
///
/// is the locked stakeable balance, in nAVAX.
///
pub type NumberL6Q4SN4S = f64;
/// NumberFm61Op1N
///
/// is the locked and not stakeable balance, in nAVAX.
///
pub type NumberFm61Op1N = f64;
pub type TxID = String;
#[derive(Serialize, Deserialize)]
pub struct ObjectOfTxIDNumberHo1ClIqD69VDNu5Y {
    pub(crate) txID: Option<TxID>,
    pub(crate) outputIndex: Option<NumberHo1ClIqD>,
}
/// UnorderedSetOfObjectOfTxIDNumberHo1ClIqD69VDNu5YpF9Ga0Z5
///
///  are the IDs of the UTXOs that reference address.
///
pub type UnorderedSetOfObjectOfTxIDNumberHo1ClIqD69VDNu5YpF9Ga0Z5 = Vec<ObjectOfTxIDNumberHo1ClIqD69VDNu5Y>;
/// StringOjNcTNl5
///
/// is the blockchain’s ID.
///
pub type StringOjNcTNl5 = String;
/// StringDKEc8I2F
///
/// is the human-readable name of this blockchain.
///
pub type StringDKEc8I2F = String;
/// StringInKlYZ6F
///
/// is the ID of the Subnet that validates this blockchain.
///
pub type StringInKlYZ6F = String;
/// StringUfYDgkNc
///
/// is the ID of the Virtual Machine the blockchain runs.
///
pub type StringUfYDgkNc = String;
#[derive(Serialize, Deserialize)]
pub struct ObjectOfStringUfYDgkNcStringInKlYZ6FStringDKEc8I2FStringOjNcTNl5AAFPeeNo {
    pub(crate) id: Option<StringOjNcTNl5>,
    pub(crate) name: Option<StringDKEc8I2F>,
    pub(crate) subnetID: Option<StringInKlYZ6F>,
    pub(crate) vmID: Option<StringUfYDgkNc>,
}
/// NumberCkPiL9T4
///
///  s the Unix time when the validator starts validating the Subnet.
///
pub type NumberCkPiL9T4 = f64;
/// NumberX6PqqnTc
///
/// is the Unix time when the validator stops validating the Subnet.
///
pub type NumberX6PqqnTc = f64;
/// NumberZljb5ZOi
///
/// is the amount of nAVAX this validator staked. Omitted if subnetID is not the Primary Network
///
pub type NumberZljb5ZOi = f64;
/// Addresses
///
/// List of contract addresses from which to monitor events
///
pub type Addresses = Vec<Address>;
#[derive(Serialize, Deserialize)]
pub struct ObjectOfNumberHo1ClIqDNumberHo1ClIqDAddressesNHhcVYMt {
    pub(crate) locktime: Option<NumberHo1ClIqD>,
    pub(crate) threshold: Option<NumberHo1ClIqD>,
    pub(crate) addresses: Option<Addresses>,
}
pub type BooleanVyG3AETh = bool;
/// NumberZS0MMx1A
///
/// is the Unix time when the validator starts validating the Subnet.
///
pub type NumberZS0MMx1A = f64;
/// StringU14URhxQ
///
/// is the validator’s node ID.
///
pub type StringU14URhxQ = String;
#[derive(Serialize, Deserialize)]
pub struct ObjectOfTxIDNumberZS0MMx1ANumberZljb5ZOiObjectOfNumberHo1ClIqDNumberHo1ClIqDAddressesNHhcVYMtNumberHo1ClIqDStringU14URhxQNumberX6PqqnTcK2OaXAFy {
    pub(crate) txID: Option<TxID>,
    pub(crate) startTime: Option<NumberZS0MMx1A>,
    pub(crate) endTime: Option<NumberX6PqqnTc>,
    pub(crate) stakeAmount: Option<NumberZljb5ZOi>,
    pub(crate) nodeID: Option<StringU14URhxQ>,
    pub(crate) rewardOwner: Option<ObjectOfNumberHo1ClIqDNumberHo1ClIqDAddressesNHhcVYMt>,
    pub(crate) potentialReward: Option<NumberHo1ClIqD>,
}
pub type UnorderedSetOfObjectOfTxIDNumberZS0MMx1ANumberZljb5ZOiObjectOfNumberHo1ClIqDNumberHo1ClIqDAddressesNHhcVYMtNumberHo1ClIqDStringU14URhxQNumberX6PqqnTcK2OaXAFytx2AdPt8 = Vec<ObjectOfTxIDNumberZS0MMx1ANumberZljb5ZOiObjectOfNumberHo1ClIqDNumberHo1ClIqDAddressesNHhcVYMtNumberHo1ClIqDStringU14URhxQNumberX6PqqnTcK2OaXAFy>;
#[derive(Serialize, Deserialize)]
pub struct ObjectOfNumberHo1ClIqDTxIDNumberCkPiL9T4NumberZljb5ZOiObjectOfNumberHo1ClIqDNumberHo1ClIqDAddressesNHhcVYMtNumberHo1ClIqDStringDoaGddGANumberX6PqqnTcUnorderedSetOfObjectOfTxIDNumberZS0MMx1ANumberZljb5ZOiObjectOfNumberHo1ClIqDNumberHo1ClIqDAddressesNHhcVYMtNumberHo1ClIqDStringU14URhxQNumberX6PqqnTcK2OaXAFytx2AdPt8NumberHo1ClIqDBooleanVyG3AEThHpZBWZ54 {
    pub(crate) txID: Option<TxID>,
    pub(crate) startTime: Option<NumberCkPiL9T4>,
    pub(crate) endTime: Option<NumberX6PqqnTc>,
    pub(crate) stakeAmount: Option<NumberZljb5ZOi>,
    pub(crate) nodeID: Option<StringDoaGddGA>,
    pub(crate) rewardOwner: Option<ObjectOfNumberHo1ClIqDNumberHo1ClIqDAddressesNHhcVYMt>,
    pub(crate) potentialReward: Option<NumberHo1ClIqD>,
    pub(crate) delegationFee: Option<NumberHo1ClIqD>,
    pub(crate) uptime: Option<NumberHo1ClIqD>,
    pub(crate) connected: Option<BooleanVyG3AETh>,
    pub(crate) delegators: Option<UnorderedSetOfObjectOfTxIDNumberZS0MMx1ANumberZljb5ZOiObjectOfNumberHo1ClIqDNumberHo1ClIqDAddressesNHhcVYMtNumberHo1ClIqDStringU14URhxQNumberX6PqqnTcK2OaXAFytx2AdPt8>,
}
/// BooleanMqW2Gfal
///
/// if the node is connected.
///
pub type BooleanMqW2Gfal = bool;
#[derive(Serialize, Deserialize)]
pub struct ObjectOfTxIDNumberZS0MMx1ANumberZljb5ZOiStringU14URhxQNumberX6PqqnTcNumberHo1ClIqDBooleanMqW2GfalC3Tj2OYE {
    pub(crate) txID: Option<TxID>,
    pub(crate) startTime: Option<NumberZS0MMx1A>,
    pub(crate) endTime: Option<NumberX6PqqnTc>,
    pub(crate) stakeAmount: Option<NumberZljb5ZOi>,
    pub(crate) nodeID: Option<StringU14URhxQ>,
    pub(crate) delegationFee: Option<NumberHo1ClIqD>,
    pub(crate) connected: Option<BooleanMqW2Gfal>,
}
pub type UnorderedSetOfObjectOfTxIDNumberZS0MMx1ANumberZljb5ZOiStringU14URhxQNumberX6PqqnTcNumberHo1ClIqDBooleanMqW2GfalC3Tj2OYElguK8Z1W = Vec<ObjectOfTxIDNumberZS0MMx1ANumberZljb5ZOiStringU14URhxQNumberX6PqqnTcNumberHo1ClIqDBooleanMqW2GfalC3Tj2OYE>;
pub type UnorderedSetOfStringDoaGddGA8Uulm6IQ = Vec<StringDoaGddGA>;
#[derive(Serialize, Deserialize)]
pub struct ObjectOfStringDoaGddGAAddressTUFok2IX {
    pub(crate) address: Option<Address>,
    pub(crate) utxo: Option<StringDoaGddGA>,
}
/// String9OVXUiHX
///
/// is this node's version
///
pub type String9OVXUiHX = String;
/// StringOnhjWyns
///
/// is the version of the database this node is using
///
pub type StringOnhjWyns = String;
/// StringGn1XszHY
///
/// is the Git commit that this node was built from
///
pub type StringGn1XszHY = String;
/// ObjectOfStringDoaGddGAStringDoaGddGAStringDoaGddGA5OHTjo2U
///
/// is map where each key/value pair is the name of a VM, and the version of that VM this node runs
///
#[derive(Serialize, Deserialize)]
pub struct ObjectOfStringDoaGddGAStringDoaGddGAStringDoaGddGA5OHTjo2U {
    pub(crate) avm: Option<StringDoaGddGA>,
    pub(crate) evm: Option<StringDoaGddGA>,
    pub(crate) platform: Option<StringDoaGddGA>,
}
#[derive(Serialize, Deserialize)]
pub struct ObjectOfStringDoaGddGAStringDoaGddGAStringDoaGddGAStringDoaGddGAStringDoaGddGAStringDoaGddGAMtoeI6Ie {
    pub(crate) ip: Option<StringDoaGddGA>,
    pub(crate) publicIP: Option<StringDoaGddGA>,
    pub(crate) nodeID: Option<StringDoaGddGA>,
    pub(crate) version: Option<StringDoaGddGA>,
    pub(crate) lastSent: Option<StringDoaGddGA>,
    pub(crate) lastReceived: Option<StringDoaGddGA>,
}
pub type UnorderedSetOfObjectOfStringDoaGddGAStringDoaGddGAStringDoaGddGAStringDoaGddGAStringDoaGddGAStringDoaGddGAMtoeI6IeJMY8TQNp = Vec<ObjectOfStringDoaGddGAStringDoaGddGAStringDoaGddGAStringDoaGddGAStringDoaGddGAStringDoaGddGAMtoeI6Ie>;
/// Nonce
///
/// A number only to be used once
///
pub type Nonce = String;
/// BlockHash
///
/// The hex representation of the Keccak 256 of the RLP encoded block
///
pub type BlockHash = String;
#[derive(Serialize, Deserialize)]
pub enum NonceOrNull {
    Nonce,
    Null
}
/// BlockShaUncles
///
/// Keccak hash of the uncles data in the block
///
pub type BlockShaUncles = String;
/// BlockLogsBloom
///
/// The bloom filter for the logs of the block or null when its the pending block
///
pub type BlockLogsBloom = String;
/// BlockTransactionsRoot
///
/// The root of the transactions trie of the block.
///
pub type BlockTransactionsRoot = String;
/// BlockStateRoot
///
/// The root of the final state trie of the block
///
pub type BlockStateRoot = String;
/// BlockReceiptsRoot
///
/// The root of the receipts trie of the block
///
pub type BlockReceiptsRoot = String;
#[derive(Serialize, Deserialize)]
pub enum AddressOrNull {
    Address,
    Null
}
/// BlockDifficulty
///
/// Integer of the difficulty for this block
///
pub type BlockDifficulty = String;
#[derive(Serialize, Deserialize)]
pub enum BlockTotalDifficulty {
    Integer,
    Null
}
/// BlockExtraData
///
/// The 'extra data' field of this block
///
pub type BlockExtraData = String;
/// BlockSize
///
/// Integer the size of this block in bytes
///
pub type BlockSize = String;
/// BlockGasLimit
///
/// The maximum gas allowed in this block
///
pub type BlockGasLimit = String;
/// BlockGasUsed
///
/// The total used gas by all transactions in this block
///
pub type BlockGasUsed = String;
/// BlockTimeStamp
///
/// The unix timestamp for when the block was collated
///
pub type BlockTimeStamp = String;
#[derive(Serialize, Deserialize)]
pub struct Transaction {
    pub(crate) blockHash: Option<BlockHashOrNull>,
    pub(crate) blockNumber: Option<BlockNumberOrNull>,
    pub(crate) from: Option<From>,
    pub(crate) gas: TransactionGas,
    pub(crate) gasPrice: TransactionGasPrice,
    pub(crate) hash: Option<TransactionHash>,
    pub(crate) input: Option<TransactionInput>,
    pub(crate) nonce: TransactionNonce,
    pub(crate) to: Option<To>,
    pub(crate) transactionIndex: Option<TransactionIndex>,
    pub(crate) value: Option<TransactionValue>,
    pub(crate) v: Option<TransactionSigV>,
    pub(crate) r: Option<TransactionSigR>,
    pub(crate) s: Option<TransactionSigS>,
}
#[derive(Serialize, Deserialize)]
pub enum TransactionOrTransactionHash {
    Transaction,
    TransactionHash
}
/// TransactionsOrHashes
///
/// Array of transaction objects, or 32 Bytes transaction hashes depending on the last given parameter
///
pub type TransactionsOrHashes = Vec<TransactionOrTransactionHash>;
/// UncleHash
///
/// Block hash of the RLP encoding of an uncle block
///
pub type UncleHash = String;
/// UncleHashes
///
/// Array of uncle hashes
///
pub type UncleHashes = Vec<UncleHash>;
/// Block
///
/// The Block is the collection of relevant pieces of information (known as the block header), together with information corresponding to the comprised transactions, and a set of other block headers that are known to have a parent equal to the present block’s parent’s parent.
///
#[derive(Serialize, Deserialize)]
pub struct Block {
    pub(crate) number: Option<BlockNumberOrNull>,
    pub(crate) hash: Option<BlockHashOrNull>,
    pub(crate) parentHash: Option<BlockHash>,
    pub(crate) nonce: Option<NonceOrNull>,
    pub(crate) sha3Uncles: Option<BlockShaUncles>,
    pub(crate) logsBloom: Option<BlockLogsBloom>,
    pub(crate) transactionsRoot: Option<BlockTransactionsRoot>,
    pub(crate) stateRoot: Option<BlockStateRoot>,
    pub(crate) receiptsRoot: Option<BlockReceiptsRoot>,
    pub(crate) miner: Option<AddressOrNull>,
    pub(crate) difficulty: Option<BlockDifficulty>,
    pub(crate) totalDifficulty: Option<BlockTotalDifficulty>,
    pub(crate) extraData: Option<BlockExtraData>,
    pub(crate) size: Option<BlockSize>,
    pub(crate) gasLimit: Option<BlockGasLimit>,
    pub(crate) gasUsed: Option<BlockGasUsed>,
    pub(crate) timestamp: Option<BlockTimeStamp>,
    pub(crate) transactions: Option<TransactionsOrHashes>,
    pub(crate) uncles: Option<UncleHashes>,
}
#[derive(Serialize, Deserialize)]
pub enum ReceiptContractAddress {
    Address,
    Null
}
/// ReceiptCumulativeGasUsed
///
/// The gas units used by the transaction
///
pub type ReceiptCumulativeGasUsed = String;
/// ReceiptGasUsed
///
/// The total gas used by the transaction
///
pub type ReceiptGasUsed = String;
/// LogAddress
///
/// Sender of the transaction
///
pub type LogAddress = String;
/// LogData
///
/// The data/input string sent along with the transaction
///
pub type LogData = String;
/// LogIndex
///
/// The index of the event within its transaction, null when its pending
///
pub type LogIndex = String;
/// LogIsRemoved
///
/// Whether or not the log was orphaned off the main chain
///
pub type LogIsRemoved = bool;
/// Topic
///
/// 32 Bytes DATA of indexed log arguments. (In solidity: The first topic is the hash of the signature of the event (e.g. Deposit(address,bytes32,uint256))
///
pub type Topic = String;
/// LogTopics
///
/// Topics are order-dependent. Each topic can also be an array of DATA with 'or' options.
///
pub type LogTopics = Vec<Topic>;
/// Log
///
/// An indexed event generated during a transaction
///
#[derive(Serialize, Deserialize)]
pub struct Log {
    pub(crate) address: Option<LogAddress>,
    pub(crate) blockHash: Option<BlockHash>,
    pub(crate) blockNumber: Option<BlockNumber>,
    pub(crate) data: Option<LogData>,
    pub(crate) logIndex: Option<LogIndex>,
    pub(crate) removed: Option<LogIsRemoved>,
    pub(crate) topics: Option<LogTopics>,
    pub(crate) transactionHash: Option<TransactionHash>,
    pub(crate) transactionIndex: Option<TransactionIndex>,
}
/// Logs
///
/// An array of all the logs triggered during the transaction
///
pub type Logs = Vec<Log>;
/// BloomFilter
///
/// A 2048 bit bloom filter from the logs of the transaction. Each log sets 3 bits though taking the low-order 11 bits of each of the first three pairs of bytes in a Keccak 256 hash of the log's byte series
///
pub type BloomFilter = String;
/// ReceiptPostTransactionState
///
/// The intermediate stateRoot directly after transaction execution.
///
pub type ReceiptPostTransactionState = String;
/// ReceiptStatus
///
/// Whether or not the transaction threw an error.
///
pub type ReceiptStatus = bool;
/// Receipt
///
/// The receipt of a transaction
///
#[derive(Serialize, Deserialize)]
pub struct Receipt {
    pub(crate) blockHash: BlockHash,
    pub(crate) blockNumber: BlockNumber,
    pub(crate) contractAddress: ReceiptContractAddress,
    pub(crate) cumulativeGasUsed: ReceiptCumulativeGasUsed,
    pub(crate) from: From,
    pub(crate) gasUsed: ReceiptGasUsed,
    pub(crate) logs: Logs,
    pub(crate) logsBloom: BloomFilter,
    pub(crate) to: To,
    pub(crate) transactionHash: TransactionHash,
    pub(crate) transactionIndex: TransactionIndex,
    pub(crate) postTransactionState: Option<ReceiptPostTransactionState>,
    pub(crate) status: Option<ReceiptStatus>,
}
pub type AnyHbW97Yqg = serde_json::Value;
pub type AnyGVzl2YJR = serde_json::Value;
#[derive(Serialize, Deserialize)]
pub struct ObjectOfStringDoaGddGAStringDoaGddGA4SZgw1S0 {
    pub(crate) address: Option<StringDoaGddGA>,
    pub(crate) utxo: Option<StringDoaGddGA>,
}
pub type Ids = Vec<StringDoaGddGA>;
pub type UnorderedSetOfStringDoaGddGARCi5AHUB = Vec<StringDoaGddGA>;
#[derive(Serialize, Deserialize)]
pub enum BlockNumberOrTag {
    BlockNumber,
    BlockNumberTag
}
#[derive(Serialize, Deserialize)]
pub struct SignTransaction {
    pub(crate) data: StringDoaGddGA,
    pub(crate) from: From,
    pub(crate) gas: TransactionGas,
    pub(crate) gasPrice: TransactionGasPrice,
    pub(crate) to: To,
    pub(crate) value: TransactionValue,
}
/// Bytes
///
/// Hex representation of a variable length byte array
///
pub type Bytes = String;
pub type IsTransactionsIncluded = bool;
#[derive(Serialize, Deserialize)]
pub struct StartIndex {
    pub(crate) address: Option<StringDoaGddGA>,
    pub(crate) utxo: Option<StringDoaGddGA>,
}
pub type Data = String;
#[derive(Serialize, Deserialize)]
pub struct Balances {
    pub(crate) asset: Option<StringDoaGddGA>,
    pub(crate) balance: Option<NumberHo1ClIqD>,
}
#[derive(Serialize, Deserialize)]
pub struct ObjectOfStringDoaGddGAStringDoaGddGANumberHo1ClIqDStringDoaGddGAZzZvQgFI {
    pub(crate) assetID: Option<StringDoaGddGA>,
    pub(crate) name: Option<StringDoaGddGA>,
    pub(crate) symbol: Option<StringDoaGddGA>,
    pub(crate) denomination: Option<NumberHo1ClIqD>,
}
#[derive(Serialize, Deserialize)]
pub struct ObjectOfUnorderedSetOfObjectOfStringDoaGddGANumberHo1ClIqDLjEorPOjZ3FrnfAPNumberHo1ClIqDHIfq2P7W {
    pub(crate) balance: Option<NumberHo1ClIqD>,
    pub(crate) utxoIDs: Option<UnorderedSetOfObjectOfStringDoaGddGANumberHo1ClIqDLjEorPOjZ3FrnfAP>,
}
#[derive(Serialize, Deserialize)]
pub struct ObjectOfStringDoaGddGAAnyCVPkZfkLPuZmtk8Z {
    pub(crate) tx: Option<StringDoaGddGA>,
    pub(crate) encoding: Option<AnyCVPkZfkL>,
}
pub type Status = String;
#[derive(Serialize, Deserialize)]
pub struct ObjectOfUnorderedSetOfStringDoaGddGADvj0XlFaNumberHo1ClIqDObjectOfStringDoaGddGAAddressAJNaKQBQAnyCVPkZfkL1ZHz1Bxo {
    pub(crate) numFetched: Option<NumberHo1ClIqD>,
    pub(crate) utxos: Option<UnorderedSetOfStringDoaGddGADvj0XlFa>,
    pub(crate) endIndex: Option<ObjectOfStringDoaGddGAAddressAJNaKQBQ>,
    pub(crate) encoding: Option<AnyCVPkZfkL>,
}
#[derive(Serialize, Deserialize)]
pub struct ObjectOfUnorderedSetOfObjectOfTxIDNumberHo1ClIqD69VDNu5YpF9Ga0Z5NumberP3FC9PHtNumberL6Q4SN4SNumberFm61Op1NNumberJCGGoG2P7ODD4O8M {
    pub(crate) balance: Option<NumberJCGGoG2P>,
    pub(crate) unlocked: Option<NumberP3FC9PHt>,
    pub(crate) lockedStakeable: Option<NumberL6Q4SN4S>,
    pub(crate) lockedNotStakeable: Option<NumberFm61Op1N>,
    pub(crate) utxoIDs: Option<UnorderedSetOfObjectOfTxIDNumberHo1ClIqD69VDNu5YpF9Ga0Z5>,
}
/// Blockchains
///
///  is all of the blockchains that exists on the Avalanche network.
///
pub type Blockchains = Vec<ObjectOfStringUfYDgkNcStringInKlYZ6FStringDKEc8I2FStringOjNcTNl5AAFPeeNo>;
pub type Supply = f64;
pub type Validators = Vec<ObjectOfNumberHo1ClIqDTxIDNumberCkPiL9T4NumberZljb5ZOiObjectOfNumberHo1ClIqDNumberHo1ClIqDAddressesNHhcVYMtNumberHo1ClIqDStringDoaGddGANumberX6PqqnTcUnorderedSetOfObjectOfTxIDNumberZS0MMx1ANumberZljb5ZOiObjectOfNumberHo1ClIqDNumberHo1ClIqDAddressesNHhcVYMtNumberHo1ClIqDStringU14URhxQNumberX6PqqnTcK2OaXAFytx2AdPt8NumberHo1ClIqDBooleanVyG3AEThHpZBWZ54>;
pub type Height = f64;
#[derive(Serialize, Deserialize)]
pub struct ObjectOfNumberHo1ClIqDNumberHo1ClIqDDx6Z4VqH {
    pub(crate) minValidatorStake: Option<NumberHo1ClIqD>,
    pub(crate) minDelegatorStake: Option<NumberHo1ClIqD>,
}
#[derive(Serialize, Deserialize)]
pub struct ObjectOfUnorderedSetOfStringDoaGddGADvj0XlFaNumberHo1ClIqDMGfbUCXP {
    pub(crate) numFetched: Option<NumberHo1ClIqD>,
    pub(crate) utxos: Option<UnorderedSetOfStringDoaGddGADvj0XlFa>,
}
#[derive(Serialize, Deserialize)]
pub struct AssetID {
    pub(crate) numFetched: Option<NumberHo1ClIqD>,
    pub(crate) utxos: Option<UnorderedSetOfStringDoaGddGADvj0XlFa>,
}
#[derive(Serialize, Deserialize)]
pub struct Subnets {
    pub(crate) id: Option<StringDoaGddGA>,
    pub(crate) controlKeys: Option<UnorderedSetOfStringDoaGddGA8Uulm6IQ>,
}
pub type Staked = f64;
pub type Stake = f64;
#[derive(Serialize, Deserialize)]
pub struct ObjectOfUnorderedSetOfStringDoaGddGADvj0XlFaNumberHo1ClIqDObjectOfStringDoaGddGAAddressTUFok2IXAnyCVPkZfkLOItHIivc {
    pub(crate) numFetched: Option<NumberHo1ClIqD>,
    pub(crate) utxos: Option<UnorderedSetOfStringDoaGddGADvj0XlFa>,
    pub(crate) endIndex: Option<ObjectOfStringDoaGddGAAddressTUFok2IX>,
    pub(crate) encoding: Option<AnyCVPkZfkL>,
}
pub type SubnetID = String;
pub type BlockchainIDs = Vec<StringDoaGddGA>;
#[derive(Serialize, Deserialize)]
pub struct ObjectOfObjectOfStringDoaGddGAStringDoaGddGAStringDoaGddGA5OHTjo2UString9OVXUiHXStringGn1XszHYStringOnhjWynsL4OwtBVJ {
    pub(crate) version: Option<String9OVXUiHX>,
    pub(crate) databaseVersion: Option<StringOnhjWyns>,
    pub(crate) gitCommit: Option<StringGn1XszHY>,
    pub(crate) vmVersions: Option<ObjectOfStringDoaGddGAStringDoaGddGAStringDoaGddGA5OHTjo2U>,
}
pub type BooleanLu7POU17 = bool;
#[derive(Serialize, Deserialize)]
pub struct ObjectOfUnorderedSetOfObjectOfStringDoaGddGAStringDoaGddGAStringDoaGddGAStringDoaGddGAStringDoaGddGAStringDoaGddGAMtoeI6IeJMY8TQNpNumberHo1ClIqDFEjTP7UK {
    pub(crate) numPeers: Option<NumberHo1ClIqD>,
    pub(crate) peers: Option<UnorderedSetOfObjectOfStringDoaGddGAStringDoaGddGAStringDoaGddGAStringDoaGddGAStringDoaGddGAStringDoaGddGAMtoeI6IeJMY8TQNp>,
}
#[derive(Serialize, Deserialize)]
pub struct ObjectOfNumberHo1ClIqDNumberHo1ClIqD6FHq3FGd {
    pub(crate) creationTxFee: Option<NumberHo1ClIqD>,
    pub(crate) txFee: Option<NumberHo1ClIqD>,
}
pub type ChainId = String;
#[derive(Serialize, Deserialize)]
pub enum IntegerOrNull {
    Integer,
    Null
}
#[derive(Serialize, Deserialize)]
pub enum BlockOrNull {
    Block,
    Null
}
#[derive(Serialize, Deserialize)]
pub enum TransactionOrNull {
    Transaction,
    Null
}
#[derive(Serialize, Deserialize)]
pub enum TransactionReceiptOrNull {
    Receipt,
    Null
}
#[derive(Serialize, Deserialize)]
pub enum AtomicTxStatusStatus {
    AnyHbW97Yqg,
    AnyGVzl2YJR
}
pub type ObjectHAgrRKSz = HashMap<String, Option<serde_json::Value>>;
pub type NetworkId = String;
pub type ClientVersion = String;
#[derive(Serialize, Deserialize)]
pub enum AnyOfStringDoaGddGAAddressStringDoaGddGAAddressStringDoaGddGATxIDStringDoaGddGATxIDAddressesNumberHo1ClIqDObjectOfStringDoaGddGAStringDoaGddGA4SZgw1S0StringDoaGddGAStringDoaGddGAStringDoaGddGAStringDoaGddGAAddressStringDoaGddGAStringDoaGddGAStringDoaGddGAStringDoaGddGAStringDoaGddGATxIDStringDoaGddGAStringDoaGddGAStringDoaGddGAIdsAddressesTxIDStringDoaGddGATxIDAddressesNumberHo1ClIqDObjectOfStringDoaGddGAStringDoaGddGA4SZgw1S0StringDoaGddGAStringDoaGddGAStringDoaGddGAStringDoaGddGANumberHo1ClIqDStringDoaGddGAStringDoaGddGAStringDoaGddGAStringDoaGddGAStringDoaGddGAUnorderedSetOfStringDoaGddGARCi5AHUBTransactionBlockNumberOrTagAddressBlockNumberStringDoaGddGAAddressBlockNumberSignTransactionAddressBlockNumberOrTagBytesBlockHashIsTransactionsIncludedBlockNumberOrTagIsTransactionsIncludedTransactionHashTransactionHashTxIDUnorderedSetOfStringDoaGddGADvj0XlFaStringDoaGddGAStringDoaGddGAStartIndexStringDoaGddGAStringDoaGddGAStringDoaGddGADataStringDoaGddGABalancesObjectOfStringDoaGddGAStringDoaGddGANumberHo1ClIqDStringDoaGddGAZzZvQgFIObjectOfUnorderedSetOfObjectOfStringDoaGddGANumberHo1ClIqDLjEorPOjZ3FrnfAPNumberHo1ClIqDHIfq2P7WObjectOfStringDoaGddGAAnyCVPkZfkLPuZmtk8ZStatusObjectOfUnorderedSetOfStringDoaGddGADvj0XlFaNumberHo1ClIqDObjectOfStringDoaGddGAAddressAJNaKQBQAnyCVPkZfkL1ZHz1BxoTxIDObjectOfUnorderedSetOfObjectOfTxIDNumberHo1ClIqD69VDNu5YpF9Ga0Z5NumberP3FC9PHtNumberL6Q4SN4SNumberFm61Op1NNumberJCGGoG2P7ODD4O8MBlockchainsStatusSupplyValidatorsHeightObjectOfNumberHo1ClIqDNumberHo1ClIqDDx6Z4VqHValidatorsObjectOfUnorderedSetOfStringDoaGddGADvj0XlFaNumberHo1ClIqDMGfbUCXPAssetIDSubnetsStakedStakeSubnetsStatusObjectOfUnorderedSetOfStringDoaGddGADvj0XlFaNumberHo1ClIqDObjectOfStringDoaGddGAAddressTUFok2IXAnyCVPkZfkLOItHIivcTxIDUnorderedSetOfStringDoaGddGADvj0XlFaSubnetIDBlockchainIDsStringDoaGddGANumberHo1ClIqDStringDoaGddGAObjectOfObjectOfStringDoaGddGAStringDoaGddGAStringDoaGddGA5OHTjo2UString9OVXUiHXStringGn1XszHYStringOnhjWynsL4OwtBVJBooleanLu7POU17ObjectOfUnorderedSetOfObjectOfStringDoaGddGAStringDoaGddGAStringDoaGddGAStringDoaGddGAStringDoaGddGAStringDoaGddGAMtoeI6IeJMY8TQNpNumberHo1ClIqDFEjTP7UKObjectOfNumberHo1ClIqDNumberHo1ClIqD6FHq3FGdBlockNumberOrTagBytesChainIdIntegerOrNullIntegerOrNullBytesNonceOrNullKeccakBlockOrNullBlockOrNullTransactionOrNullTransactionReceiptOrNullAtomicTxStatusStatusObjectHAgrRKSzTxIDNetworkIdClientVersionKeccak {
    StringDoaGddGA,
    Address,
    TxID,
    Addresses,
    NumberHo1ClIqD,
    ObjectOfStringDoaGddGAStringDoaGddGA4SZgw1S0,
    Ids,
    UnorderedSetOfStringDoaGddGARCi5AHUB,
    Transaction,
    BlockNumberOrTag,
    BlockNumber,
    SignTransaction,
    Bytes,
    BlockHash,
    IsTransactionsIncluded,
    TransactionHash,
    UnorderedSetOfStringDoaGddGADvj0XlFa,
    StartIndex,
    Data,
    Balances,
    ObjectOfStringDoaGddGAStringDoaGddGANumberHo1ClIqDStringDoaGddGAZzZvQgFI,
    ObjectOfUnorderedSetOfObjectOfStringDoaGddGANumberHo1ClIqDLjEorPOjZ3FrnfAPNumberHo1ClIqDHIfq2P7W,
    ObjectOfStringDoaGddGAAnyCVPkZfkLPuZmtk8Z,
    Status,
    ObjectOfUnorderedSetOfStringDoaGddGADvj0XlFaNumberHo1ClIqDObjectOfStringDoaGddGAAddressAJNaKQBQAnyCVPkZfkL1ZHz1Bxo,
    ObjectOfUnorderedSetOfObjectOfTxIDNumberHo1ClIqD69VDNu5YpF9Ga0Z5NumberP3FC9PHtNumberL6Q4SN4SNumberFm61Op1NNumberJCGGoG2P7ODD4O8M,
    Blockchains,
    Supply,
    Validators,
    Height,
    ObjectOfNumberHo1ClIqDNumberHo1ClIqDDx6Z4VqH,
    ObjectOfUnorderedSetOfStringDoaGddGADvj0XlFaNumberHo1ClIqDMGfbUCXP,
    AssetID,
    Subnets,
    Staked,
    Stake,
    ObjectOfUnorderedSetOfStringDoaGddGADvj0XlFaNumberHo1ClIqDObjectOfStringDoaGddGAAddressTUFok2IXAnyCVPkZfkLOItHIivc,
    SubnetID,
    BlockchainIDs,
    ObjectOfObjectOfStringDoaGddGAStringDoaGddGAStringDoaGddGA5OHTjo2UString9OVXUiHXStringGn1XszHYStringOnhjWynsL4OwtBVJ,
    BooleanLu7POU17,
    ObjectOfUnorderedSetOfObjectOfStringDoaGddGAStringDoaGddGAStringDoaGddGAStringDoaGddGAStringDoaGddGAStringDoaGddGAMtoeI6IeJMY8TQNpNumberHo1ClIqDFEjTP7UK,
    ObjectOfNumberHo1ClIqDNumberHo1ClIqD6FHq3FGd,
    ChainId,
    IntegerOrNull,
    NonceOrNull,
    Keccak,
    BlockOrNull,
    TransactionOrNull,
    TransactionReceiptOrNull,
    AtomicTxStatusStatus,
    ObjectHAgrRKSz,
    NetworkId,
    ClientVersion
}

jsonrpc_client!(pub struct AvalancheJSONRPC {
pub fn AvmBuildGenesis(&mut self, alias: StringDoaGddGA) -> RpcRequest<StringDoaGddGA>;
pub fn AvmGetAllBalances(&mut self, Address: Address) -> RpcRequest<Balances>;
pub fn AvmGetAssetDescription(&mut self, assetID: StringDoaGddGA) -> RpcRequest<ObjectOfStringDoaGddGAStringDoaGddGANumberHo1ClIqDStringDoaGddGAZzZvQgFI>;
pub fn AvmGetBalance(&mut self, Address: Address, assetID: StringDoaGddGA) -> RpcRequest<ObjectOfUnorderedSetOfObjectOfStringDoaGddGANumberHo1ClIqDLjEorPOjZ3FrnfAPNumberHo1ClIqDHIfq2P7W>;
pub fn AvmGetTx(&mut self, txID: TxID, encoding: StringDoaGddGA) -> RpcRequest<ObjectOfStringDoaGddGAAnyCVPkZfkL9MakbCFB>;
pub fn AvmGetTxStatus(&mut self, txID: TxID) -> RpcRequest<Status>;
pub fn AvmGetUTXOs(&mut self, addresses: Addresses, limit: NumberHo1ClIqD, startIndex: ObjectOfStringDoaGddGAStringDoaGddGA4SZgw1S0, sourceChain: StringDoaGddGA, encoding: StringDoaGddGA) -> RpcRequest<ObjectOfUnorderedSetOfStringDoaGddGADvj0XlFaNumberHo1ClIqDObjectOfStringDoaGddGAAddressAJNaKQBQAnyCVPkZfkLL5YMy317>;
pub fn AvmIssueTx(&mut self, tx: StringDoaGddGA, encoding: StringDoaGddGA) -> RpcRequest<TxID>;
pub fn PlatformGetBalance(&mut self, Address: Address) -> RpcRequest<ObjectOfUnorderedSetOfObjectOfTxIDNumberHo1ClIqD69VDNu5YpF9Ga0Z5NumberP3FC9PHtNumberL6Q4SN4SNumberFm61Op1NNumberJCGGoG2P7ODD4O8M>;
pub fn PlatformGetBlockchains(&mut self) -> RpcRequest<Blockchains>;
pub fn PlatformGetBlockchainStatus(&mut self, blockchainID: StringDoaGddGA) -> RpcRequest<Status>;
pub fn PlatformGetCurrentSupply(&mut self) -> RpcRequest<Supply>;
pub fn PlatformGetCurrentValidators(&mut self, subnetID: StringDoaGddGA, nodeIDs: StringDoaGddGA) -> RpcRequest<Validators>;
pub fn PlatformGetHeight(&mut self) -> RpcRequest<Height>;
pub fn PlatformGetMinStake(&mut self) -> RpcRequest<ObjectOfNumberHo1ClIqDNumberHo1ClIqDDx6Z4VqH>;
pub fn PlatformGetPendingValidators(&mut self, subnetID: StringDoaGddGA, nodeIDs: StringDoaGddGA) -> RpcRequest<Validators>;
pub fn PlatformGetRewardUTXOs(&mut self, txID: TxID, encoding: StringDoaGddGA) -> RpcRequest<ObjectOfUnorderedSetOfStringDoaGddGADvj0XlFaNumberHo1ClIqDMGfbUCXP>;
pub fn PlatformGetStakingAssetID(&mut self, subnetID: StringDoaGddGA, assetID: StringDoaGddGA) -> RpcRequest<AssetID>;
pub fn PlatformGetSubnets(&mut self, ids: Ids) -> RpcRequest<Subnets>;
pub fn PlatformGetStake(&mut self, addresses: Addresses) -> RpcRequest<Staked>;
pub fn PlatformGetTotalStake(&mut self) -> RpcRequest<Stake>;
pub fn PlatformGetTx(&mut self, txID: TxID, encoding: StringDoaGddGA) -> RpcRequest<Subnets>;
pub fn PlatformGetTxStatus(&mut self, txID: TxID) -> RpcRequest<Status>;
pub fn PlatformGetUTXOs(&mut self, addresses: Addresses, limit: NumberHo1ClIqD, startIndex: ObjectOfStringDoaGddGAStringDoaGddGA4SZgw1S0, sourceChain: StringDoaGddGA, encoding: StringDoaGddGA) -> RpcRequest<ObjectOfUnorderedSetOfStringDoaGddGADvj0XlFaNumberHo1ClIqDObjectOfStringDoaGddGAAddressTUFok2IXAnyCVPkZfkLKhuEIl1J>;
pub fn PlatformIssueTx(&mut self, tx: StringDoaGddGA, encoding: StringDoaGddGA) -> RpcRequest<TxID>;
pub fn PlatformSampleValidators(&mut self, size: NumberHo1ClIqD, subnetID: StringDoaGddGA) -> RpcRequest<UnorderedSetOfStringDoaGddGADvj0XlFa>;
pub fn PlatformValidatedBy(&mut self, blockchainID: StringDoaGddGA) -> RpcRequest<SubnetID>;
pub fn PlatformValidates(&mut self, subnetID: StringDoaGddGA) -> RpcRequest<BlockchainIDs>;
pub fn InfoGetBlockchainID(&mut self, alias: StringDoaGddGA) -> RpcRequest<StringDoaGddGA>;
pub fn InfoGetNetworkID(&mut self) -> RpcRequest<NumberHo1ClIqD>;
pub fn InfoGetNetworkName(&mut self) -> RpcRequest<StringDoaGddGA>;
pub fn InfoGetNodeVersion(&mut self) -> RpcRequest<ObjectOfObjectOfStringDoaGddGAStringDoaGddGAStringDoaGddGA5OHTjo2UString9OVXUiHXStringGn1XszHYStringOnhjWynsL4OwtBVJ>;
pub fn InfoIsBootstrapped(&mut self, chain: StringDoaGddGA) -> RpcRequest<BooleanLu7POU17>;
pub fn InfoPeers(&mut self, nodeIDs: UnorderedSetOfStringDoaGddGARCi5AHUB) -> RpcRequest<ObjectOfUnorderedSetOfObjectOfStringDoaGddGAStringDoaGddGAStringDoaGddGAStringDoaGddGAStringDoaGddGAStringDoaGddGAMtoeI6IeJMY8TQNpNumberHo1ClIqDFEjTP7UK>;
pub fn InfoGetTxFee(&mut self) -> RpcRequest<ObjectOfNumberHo1ClIqDNumberHo1ClIqD6FHq3FGd>;
pub fn EthBlockNumber(&mut self) -> RpcRequest<BlockNumberOrTag>;
pub fn EthCall(&mut self, transaction: Transaction, blockNumber: BlockNumberOrTag) -> RpcRequest<Bytes>;
pub fn EthChainId(&mut self) -> RpcRequest<ChainId>;
pub fn EthGetAssetBalance(&mut self, address: Address, blockNumber: BlockNumber, assetID: StringDoaGddGA) -> RpcRequest<IntegerOrNull>;
pub fn EthGetBalance(&mut self, address: Address, blockNumber: BlockNumber) -> RpcRequest<IntegerOrNull>;
pub fn EthSignTransaction(&mut self, Transaction: SignTransaction) -> RpcRequest<Bytes>;
pub fn EthGetTransactionCount(&mut self, address: Address, blockNumber: BlockNumberOrTag) -> RpcRequest<NonceOrNull>;
pub fn EthSendRawTransaction(&mut self, signedTransactionData: Bytes) -> RpcRequest<Keccak>;
pub fn EthGetBlockByHash(&mut self, blockHash: BlockHash, includeTransactions: IsTransactionsIncluded) -> RpcRequest<BlockOrNull>;
pub fn EthGetBlockByNumber(&mut self, blockNumber: BlockNumberOrTag, includeTransactions: IsTransactionsIncluded) -> RpcRequest<BlockOrNull>;
pub fn EthGetTransactionByHash(&mut self, transactionHash: TransactionHash) -> RpcRequest<TransactionOrNull>;
pub fn EthGetTransactionReceipt(&mut self, transactionHash: TransactionHash) -> RpcRequest<TransactionReceiptOrNull>;
pub fn AvaxGetAtomicTxStatus(&mut self, txID: TxID) -> RpcRequest<AtomicTxStatusStatus>;
pub fn AvaxGetUTXOs(&mut self, addresses: UnorderedSetOfStringDoaGddGADvj0XlFa, limit: StringDoaGddGA, sourceChain: StringDoaGddGA, startIndex: StartIndex, encoding: StringDoaGddGA) -> RpcRequest<ObjectHAgrRKSz>;
pub fn AvaxIssueTx(&mut self, tx: StringDoaGddGA, encoding: StringDoaGddGA) -> RpcRequest<TxID>;
pub fn NetVersion(&mut self) -> RpcRequest<NetworkId>;
pub fn Web3ClientVersion(&mut self) -> RpcRequest<ClientVersion>;
pub fn Web3Sha3(&mut self, data: Data) -> RpcRequest<Keccak>;
});

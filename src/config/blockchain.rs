use super::UNIT;

use crate::blockchain::{BlockAndPatch, BlockchainConfig, ZkBlockchainPatch};
use crate::common::*;
use crate::consensus::pow::Difficulty;
use crate::core::{
    Address, Block, ContractId, Header, Money, ProofOfWork, Signature, Transaction,
    TransactionAndDelta, TransactionData, ZkHasher,
};
use crate::zk;

#[cfg(test)]
use crate::wallet::TxBuilder;

const MPN_LOG4_ACCOUNT_CAPACITY: u8 = 3;
const MPN_LOG4_PAYMENT_CAPACITY: u8 = 1;

const TESTNET_HEIGHT_LIMIT: u64 = 5000;

lazy_static! {
    pub static ref MPN_UPDATE_VK: zk::groth16::Groth16VerifyingKey =
        bincode::deserialize(&hex::decode("213f36c08dd39f6fc0bdbf4a0270597d91ade8f0399f36e85f7009310c126c3b02e2e44a43396c350645640daf7f630c1218d5362ded84bd320f577995dd6d1095f4ce9a07be8badcaba05dfae206631f6bdbadb3e8e183cbe48e5175dd14208005f70c17532fa40c6e275c04636399f27595ffcb353cdd6906192bc5d834e9475d271d49cbae1df8dc9de4b0537b070067aa0356819ce8d4b6009267c534a12e022845bc3f6511668807ac8ca094cc5501249c77a049cbb5378cc52b591e00e1900b03ea20ed68171935cbf8a1c3556f8d2f4588157b0c58b7c658db4f858d74e9f54f25dde23ca206add8d28478bee890aca353c4fc6517ee7c38b0caf134b9466583b2275c8b9ef5816084b78760d624a894cb491f8f85ef1b150b8751433c4183e373cd1724596cf68c099a8a2da9e8e26425393183f3f1ef7acb65a50c4476f4fef8323067a3123d5509bfd6d066713db23475cb9d826b29ca8f8d0bb71c594f1543288884fef9a3d7868e7ee32530db3a29e334be7745600446b44748d8e1600869f66ef34f41ccddde72995d39e624ebe092d7aeec6c2ce4444e69bdaad249f9b2b9a9a86a0ac3f48dd17abeaa9680bd689bdf47350f776c2c56a3c1efc7620d646bd88e8ffb88da90dae8ed5645667515ac684062008902067219000e6380cdf02a9c1fb194a703af32029571df3e91451943476bfc8c2b3cc4352be45c1fc59e1f7b54f43ce1cc635a9645e67d90c000d0480f67d4214de3681b86452cbc7966409ef61e78598bc134cbdf6fefc08bf71b5bafd41feafe2fb4f4da51d8107008f8c276c1277fff4820158a975fa9f71fdba96b879d25819eda2585b565914277fd11b94f6d226a1b1054bd78988460eecd6fcb706bf74287fe458f59c481d35ff1827fe63644a37d0ca00b0018c563645ed05ad08b26277445c8d4ee85e631500962f1ed43bf896c4db1dc55cac3192c49f6f540d0fbf2b194560846953b1e9a8e0bf0ae913d6d220e4d77ca083a31419024086eb0b47a6fd562b291a273817be990227de020e1409b88ee38127f989e8564e83be6415ef4d8fcde9edcc0d8e0b5a658fe5db59b717654a263b94ebb7260fb4c7d8055fd0b6677012d4e63a243ffbfcf54df708dcc6136510469cd561058191d79717174187a904d9f944b4c66ded36c3fe192b0f3583b03388fb7918234c51dd7f10d599af689e47e51daef206000500000000000000c9c4cf19e914c9ea4bb751af0a5b728ea2e08fe70e1570b53c43b37fdad84f0b7f6a0cf358239d6fe6183c343ea8eb133d1f11c427483207d128fe0d64665521c74b04ae35fba6c6d9668099dca58d18ab96e671a248d1ed7b73a6d7785d1412002ab75a196069440cbb42749f0ebd2e3c14ba9aaa05366e49f0887c8f97c9682ee8edcb9361ab6ca8ac5cb82699beff025414f446a55ef92eb9b648a3d43b11a98aa74cd76ef30984be9e2e8dd0661f0c645e0f483d99e0a8fc09421228114706005abc3e4fbf469719d11c28be8654c51ff14cf6ec7734483d1e665f1d1a8b3c6eec4dda2dceb5c0420ff68716dc10e917605bdf1ae0a8f4b0433663720a38d86a39551dce436821de87a687e5748b500fae04da91938a8411ebe719b93c82b40f0091fbcdefeba8cad3fa871f86c0c01ff7e7ef8750ba86ba88cce6fdcf9bf5f95101c0fff6241f6d9ed29a84e0f712390752a0d1e2b5901bb07e29964dd428b0a1fe26a8cc0f93146f950421d4235a11f63856ad6d09901b5df23922801612b501000a385dd246598295c9755c7318addc25e2cb6d7ee4b1e78947a56858bc3bb85b5625630a8b27b53ad4eba7ab6263750e2b7d5a3ab65e82b92a66d41a61783427e145f21840f85c5bbf92091f7de9d602ab198016222faa6c394573f3f3fa2d0f00").unwrap()).unwrap();
    pub static ref MPN_DEPOSIT_VK: zk::groth16::Groth16VerifyingKey =
        bincode::deserialize(&hex::decode("213f36c08dd39f6fc0bdbf4a0270597d91ade8f0399f36e85f7009310c126c3b02e2e44a43396c350645640daf7f630c1218d5362ded84bd320f577995dd6d1095f4ce9a07be8badcaba05dfae206631f6bdbadb3e8e183cbe48e5175dd14208005f70c17532fa40c6e275c04636399f27595ffcb353cdd6906192bc5d834e9475d271d49cbae1df8dc9de4b0537b070067aa0356819ce8d4b6009267c534a12e022845bc3f6511668807ac8ca094cc5501249c77a049cbb5378cc52b591e00e1900b03ea20ed68171935cbf8a1c3556f8d2f4588157b0c58b7c658db4f858d74e9f54f25dde23ca206add8d28478bee890aca353c4fc6517ee7c38b0caf134b9466583b2275c8b9ef5816084b78760d624a894cb491f8f85ef1b150b8751433c4183e373cd1724596cf68c099a8a2da9e8e26425393183f3f1ef7acb65a50c4476f4fef8323067a3123d5509bfd6d066713db23475cb9d826b29ca8f8d0bb71c594f1543288884fef9a3d7868e7ee32530db3a29e334be7745600446b44748d8e1600869f66ef34f41ccddde72995d39e624ebe092d7aeec6c2ce4444e69bdaad249f9b2b9a9a86a0ac3f48dd17abeaa9680bd689bdf47350f776c2c56a3c1efc7620d646bd88e8ffb88da90dae8ed5645667515ac684062008902067219000e6380cdf02a9c1fb194a703af32029571df3e91451943476bfc8c2b3cc4352be45c1fc59e1f7b54f43ce1cc635a9645e67d90c000d0480f67d4214de3681b86452cbc7966409ef61e78598bc134cbdf6fefc08bf71b5bafd41feafe2fb4f4da51d8107008f8c276c1277fff4820158a975fa9f71fdba96b879d25819eda2585b565914277fd11b94f6d226a1b1054bd78988460eecd6fcb706bf74287fe458f59c481d35ff1827fe63644a37d0ca00b0018c563645ed05ad08b26277445c8d4ee85e631500962f1ed43bf896c4db1dc55cac3192c49f6f540d0fbf2b194560846953b1e9a8e0bf0ae913d6d220e4d77ca083a31419024086eb0b47a6fd562b291a273817be990227de020e1409b88ee38127f989e8564e83be6415ef4d8fcde9edcc0d8e0b5a658fe5db59b717654a263b94ebb7260fb4c7d8055fd0b6677012d4e63a243ffbfcf54df708dcc6136510469cd561058191d79717174187a904d9f944b4c66ded36c3fe192b0f3583b03388fb7918234c51dd7f10d599af689e47e51daef206000500000000000000ff4be99e665746c104adff326b079c99bbc963871e2b47907909636451b70548c03bfc95243e73f1f1949be34ea67919971a372f932d01372bb88554e2dc58a0b858a1ccd246c096a2c32a1cfa71d1b3d80aa461b2788b32ea7ddb4c6a575f0a00ceb3004059aeb768af0270c2a32f7168f56840c969d0c6bc9ed3a8ccff142b17d7f56ee7f8cbe45615f6e8ea2ac97902c57fece9c100e835eeaae2f948af416aaf5ead4bb88389522cc53a31ec06957bbb5409929ea4efb93d3d3359fc6c4d02003021bd5d82d56e312993ca409e81973e3a3c493ca3f1fc5f854026aeba7f0c0de62e93edfa235f3ff4e585b9039e89045039304c4979a2faaf0c8216f262c906a9354faf9c71a86813b00d8e7ade0c4776c1b1e99e2bbe364da304cf231ad90d002b4e67944ff17e915158a2f6ad9cd3013994f80180c070cbbbfc2f866d701a0f2c4f727f3132fbd059e385e60b226f0b1f4c1bb5d32bfc23b5457b18602bba966093eb3c05e0a19555ad4ed03b2c8b05e78b0fb297443d94c4f8b4ade52d6913003a7d6bfd820b3c7969f1bf65b2e3fa3ed27c16a9270ba191caa0ce3dc393550319deaddfff7ac0cda704ae4c6d35f017719dd91dcc9e7b8e6ec85a02e862dca8b8f517bfe4f8a0ab0cc870310c8043da5f1ec7c2ef9ce8f3a905cc4393cd6b1300").unwrap()).unwrap();
    pub static ref MPN_WITHDRAW_VK: zk::groth16::Groth16VerifyingKey =
        bincode::deserialize(&hex::decode("213f36c08dd39f6fc0bdbf4a0270597d91ade8f0399f36e85f7009310c126c3b02e2e44a43396c350645640daf7f630c1218d5362ded84bd320f577995dd6d1095f4ce9a07be8badcaba05dfae206631f6bdbadb3e8e183cbe48e5175dd14208005f70c17532fa40c6e275c04636399f27595ffcb353cdd6906192bc5d834e9475d271d49cbae1df8dc9de4b0537b070067aa0356819ce8d4b6009267c534a12e022845bc3f6511668807ac8ca094cc5501249c77a049cbb5378cc52b591e00e1900b03ea20ed68171935cbf8a1c3556f8d2f4588157b0c58b7c658db4f858d74e9f54f25dde23ca206add8d28478bee890aca353c4fc6517ee7c38b0caf134b9466583b2275c8b9ef5816084b78760d624a894cb491f8f85ef1b150b8751433c4183e373cd1724596cf68c099a8a2da9e8e26425393183f3f1ef7acb65a50c4476f4fef8323067a3123d5509bfd6d066713db23475cb9d826b29ca8f8d0bb71c594f1543288884fef9a3d7868e7ee32530db3a29e334be7745600446b44748d8e1600869f66ef34f41ccddde72995d39e624ebe092d7aeec6c2ce4444e69bdaad249f9b2b9a9a86a0ac3f48dd17abeaa9680bd689bdf47350f776c2c56a3c1efc7620d646bd88e8ffb88da90dae8ed5645667515ac684062008902067219000e6380cdf02a9c1fb194a703af32029571df3e91451943476bfc8c2b3cc4352be45c1fc59e1f7b54f43ce1cc635a9645e67d90c000d0480f67d4214de3681b86452cbc7966409ef61e78598bc134cbdf6fefc08bf71b5bafd41feafe2fb4f4da51d8107008f8c276c1277fff4820158a975fa9f71fdba96b879d25819eda2585b565914277fd11b94f6d226a1b1054bd78988460eecd6fcb706bf74287fe458f59c481d35ff1827fe63644a37d0ca00b0018c563645ed05ad08b26277445c8d4ee85e631500962f1ed43bf896c4db1dc55cac3192c49f6f540d0fbf2b194560846953b1e9a8e0bf0ae913d6d220e4d77ca083a31419024086eb0b47a6fd562b291a273817be990227de020e1409b88ee38127f989e8564e83be6415ef4d8fcde9edcc0d8e0b5a658fe5db59b717654a263b94ebb7260fb4c7d8055fd0b6677012d4e63a243ffbfcf54df708dcc6136510469cd561058191d79717174187a904d9f944b4c66ded36c3fe192b0f3583b03388fb7918234c51dd7f10d599af689e47e51daef20600050000000000000089449131a16665235e07e2514af2acac876d0517413570a75fd37c432f67b5fb39cb62eb535dba4d82505bddcabbb901d0a6e70b078843b4209e2527e02cb9f56cd3de446a0d6dd11560b4a9b027b824a6d0733d2e3287a39d5ce317bf2ad405005b899755c68e66b0b9600137069a28d0863a37eda17eca79b9c7bfd9490471502f913f14c84891cfd55cd8b540c9b110ffd8cdab352e2e03b375f278edf5eaeed7a78870d390d41dcd828bc0b76f1b92984cd98b03b0d935968f4180da8d290a0086dd41f2b0f32c35417a9aae9c20360cdaa14a417ce919da69d1d56d6edefca44c1a577ad0fa801b502f20a373f3230ce999939b8e8d58e9830ae9020800188270d04908566b36073de40dec782e9376558ba604395614841bbee4741262b6130055f582bc63f17acec0fd467a6fee03a33aae5294894a1df9d33bf16cf99106dd1273cc8d06e7ef30e9b9052654f1f70d794c11894e7dcdc92eb12fbc634454a7bc51999da47b386f72042ce29b1c0a5c1da1ea90ce9af2edce7509169aa60e04009846d4289db4126a33cc262527f78c778d4a46ba15622ea5d6e57c7e265fc790725f980cf42cfcdab94b75ecfe3b6813f9f6defcde58f9e465e5470f82f63f34b41d54b8f7e224f9fbfa903c1524ddc90df801177f04ab4225f5562afb29e81600").unwrap()).unwrap();
}

fn get_mpn_contract() -> TransactionAndDelta {
    let mpn_state_model = zk::ZkStateModel::List {
        log4_size: MPN_LOG4_ACCOUNT_CAPACITY,
        item_type: Box::new(zk::ZkStateModel::Struct {
            field_types: vec![
                zk::ZkStateModel::Scalar, // Nonce
                zk::ZkStateModel::Scalar, // Pub-key X
                zk::ZkStateModel::Scalar, // Pub-key Y
                zk::ZkStateModel::Scalar, // Balance
            ],
        }),
    };
    let mpn_contract = zk::ZkContract {
        state_model: mpn_state_model.clone(),
        initial_state: zk::ZkCompressedState::empty::<ZkHasher>(mpn_state_model),
        deposit_functions: vec![zk::ZkMultiInputVerifierKey {
            verifier_key: zk::ZkVerifierKey::Groth16(Box::new(MPN_DEPOSIT_VK.clone())),
            log4_payment_capacity: MPN_LOG4_PAYMENT_CAPACITY,
        }],
        withdraw_functions: vec![zk::ZkMultiInputVerifierKey {
            verifier_key: zk::ZkVerifierKey::Groth16(Box::new(MPN_WITHDRAW_VK.clone())),
            log4_payment_capacity: MPN_LOG4_PAYMENT_CAPACITY,
        }],
        functions: vec![zk::ZkSingleInputVerifierKey {
            verifier_key: zk::ZkVerifierKey::Groth16(Box::new(MPN_UPDATE_VK.clone())),
        }],
    };
    let mpn_contract_create_tx = Transaction {
        src: Address::Treasury,
        data: TransactionData::CreateContract {
            contract: mpn_contract,
        },
        nonce: 1,
        fee: Money(0),
        sig: Signature::Unsigned,
    };
    TransactionAndDelta {
        tx: mpn_contract_create_tx,
        state_delta: Some(zk::ZkDeltaPairs::default()),
    }
}

#[cfg(test)]
fn get_test_mpn_contract() -> TransactionAndDelta {
    let mut mpn_tx_delta = get_mpn_contract();
    let init_state = zk::ZkDataPairs(
        [(zk::ZkDataLocator(vec![100]), zk::ZkScalar::from(200))]
            .into_iter()
            .collect(),
    );
    match &mut mpn_tx_delta.tx.data {
        TransactionData::CreateContract { contract } => {
            contract.state_model = zk::ZkStateModel::List {
                log4_size: 5,
                item_type: Box::new(zk::ZkStateModel::Scalar),
            };
            contract.initial_state = contract
                .state_model
                .compress::<ZkHasher>(&init_state)
                .unwrap();
            contract.deposit_functions = vec![zk::ZkMultiInputVerifierKey {
                verifier_key: zk::ZkVerifierKey::Dummy,
                log4_payment_capacity: 1,
            }];
            contract.withdraw_functions = vec![zk::ZkMultiInputVerifierKey {
                verifier_key: zk::ZkVerifierKey::Dummy,
                log4_payment_capacity: 1,
            }];
            contract.functions = vec![zk::ZkSingleInputVerifierKey {
                verifier_key: zk::ZkVerifierKey::Dummy,
            }];
        }
        _ => panic!(),
    }
    mpn_tx_delta.state_delta = Some(init_state.as_delta());
    mpn_tx_delta
}

pub fn get_blockchain_config() -> BlockchainConfig {
    let mpn_tx_delta = get_mpn_contract();
    let mpn_contract_id = ContractId::new(&mpn_tx_delta.tx);

    let min_diff = Difficulty(0x0100ffff);

    let blk = Block {
        header: Header {
            parent_hash: Default::default(),
            number: 0,
            block_root: Default::default(),
            proof_of_work: ProofOfWork {
                timestamp: 0,
                target: min_diff,
                nonce: 0,
            },
        },
        body: vec![mpn_tx_delta.tx],
    };

    BlockchainConfig {
        limited_miners: None,
        mpn_contract_id,
        genesis: BlockAndPatch {
            block: blk,
            patch: ZkBlockchainPatch {
                patches: [(
                    mpn_contract_id,
                    zk::ZkStatePatch::Delta(mpn_tx_delta.state_delta.unwrap()),
                )]
                .into_iter()
                .collect(),
            },
        },
        total_supply: Money(2_000_000_000_u64 * UNIT), // 2 Billion ZIK
        reward_ratio: 100_000, // 1/100_000 -> 0.01% of Treasury Supply per block
        max_block_size: (1 * MB) as usize,
        max_delta_count: 1024, // Only allow max of 1024 ZkScalar cells to be added per block
        block_time: 60,        // Seconds
        difficulty_calc_interval: 128, // Blocks

        // 0 63 -> BAZUKA BASE KEY
        // 64 2111 -> hash(blk#0)
        // 2112 4159 -> hash(blk#2048)
        // 4160 6207 -> hash(blk#4096)
        // ...
        pow_base_key: b"BAZUKA BASE KEY",
        pow_key_change_delay: 64,      // Blocks
        pow_key_change_interval: 2048, // Blocks

        // New block's timestamp should be higher than median
        // timestamp of 10 previous blocks
        median_timestamp_count: 10,

        // We expect a minimum number of MPN contract updates
        // in a block to consider it valid
        mpn_num_function_calls: 0,
        mpn_num_contract_deposits: 0,
        mpn_num_contract_withdraws: 0,

        minimum_pow_difficulty: min_diff,

        testnet_height_limit: Some(TESTNET_HEIGHT_LIMIT),
    }
}

#[cfg(test)]
pub fn get_test_blockchain_config() -> BlockchainConfig {
    let mpn_tx_delta = get_test_mpn_contract();
    let mpn_contract_id = ContractId::new(&mpn_tx_delta.tx);

    let min_diff = Difficulty(0x007fffff);

    let mut conf = get_blockchain_config();
    conf.limited_miners = None;
    conf.mpn_num_contract_deposits = 0;
    conf.mpn_num_contract_withdraws = 0;
    conf.mpn_num_function_calls = 0;
    conf.mpn_contract_id = mpn_contract_id;
    conf.minimum_pow_difficulty = min_diff;
    conf.genesis.block.header.proof_of_work.target = min_diff;
    conf.testnet_height_limit = None;

    conf.genesis.block.body[0] = get_test_mpn_contract().tx;
    let abc = TxBuilder::new(&Vec::from("ABC"));
    conf.genesis.block.body.push(Transaction {
        src: Address::Treasury,
        data: TransactionData::RegularSend {
            dst: abc.get_address(),
            amount: Money(10000),
        },
        nonce: 2,
        fee: Money(0),
        sig: Signature::Unsigned,
    });
    conf.genesis.patch = ZkBlockchainPatch {
        patches: [(
            mpn_contract_id,
            zk::ZkStatePatch::Delta(mpn_tx_delta.state_delta.unwrap()),
        )]
        .into_iter()
        .collect(),
    };
    conf
}

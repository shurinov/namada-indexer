use crate::balance::Amount;
use crate::id::Id;

#[derive(Debug, Clone)]
pub enum MaspEntryDirection {
    In,
    Out,
}

#[derive(Debug, Clone)]
pub struct MaspEntry {
    pub token_address: String,
    pub timestamp: i64,
    pub raw_amount: Amount,
    pub direction: MaspEntryDirection,
    pub inner_tx_id: Id,
}

#[derive(Debug, Clone)]
pub struct MaspRewardData {
    pub address: Id,
    pub max_reward_rate: String,
    pub kp_gain: String,
    pub kd_gain: String,
    pub locked_amount_target: Amount,
}

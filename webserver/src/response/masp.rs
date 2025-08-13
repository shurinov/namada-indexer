use serde::{Deserialize, Serialize};
use shared::masp::MaspRewardData;

use crate::entity::masp::{
    MaspPoolAggregate, MaspPoolAggregateKind, MaspPoolAggregateWindow,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum MaspPoolAggregateWindowResponse {
    OneDay,
    SevenDays,
    ThirtyDays,
    AllTime,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum MaspPoolAggregateKindResponse {
    Inflows,
    Outflows,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MaspPoolAggregateResponse {
    pub token_address: String,
    pub time_window: MaspPoolAggregateWindowResponse,
    pub kind: MaspPoolAggregateKindResponse,
    pub total_amount: String,
}

impl From<MaspPoolAggregate> for MaspPoolAggregateResponse {
    fn from(value: MaspPoolAggregate) -> Self {
        Self {
            token_address: value.token_address.to_string(),
            time_window: match value.time_window {
                MaspPoolAggregateWindow::OneDay => {
                    MaspPoolAggregateWindowResponse::OneDay
                }
                MaspPoolAggregateWindow::SevenDays => {
                    MaspPoolAggregateWindowResponse::SevenDays
                }
                MaspPoolAggregateWindow::ThirtyDays => {
                    MaspPoolAggregateWindowResponse::ThirtyDays
                }
                MaspPoolAggregateWindow::AllTime => {
                    MaspPoolAggregateWindowResponse::AllTime
                }
            },
            kind: match value.kind {
                MaspPoolAggregateKind::Inflows => {
                    MaspPoolAggregateKindResponse::Inflows
                }
                MaspPoolAggregateKind::Outflows => {
                    MaspPoolAggregateKindResponse::Outflows
                }
            },
            total_amount: value.total_amount.to_string(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MaspRewardDataResponse {
    pub address: String,
    pub max_reward_rate: String,
    pub kp_gain: String,
    pub kd_gain: String,
    pub locked_amount_target: String,
}

impl From<MaspRewardData> for MaspRewardDataResponse {
    fn from(value: MaspRewardData) -> Self {
        Self {
            address: value.address.to_string(),
            max_reward_rate: value.max_reward_rate.to_string(),
            kp_gain: value.kp_gain.to_string(),
            kd_gain: value.kd_gain.to_string(),
            locked_amount_target: value.locked_amount_target.to_string(),
        }
    }
}

use anyhow::Context;
use diesel::upsert::excluded;
use diesel::{ExpressionMethods, PgConnection, RunQueryDsl};
use orm::masp::MaspRewardDataInsertDb;
use orm::schema::masp_rates;
use shared::masp::MaspRewardData;

pub fn insert_masp_rates(
    transaction_conn: &mut PgConnection,
    masp_reward_data: Vec<MaspRewardData>,
) -> anyhow::Result<()> {
    diesel::insert_into(masp_rates::table)
        .values(
            masp_reward_data
                .into_iter()
                .map(MaspRewardDataInsertDb::from)
                .collect::<Vec<_>>(),
        )
        .on_conflict(masp_rates::columns::token)
        .do_update()
        .set((
            masp_rates::columns::max_reward_rate
                .eq(excluded(masp_rates::columns::max_reward_rate)),
            masp_rates::columns::kp_gain
                .eq(excluded(masp_rates::columns::kp_gain)),
            masp_rates::columns::kd_gain
                .eq(excluded(masp_rates::columns::kd_gain)),
            masp_rates::columns::locked_amount_target
                .eq(excluded(masp_rates::columns::locked_amount_target)),
        ))
        .execute(transaction_conn)
        .context("Failed to update masp rates in db")?;

    Ok(())
}

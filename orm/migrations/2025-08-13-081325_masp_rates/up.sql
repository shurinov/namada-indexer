-- Your SQL goes here
CREATE TABLE masp_rates (
    token VARCHAR PRIMARY KEY NOT NULL,
    max_reward_rate VARCHAR NOT NULL,
    kp_gain VARCHAR NOT NULL,
    kd_gain VARCHAR NOT NULL,
    locked_amount_target NUMERIC(78, 0) NOT NULL
);
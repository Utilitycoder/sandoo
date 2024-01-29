use anyhow::Result;
use ethers::core::rand::thread_rng;
use ethers::prelude::*;
use ethers::{
    self,
    types::{
        transaction::eip2930::{AccessList, AccessListItem},
        U256,
    },
};
use fern::colors::{Color, ColoredLevelConfig};
use foundry_evm_mini::evm::utils::{b160_to_h160, h160_to_b160, ru256_to_u256, u256_to_ru256};
use log::LevelFilter;
use rand::Rng;
use revm::primitives::{B160, U256 as rU256};
use std::str::FromStr;
use std::sync::Arc;

use crate::common::constants::{PROJECT_NAME, WETH};

// Format our console logs
pub fn setup_logger() -> Result<()> {
    let colors = ColoredLevelConfig {
        trace: Color::Cyan,
        debug: Color::Magenta,
        info: Color::Green,
        warn: Color::Red,
        error: Color::BrightRed,
    };

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{}[{}] {}",
                chrono::Local::now().format("[%H:%M:%S]"),
                colors.color(record.level()),
                message
            ))
        })
        .chain(std::io::stdout())
        .level(log::LevelFilter::Error)
        .level_for(PROJECT_NAME, LevelFilter::Info)
        .apply()?;
    Ok(())
}

// Calculates the next block base fee given the previous block's gas usage / limits
// Refer to: https://www.blocknative.com/blog/eip-1559-fees
pub fn calculate_next_block_base_fee(
    gas_used: U256,
    gas_limit: U256,
    base_fee_per_gas: U256,
) -> U256 {
    let gas_used = gas_used;

    let mut target_gas_used = gas_limit / 2;

    target_gas_used = if target_gas_used == U256::zero() {
        U256::one()
    } else {
        target_gas_used
    };

    let new_base_fee = {
        if gas_used > target_gas_used {
            base_fee_per_gas
                + ((base_fee_per_gas * (gas_used - target_gas_used)) / target_gas_used)
                    / U256::from(8u64)
        } else {
            base_fee_per_gas
                - ((base_fee_per_gas * (target_gas_used - gas_used)) / target_gas_used)
                    / U256::from(8u64)
        }
    };
}

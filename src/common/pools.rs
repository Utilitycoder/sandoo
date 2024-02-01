use anyhow::{Ok, Result};
use csv::StringRecord;
use ethers::abi::{parse_abi, ParamType};
use ethers::prelude::*;
use ethers::{
    providers::{Provider, Ws},
    types::{H160, H256},
};
use indicatif::{ProgressBar, ProgressStyle};
use itertools::Itertools;
use log::info;
use serde::{Deserialize, Serialize};
use std::fmt::format;
use std::{collections::HashMap, fs::OpenOptions, path::Path, str::FromStr, sync::Arc};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum DexVariant {
    UniswapV2, // 2
}

impl DexVariant {
    pub fn num(&self) -> u8 {
        match self {
            DexVariant::UniswapV2 => 2,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Pool {
    pub id: i64,
    pub address: H160,
    pub version: DexVariant,
    pub token0: H160,
    pub token1: H160,
    pub fee: u32, // uniswap v3 specific
    pub block_number: u64,
    pub timestamp: u64,
}

impl From<StringRecord> for Pool {
    fn from(record: StringRecord) -> Self {
        let version = match record.get(2).unwrap().parse().unwrap() {
            2 => DexVariant::UniswapV2,
            _ => DexVariant::UniswapV2,
        };
        Self {
            id: record.get(0).unwrap().parse().unwrap(),
            address: H160::from_str(record.get(1).unwrap()).unwrap(),
            version,
            token0: H160::from_str(record.get(3).unwrap()).unwrap(),
            token1: H160::from_str(record.get(4).unwrap()).unwrap(),
            fee: record.get(5).unwrap().parse().unwrap(),
            block_number: record.get(6).unwrap().parse().unwrap(),
            timestamp: record.get(7).unwrap().parse().unwrap(),
        }
    }
}

impl Pool {
    pub fn cache_row(&self) -> (i64, String, i32, String, String, u32, u64, u64) {
        (
            self.id,
            format!("{:?}", self.address),
            self.version.num() as i32,
            format!("{:?}", self.token0),
            format!("{:?}", self.token1),
            self.fee,
            self.block_number,
            self.timestamp,
        )
    }

    pub fn trades(&self, token_a: H160, token_b: H160) -> bool {
        let is_zero_for_one = self.token0 == token_a && self.token1 == token_b;
        let is_one_for_zero = self.token1 == token_a && self.token0 == token_b;
        is_zero_for_one || is_one_for_zero
    }

    pub fn pretty_msg(&self) -> String {
        format!(
            "[{:?}] {:?}: {:?} --> {:?}",
            self.version, self.address, self.token0, self.token1
        )
    }

    pub fn pretty_print(&self) {
        info!("{}", self.pretty_msg());
    }
}

pub async fn get_touched_pools(
    provider: &Arc<Provider<Ws>>,
    block_number: u64,
) -> Result<Vec<H160>> {
    let v2_swap_event = "Swap(address,uint256,uint256,uint256,uint256,address)";
    let event_filter = Filter::new()
        .from_block(block_number)
        .to_block(block_number)
        .events(vec![v2_swap_event]);
    let logs = provider.get_logs(&event_filter).await?;
    let touched_pools: Vec<H160> = logs.iter().map(|log| log.address).unique().collect();
    Ok(touched_pools)
}

pub async fn load_all_pools(
    wss_url: String,
    from_block: u64,
    chunk: u64,
) -> Result<(Vec<Pool>, i64)> {
    let cache_file = "cache/.cached-pools.csv";
    let file_path = Path::new(cache_file);
    let file_exists = file_path.exists();
    let file = OpenOptions::new().append(true).create(true).open(file_path).unwrap();
    let mut writer = csv::Writer::from_writer(file);

    let mut pools = Vec::new();

    let mut v2_pool_cnt = 0;

    if file_exists {
        let mut reader = csv::Reader::from_path(file_path)?;

        for row in reader.records() {
            let row = row.unwrap();
            let pool = Pool::from(row);
            match pool.version {
                DexVariant::UniswapV2 => v2_pool_cnt += 1,
            } 
            pools.push(pool)
        }
    } else {
        writer.write_record(&[
            "id",
            "address",
            "version",
            "token0",
            "token1",
            "fee",
            "block_number",
            "timestamp",
        ])?;
    }
    info!("Pools loaded: {:?}", pools.len());
    info!("V2 pools: {:?}", v2_pool_cnt);

    let ws = Ws::connect(wss_url).await?;
    let provider = Arc::new(Provider::new(ws));

    // Uniswap V2
    let pair_created_event = "PairCreated(address,address,address,uint256)";
    let abi = parse_abi(&[&format!("event {}", pair_created_event)]).unwrap();

    
}

ghgjkk

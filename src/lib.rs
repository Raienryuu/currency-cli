use std::collections::HashMap;
use std::env;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde::Deserialize;

use crate::exchange_rates::database::DbConnection;
use crate::exchange_rates::open_exchange_api::get_current_exchangerates;

pub mod exchange_rates;

struct ConvertData {
    from: String,
    to: String,
    value: f64,
}

pub fn get_arguments() -> Vec<String> {
    env::args().collect()
}

pub fn match_args(args: Vec<String>, db: DbConnection) {
    match args.iter().count() {
        1 => start_ui(),
        2 => recognize_arg(args),
        4 => create_request_args(args, db),
        _ => panic!("Invalid arguments. Check -h for help."),
    }
}

fn start_ui() {}

fn recognize_arg(_args: Vec<String>) {}

fn create_request_args(args: Vec<String>, db: DbConnection) {
    let from: String = String::from(args[2].trim()).to_uppercase();
    let to: String = String::from(args[3].trim()).to_uppercase();
    let value = get_amount(args[1].trim());
    let cd = ConvertData { from, to, value };

    calculate_currency(cd, db);
}

fn calculate_currency(_cd: ConvertData, db: DbConnection) {
    let _now = get_current_timestamp();
    if is_data_old(_now.as_secs(), db.get_data_timestamp()) {
        let data = get_current_exchangerates();
        db.update_rates(data);
    }

    let (from, to) = db.find_rates(&_cd.from, &_cd.to);
    let output = _cd.value / from * to;
    println!("{} {} is equal to {:.6} {}", _cd.value, _cd.from, output, _cd.to);
    println!("Using {}/{}: {:.6}, {}/{}: {:.6}", _cd.from, _cd.to, (from / to),
             _cd.to, _cd.from, (to / from));
}

fn get_current_timestamp() -> Duration {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
}

fn is_data_old(now: u64, data: u64) -> bool {
    data + 3600 < now
}

#[derive(Deserialize)]
pub struct ExchangeRate {
    disclaimer: String,
    license: String,
    timestamp: u64,
    base: String,
    rates: HashMap<String, f64>,
}

fn get_amount(arg: &str) -> f64 {
    let parse_result: f64 = arg.parse().expect("Invalid 'amount' argument.");

    if parse_result <= 0f64 {
        panic!("'amount' should be greater than 0");
    }

    parse_result
}

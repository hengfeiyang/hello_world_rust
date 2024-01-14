use once_cell::sync::Lazy;
use promql_parser::parser::token::{self, TokenId};
use std::collections::HashMap;

type PolicyType = fn(i64) -> Result<(), ()>;

fn sum(_i: i64) -> Result<(), ()> {
    Ok(())
}

pub static POLICY: Lazy<HashMap<TokenId, PolicyType>> = Lazy::new(|| {
    let mut map: HashMap<TokenId, PolicyType> = HashMap::with_capacity(8);
    map.insert(token::T_SUM, sum);
    map.insert(token::T_AVG, sum);
    map.insert(token::T_COUNT, sum);
    map.insert(token::T_MIN, sum);
    map.insert(token::T_MAX, sum);
    map.insert(token::T_GROUP, sum);
    map.insert(token::T_STDDEV, sum);
    map.insert(token::T_STDVAR, sum);
    map
});

pub fn run(op: &token::TokenType) -> Result<(), ()> {
    match POLICY.get(&op.id()) {
        Some(_) => {}
        None => match op.id() {
            token::T_TOPK => {}
            token::T_BOTTOMK => {}
            token::T_COUNT_VALUES => {}
            token::T_QUANTILE => {}
            _ => {}
        },
    }
    Ok(())
}

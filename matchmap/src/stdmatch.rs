use promql_parser::parser::token;

pub fn run(op: &token::TokenType) -> Result<(), ()> {
    Ok(match op.id() {
        token::T_SUM => {}
        token::T_AVG => {}
        token::T_COUNT => {}
        token::T_MIN => {}
        token::T_MAX => {}
        token::T_GROUP => {}
        token::T_STDDEV => {}
        token::T_STDVAR => {}
        token::T_TOPK => {}
        token::T_BOTTOMK => {}
        token::T_COUNT_VALUES => {}
        token::T_QUANTILE => {}
        _ => {}
    })
}

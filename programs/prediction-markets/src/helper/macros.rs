#[macro_export]
macro_rules! check_zero {
    ($arr:expr) => {
            if $arr.iter().any(|amount| *amount == Decimal::ZERO) {
                return err!(MarketError::ZeroAmount);
            }
    };
}

#[macro_export]
macro_rules! add_or_sub {
    ($value_one:expr, $value_two:expr, $is_add:expr) => {
            if $is_add {
                match $value_one.checked_add($value_two) {
                    Some(val) => Ok(val),
                    None => err!(MarketError::ArithemeticOverflow)
                }
            } else {
                match $value_one.checked_sub($vaule.two) {
                    Some(val) => Ok(val),
                    None => err!(MarketError::ArithemeticUnderflow),
                }
            }
    };
}

#[macro_export]
macro_rules! div {
    ($value_one:expr, $value_two:expr) => {
            match $value_one.checked_div($value_two) {
                Some(val) => val,
                None => return Err(MarketError::ArithemeticError)
            }
    };
}

#[macro_export]
macro_rules! decimal_convo {
    ($value:expr) => {
            Decimal::from($value)
    };
}

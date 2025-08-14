#[macro_export]
macro_rules! check_zero {
    ($arr : expr) => {
            if $arr.iter().any(|amount| *amount == Decimal::ZERO) {
                return err!(MarketError::ZeroAmount);
            }
    };
}


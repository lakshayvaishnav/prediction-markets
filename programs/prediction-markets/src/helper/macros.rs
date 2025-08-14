use crate::MarketError::*;

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
                match $value_one.checked_sub($value_two) {
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
                None => return Err(MarketError::ArithemeticError.into())
            }
    };
}

#[macro_export]
macro_rules! mul {
    ($value_one:expr, $value_two:expr) => {
            match $value_one.checked_mul($value_two) {
                Some(val) => val,
                None => return err!(MarketError::ArithemeticOverflow)
            }
    };
}

#[macro_export]
macro_rules! decimal_convo {
    ($value:expr) => {
            Decimal::from($value)
    };
}

#[macro_export]
macro_rules! check_ban {
    ($ban:expr) => {
            if $ban {
                return err!(MarketError::Banned)
            }
    };
}

#[macro_export]
macro_rules! check_admin {
    ($self:expr) => {
        let admin_check = $self
        .platform_config
        .admin
        .iter()
        .any(|admin_pubkey| $self.admin.key() == *admin_pubkey);

        if !admin_check {
            return err!(MarketError::Unauthorized)
        }
    } 
}
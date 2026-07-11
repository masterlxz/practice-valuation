/// `current_price` must be the *latest* `stock_quotes.price` for the rule's
/// ticker — not `valuation.current_price`, which is frozen at calc time and
/// would make this check a permanent no-op.
pub fn evaluate_stock_price(condition: &str, fair_price: f64, current_price: f64) -> bool {
    match condition {
        "BELOW_FAIR_PRICE" => current_price < fair_price,
        "ABOVE_FAIR_PRICE" => current_price > fair_price,
        _ => false, // unreachable given creation-time validation in commands/alert_rule.rs
    }
}

pub fn evaluate_crypto_indicator(condition: &str, current_signal: &str) -> bool {
    match condition {
        "SIGNAL_GREEN" => current_signal == "GREEN",
        "SIGNAL_RED" => current_signal == "RED",
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn below_fair_price_triggers_when_current_price_is_lower() {
        assert!(evaluate_stock_price("BELOW_FAIR_PRICE", 35.0, 30.0));
    }

    #[test]
    fn below_fair_price_does_not_trigger_when_current_price_is_higher_or_equal() {
        assert!(!evaluate_stock_price("BELOW_FAIR_PRICE", 35.0, 35.0));
        assert!(!evaluate_stock_price("BELOW_FAIR_PRICE", 35.0, 40.0));
    }

    #[test]
    fn above_fair_price_triggers_when_current_price_is_higher() {
        assert!(evaluate_stock_price("ABOVE_FAIR_PRICE", 35.0, 40.0));
    }

    #[test]
    fn above_fair_price_does_not_trigger_when_current_price_is_lower_or_equal() {
        assert!(!evaluate_stock_price("ABOVE_FAIR_PRICE", 35.0, 35.0));
        assert!(!evaluate_stock_price("ABOVE_FAIR_PRICE", 35.0, 30.0));
    }

    #[test]
    fn stock_price_unknown_condition_never_triggers() {
        assert!(!evaluate_stock_price("SOMETHING_ELSE", 35.0, 10.0));
    }

    #[test]
    fn signal_green_triggers_on_green() {
        assert!(evaluate_crypto_indicator("SIGNAL_GREEN", "GREEN"));
    }

    #[test]
    fn signal_green_does_not_trigger_on_red_or_neutral() {
        assert!(!evaluate_crypto_indicator("SIGNAL_GREEN", "RED"));
        assert!(!evaluate_crypto_indicator("SIGNAL_GREEN", "NEUTRAL"));
    }

    #[test]
    fn signal_red_triggers_on_red() {
        assert!(evaluate_crypto_indicator("SIGNAL_RED", "RED"));
    }

    #[test]
    fn signal_red_does_not_trigger_on_green_or_neutral() {
        assert!(!evaluate_crypto_indicator("SIGNAL_RED", "GREEN"));
        assert!(!evaluate_crypto_indicator("SIGNAL_RED", "NEUTRAL"));
    }

    #[test]
    fn crypto_indicator_unknown_condition_never_triggers() {
        assert!(!evaluate_crypto_indicator("SOMETHING_ELSE", "GREEN"));
    }
}

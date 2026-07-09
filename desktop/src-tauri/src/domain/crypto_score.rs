use crate::error::AppError;

pub struct Threshold {
    pub green_boundary: f64,
    pub red_boundary: f64,
}

pub enum Signal {
    Green,
    Neutral,
    Red,
}

impl Signal {
    pub fn as_str(&self) -> &'static str {
        match self {
            Signal::Green => "GREEN",
            Signal::Neutral => "NEUTRAL",
            Signal::Red => "RED",
        }
    }
}

// Direction is inferred from the boundaries themselves instead of a stored
// flag: green_boundary > red_boundary means "higher is better" (ex.: Staking
// Yield, green at 2%, red at 0%); otherwise "lower is better" (ex.: MVRV
// Z-Score, green below 0, red above 7). Values strictly between the two
// boundaries are Neutral — the spec gives two cutoffs per indicator but no
// rule for the gap between them.
pub fn classify(raw_value: f64, threshold: &Threshold) -> Result<Signal, AppError> {
    if threshold.green_boundary == threshold.red_boundary {
        return Err(AppError::EqualThresholds);
    }

    let higher_is_better = threshold.green_boundary > threshold.red_boundary;

    let signal = if higher_is_better {
        if raw_value >= threshold.green_boundary {
            Signal::Green
        } else if raw_value <= threshold.red_boundary {
            Signal::Red
        } else {
            Signal::Neutral
        }
    } else if raw_value <= threshold.green_boundary {
        Signal::Green
    } else if raw_value >= threshold.red_boundary {
        Signal::Red
    } else {
        Signal::Neutral
    };

    Ok(signal)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Lower is better (ex.: MVRV Z-Score — green_boundary 0, red_boundary 7).
    #[test]
    fn lower_is_better_green_below_green_boundary() {
        let threshold = Threshold {
            green_boundary: 0.0,
            red_boundary: 7.0,
        };
        assert_eq!(classify(-1.0, &threshold).unwrap().as_str(), "GREEN");
    }

    #[test]
    fn lower_is_better_red_above_red_boundary() {
        let threshold = Threshold {
            green_boundary: 0.0,
            red_boundary: 7.0,
        };
        assert_eq!(classify(8.0, &threshold).unwrap().as_str(), "RED");
    }

    #[test]
    fn lower_is_better_neutral_in_the_gap() {
        let threshold = Threshold {
            green_boundary: 0.0,
            red_boundary: 7.0,
        };
        assert_eq!(classify(3.0, &threshold).unwrap().as_str(), "NEUTRAL");
    }

    // Higher is better (ex.: Staking Yield — green_boundary 2, red_boundary 0).
    #[test]
    fn higher_is_better_green_above_green_boundary() {
        let threshold = Threshold {
            green_boundary: 2.0,
            red_boundary: 0.0,
        };
        assert_eq!(classify(3.0, &threshold).unwrap().as_str(), "GREEN");
    }

    #[test]
    fn higher_is_better_red_below_red_boundary() {
        let threshold = Threshold {
            green_boundary: 2.0,
            red_boundary: 0.0,
        };
        assert_eq!(classify(-0.5, &threshold).unwrap().as_str(), "RED");
    }

    #[test]
    fn higher_is_better_neutral_in_the_gap() {
        let threshold = Threshold {
            green_boundary: 2.0,
            red_boundary: 0.0,
        };
        assert_eq!(classify(1.0, &threshold).unwrap().as_str(), "NEUTRAL");
    }

    #[test]
    fn rejects_equal_boundaries() {
        let threshold = Threshold {
            green_boundary: 5.0,
            red_boundary: 5.0,
        };
        assert!(matches!(
            classify(5.0, &threshold),
            Err(AppError::EqualThresholds)
        ));
    }
}

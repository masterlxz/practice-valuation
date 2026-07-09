use crate::error::AppError;

pub struct GordonInputs {
    pub current_dividend: f64,
    pub expected_growth: f64,
    pub ke: f64,
}

pub enum Verdict {
    Barato,
    Caro,
}

impl Verdict {
    pub fn as_str(&self) -> &'static str {
        match self {
            Verdict::Barato => "BARATO",
            Verdict::Caro => "CARO",
        }
    }
}

pub struct ValuationOutcome {
    pub fair_price: f64,
    pub safety_margin: f64,
    pub verdict: Verdict,
}

// D1 = D0 × (1 + g)
// Preco_Justo = D1 / (Ke − g)
pub fn calculate(inputs: &GordonInputs, current_price: f64) -> Result<ValuationOutcome, AppError> {
    if inputs.ke <= inputs.expected_growth {
        return Err(AppError::InvalidGuard);
    }

    let d1 = inputs.current_dividend * (1.0 + inputs.expected_growth);
    let fair_price = d1 / (inputs.ke - inputs.expected_growth);
    let safety_margin = (fair_price - current_price) / fair_price;
    let verdict = if safety_margin > 0.0 {
        Verdict::Barato
    } else {
        Verdict::Caro
    };

    Ok(ValuationOutcome {
        fair_price,
        safety_margin,
        verdict,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn barato_when_fair_price_above_current_price() {
        let inputs = GordonInputs {
            current_dividend: 2.0,
            expected_growth: 0.03,
            ke: 0.10,
        };

        let outcome = calculate(&inputs, 25.0).unwrap();

        assert!((outcome.fair_price - 29.428571428571427).abs() < 1e-9);
        assert!(outcome.safety_margin > 0.0);
        assert_eq!(outcome.verdict.as_str(), "BARATO");
    }

    #[test]
    fn caro_when_fair_price_below_current_price() {
        let inputs = GordonInputs {
            current_dividend: 2.0,
            expected_growth: 0.03,
            ke: 0.10,
        };

        let outcome = calculate(&inputs, 40.0).unwrap();

        assert!(outcome.safety_margin < 0.0);
        assert_eq!(outcome.verdict.as_str(), "CARO");
    }

    #[test]
    fn rejects_ke_equal_to_growth() {
        let inputs = GordonInputs {
            current_dividend: 2.0,
            expected_growth: 0.05,
            ke: 0.05,
        };

        assert!(matches!(
            calculate(&inputs, 25.0),
            Err(AppError::InvalidGuard)
        ));
    }

    #[test]
    fn rejects_ke_below_growth() {
        let inputs = GordonInputs {
            current_dividend: 2.0,
            expected_growth: 0.08,
            ke: 0.05,
        };

        assert!(matches!(
            calculate(&inputs, 25.0),
            Err(AppError::InvalidGuard)
        ));
    }
}

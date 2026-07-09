use crate::error::AppError;

pub struct BanksInputs {
    pub book_value_per_share: f64,
    pub roe: f64,
    pub payout: f64,
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

// g_sustentável = ROE × (1 − Payout)
// P/B_Justo     = (ROE − g_sustentável) / (Ke − g_sustentável)
// Preco_Justo   = P/B_Justo × VPA
pub fn calculate(inputs: &BanksInputs, current_price: f64) -> Result<ValuationOutcome, AppError> {
    let sustainable_growth = inputs.roe * (1.0 - inputs.payout);

    if inputs.ke <= sustainable_growth {
        return Err(AppError::InvalidGuard);
    }

    let fair_pb = (inputs.roe - sustainable_growth) / (inputs.ke - sustainable_growth);
    let fair_price = fair_pb * inputs.book_value_per_share;

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
        let inputs = BanksInputs {
            book_value_per_share: 20.0,
            roe: 0.18,
            payout: 0.5,
            ke: 0.12,
        };

        let outcome = calculate(&inputs, 25.0).unwrap();

        assert!(outcome.fair_price > 0.0);
        assert!(outcome.safety_margin > 0.0);
        assert_eq!(outcome.verdict.as_str(), "BARATO");
    }

    #[test]
    fn caro_when_fair_price_below_current_price() {
        let inputs = BanksInputs {
            book_value_per_share: 20.0,
            roe: 0.18,
            payout: 0.5,
            ke: 0.12,
        };

        let outcome = calculate(&inputs, 90.0).unwrap();

        assert!(outcome.safety_margin < 0.0);
        assert_eq!(outcome.verdict.as_str(), "CARO");
    }

    #[test]
    fn rejects_ke_equal_to_sustainable_growth() {
        let inputs = BanksInputs {
            book_value_per_share: 20.0,
            // g_sustentável = 0.18 * (1 - 0.5) = 0.09 = ke
            roe: 0.18,
            payout: 0.5,
            ke: 0.09,
        };

        assert!(matches!(
            calculate(&inputs, 25.0),
            Err(AppError::InvalidGuard)
        ));
    }
}

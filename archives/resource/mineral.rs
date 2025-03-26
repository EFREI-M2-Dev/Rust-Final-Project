use crate::resource::traits::ResourceBehavior;

pub struct Mineral {
    name: String,
    amount: f64, // tonnes
    purity: f64, // pourcentage
}

impl ResourceBehavior for Mineral {
    fn name(&self) -> &str {
        &self.name
    }

    fn amount(&self) -> f64 {
        self.amount
    }

    fn extract(&mut self, amount: f64) -> f64 {
        let extracted = self.amount.min(amount);
        self.amount -= extracted;
        extracted
    }

    fn store(&mut self, amount: f64) -> bool {
        self.amount += amount;
        true
    }
}

impl Mineral {
    pub fn new(name: &str, amount: f64, purity: f64) -> Self {
        Self {
            name: name.to_string(),
            amount,
            purity,
        }
    }

    fn purity(&self) -> f64 {
        self.purity
    }

    fn refine(&mut self) -> bool {
        if self.purity < 90.0 {
            self.purity = (self.purity + 5.0).min(100.0);
            return true;
        }
        false
    }
}

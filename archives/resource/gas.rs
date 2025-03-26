use crate::resource::traits::ResourceBehavior;

pub struct Gas {
    name: String,
    amount: f64, // litres
}

impl ResourceBehavior for Gas {
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

impl Gas {
    pub fn new(name: &str, amount: f64) -> Self {
        Self {
            name: name.to_string(),
            amount,
        }
    }

    fn burn(&mut self, amount: f64) -> f64 {
        let burned = self.amount.min(amount);
        self.amount -= burned;
        burned
    }
}
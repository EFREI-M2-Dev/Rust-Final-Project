use crate::resource::traits::ResourceBehavior;

pub struct RadioactiveElement {
    name: String,
    amount: f64,  // kg
    half_life: f64, // années
}

impl ResourceBehavior for RadioactiveElement {
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

impl RadioactiveElement {
    pub fn new(name: &str, amount: f64, half_life: f64) -> Self {
        Self {
            name: name.to_string(),
            amount,
            half_life,
        }
    }

    fn half_life(&self) -> f64 {
        self.half_life
    }

    fn decay(&mut self, years: f64) {
        let decay_factor = (0.5_f64).powf(years / self.half_life);
        self.amount *= decay_factor;
    }
}

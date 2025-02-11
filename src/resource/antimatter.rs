use crate::resource::traits::ResourceBehavior;

pub struct Antimatter {
    name: String,
    amount: f64,  // grammes
    containment_stability: f64, // 0.0 - 1.0 (1.0 = stable)
}

impl ResourceBehavior for Antimatter {
    fn name(&self) -> &str {
        &self.name
    }

    fn amount(&self) -> f64 {
        self.amount
    }

    fn extract(&mut self, amount: f64) -> f64 {
        if self.containment_stability < 0.3 {
            println!("⚠️ L'antimatière s'est échappée !");
            self.amount = 0.0;
            return 0.0;
        }
        let extracted = self.amount.min(amount);
        self.amount -= extracted;
        extracted
    }

    fn store(&mut self, amount: f64) -> bool {
        if self.containment_stability > 0.8 {
            self.amount += amount;
            return true;
        } else {
            println!("⚠️ L'antimatière n'a pas pu être stockée !");
            return false;
        }
    }
}

impl Antimatter {
    pub fn new(name: &str, amount: f64, containment_stability: f64) -> Self {
        Self {
            name: name.to_string(),
            amount,
            containment_stability,
        }
    }

    fn containment_stability(&self) -> f64 {
        self.containment_stability
    }

    fn degrade(&mut self) {
        self.containment_stability -= 0.05;
        if self.containment_stability < 0.2 {
            println!("⚠️ Alerte critique : L'antimatière risque de s’échapper !");
        }
    }
}

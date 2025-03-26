trait CelestialBody {
    fn extract(&mut self, amount: u32) -> Option<Box<dyn Resource>>;
    fn depletion_rate(&self) -> f32;
}

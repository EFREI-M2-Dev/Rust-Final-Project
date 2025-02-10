trait Resource {
    fn name(&self) -> &str;
    fn rarity(&self) -> u8;  // 1 (commun) à 10 (ultra-rare)
    fn extraction_difficulty(&self) -> u8;
}

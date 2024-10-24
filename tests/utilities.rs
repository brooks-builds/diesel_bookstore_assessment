pub fn random_name(name: &str) -> String {
    format!("{name} - {}", rand::random::<u16>())
}

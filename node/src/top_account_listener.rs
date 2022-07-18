use data::basics;

#[derive(Default, Debug)]
pub struct TopAccountListener {
    pub round: basics::Round,
    pub online_circulation: basics::MicroAlgos,
    pub total_circulation: basics::MicroAlgos,
    pub accounts: Vec<basics::AccountDetails>,
}
impl TopAccountListener {
    pub fn new() -> Self {
        Default::default()
    }
}

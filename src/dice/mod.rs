use rand::Rng;

pub trait Die {
    fn probabilities(&self) -> &[f64];
    fn roll(&self, rng: &mut Rng) -> u32;
}

pub mod linear_space_die;
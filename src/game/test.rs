use probability::Distribution;
use probability::distributions::*;
use rand::{thread_rng, Rng, ThreadRng};

pub fn test(total: f64, target: f64) -> f64 {
    let ref mut generator = thread_rng();
    let hit_beta = Beta::new(total, target, -total, total);
    let mut hits = hit_beta.sample::<ThreadRng>(generator);
    if hits < 0.0 {
        hits = 0.0;
    }
    return hits;
}

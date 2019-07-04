use mancala_rust::*;
use rand::SeedableRng;
use rand_pcg::Mcg128Xsl64;

fn main() {
    let mut leaner = Learner::new(Mcg128Xsl64::from_entropy(), true, 4);
    leaner.learn(10);
}

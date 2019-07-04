use std::fs::File;
use std::io;

use rand::SeedableRng;
use rand_pcg::Mcg128Xsl64;

use mancala_rust::*;

fn main() -> io::Result<()> {
    let stealing = true;
    let name = format!("p{}s{}_{}.db", PIT, SEED, stealing);

    let mut leaner = Learner::new(Mcg128Xsl64::from_entropy(), stealing, 4);
    match File::open(&name) {
        Err(_) => {
            eprintln!("no db file: {}", name);
        }
        Ok(f) => {
            eprintln!("restore db file: {}", name);
            let mut f = io::BufReader::new(f);
            leaner.restore(&mut f)?;
        }
    }
    leaner.learn(1000000);
    let mut f = io::BufWriter::new(File::create(&name)?);
    leaner.dump(&mut f)?;
    Ok(())
}

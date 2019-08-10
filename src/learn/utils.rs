use std::io::{Read, Write};

use fnv::FnvHashMap;

use crate::board::{PIT, SEED};
use crate::from_compact_key;

pub fn db_name(stealing: bool) -> String {
    format!("p{}s{}_{}.dat", PIT, SEED, stealing)
}

pub fn load(stealing: bool) -> FnvHashMap<u64, (i8, u8)> {
    let name = db_name(stealing);
    let mut f = match std::fs::File::open(&name) {
        Err(e) => {
            eprintln!("{} is not exists ({})", name, e);
            return FnvHashMap::with_capacity_and_hasher(1024, Default::default());
        }
        Ok(f) => std::io::BufReader::new(f),
    };
    let n = {
        let mut buf = [0; 8];
        match f.read_exact(&mut buf) {
            Err(e) => {
                eprintln!("read size failed: {}", e);
                return FnvHashMap::with_capacity_and_hasher(1024, Default::default());
            }
            Ok(()) => u64::from_le_bytes(buf) as usize,
        }
    };

    let mut data = FnvHashMap::with_capacity_and_hasher(n.next_power_of_two(), Default::default());
    for i in 0..n {
        let mut buf = [0; 8];
        let key = match f.read_exact(&mut buf) {
            Err(e) => {
                eprintln!("read {}th key failed: {}", i, e);
                return data;
            }
            Ok(()) => u64::from_le_bytes(buf),
        };
        let mut buf = [0; 2];
        let value = match f.read_exact(&mut buf) {
            Err(e) => {
                eprintln!("read {}th value failed: {}", i, e);
                return data;
            }
            Ok(()) => (buf[0] as i8, buf[1]),
        };
        data.insert(key, value);
    }
    data
}

pub fn just_load(stealing: bool) -> FnvHashMap<u64, i8> {
    let name = db_name(stealing);
    let mut f = std::io::BufReader::new(std::fs::File::open(&name).unwrap());
    let n = {
        let mut buf = [0; 8];
        f.read_exact(&mut buf).unwrap();
        u64::from_le_bytes(buf) as usize
    };

    let mut data = FnvHashMap::with_capacity_and_hasher(n, Default::default());
    for _ in 0..n {
        let mut buf = [0; 8];
        f.read_exact(&mut buf).unwrap();
        let key = u64::from_le_bytes(buf);

        let mut buf = [0; 2];
        f.read_exact(&mut buf).unwrap();
        let value = buf[0] as i8;
        data.insert(key, value);
    }
    data
}

pub fn save(stealing: bool, data: &FnvHashMap<u64, (i8, u8)>) -> std::io::Result<()> {
    let name = db_name(stealing);
    let mut f = std::io::BufWriter::new(std::fs::File::create(&name)?);
    f.write_all(&mut data.len().to_le_bytes())?;
    for (key, value) in data.iter() {
        f.write_all(&mut key.to_le_bytes())?;
        f.write_all(&mut [value.0 as u8, value.1])?;
    }
    Ok(())
}

pub fn iter_load(stealing: bool) -> std::io::Result<Load> {
    let name = db_name(stealing);
    let mut f = std::io::BufReader::new(std::fs::File::open(&name)?);
    {
        let mut buf = [0; 8];
        f.read_exact(&mut buf)?
    }
    Ok(Load { f })
}

pub struct Load {
    f: std::io::BufReader<std::fs::File>,
}

impl Iterator for Load {
    type Item = ([u8; 12], i8, u8);

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = [0; 8];
        let key = match self.f.read_exact(&mut buf) {
            Err(_) => {
                return None;
            }
            Ok(()) => u64::from_le_bytes(buf),
        };
        let mut buf = [0; 2];
        let value = match self.f.read_exact(&mut buf) {
            Err(_) => {
                return None;
            }
            Ok(()) => (buf[0] as i8, buf[1]),
        };
        Some((from_compact_key(key), value.0, value.1))
    }
}

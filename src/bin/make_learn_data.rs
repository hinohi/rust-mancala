use fnv::FnvHashMap;
use rand::SeedableRng;
use rand_pcg::Mcg128Xsl64;

use mancala_rust::*;

fn raw_scores(board: &Board) -> i8 {
    let (s0, s1) = board.scores();
    if board.side() == Side::First {
        s0 as i8 - s1 as i8
    } else {
        s1 as i8 - s0 as i8
    }
}

fn seed_scores(board: &Board) -> i8 {
    let l0 = board.self_seeds().iter().sum::<u8>() as i8;
    let l1 = board.opposite_seed().iter().sum::<u8>() as i8;
    l0 - l1
}

fn search(data: &mut FnvHashMap<u64, (i8, u8)>, board: Board, depth: u8) -> Option<(i8, u8)> {
    let key = compact_key(&board);
    if let Some((l, d)) = data.get(&key) {
        return Some((raw_scores(&board) + *l, *d));
    }
    if depth == 0 {
        return None;
    }
    if board.is_finished() {
        let s = raw_scores(&board);
        let l = seed_scores(&board);
        data.insert(key, (l, 0));
        return Some((s + l, 0));
    }
    let mut best_score = -128;
    let mut best_depth = 0;
    for next in board.list_next() {
        let a = search(data, next, depth - 1);
        match a {
            None => return None,
            Some((s, d)) => {
                let s = -s;
                if s > best_score {
                    best_score = s;
                    best_depth = d + 1;
                }
            }
        }
    }
    data.insert(key, (best_score - raw_scores(&board), best_depth));
    Some((best_score, best_depth))
}

fn list_path() -> Vec<Board> {
    let mut board = Board::new(STEALING);
    let mut ret = vec![board.clone()];
    let mut ai = MCTree::new(256, Mcg128Xsl64::from_entropy());
    while !board.is_finished() {
        let pos_list = ai.sow(&board);
        for pos in pos_list {
            board.sow(pos);
        }
        ret.push(board.clone());
    }
    ret
}

const STEALING: bool = true;

fn load() -> FnvHashMap<u64, (i8, u8)> {
    use std::io::Read;

    let name = format!("p{}s{}_{}.dat", PIT, SEED, STEALING);
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

fn save(data: &FnvHashMap<u64, (i8, u8)>) -> std::io::Result<()> {
    use std::io::Write;

    let name = format!("p{}s{}_{}.dat", PIT, SEED, STEALING);
    let mut f = std::io::BufWriter::new(std::fs::File::create(&name)?);
    f.write_all(&mut data.len().to_le_bytes())?;
    for (key, value) in data.iter().take(1 << 27) {
        f.write_all(&mut key.to_le_bytes())?;
        f.write_all(&mut [value.0 as u8, value.1])?;
    }
    Ok(())
}

fn main() {
    let mut data = load();
    for i in 1..=50_000 {
        let mut path = list_path();
        while let Some(board) = path.pop() {
            if search(&mut data, board, 20).is_none() {
                break;
            }
        }
        if i % 100 == 0 {
            println!("{} {}", i, data.len());
        }
    }

    println!("save: {:?}", save(&data));

    println!("depth histogram");
    let mut hist = [0; 256];
    for (key, (score, depth)) in data.iter() {
        hist[*depth as usize] += 1;
        if *depth > 33 {
            println!("{:?} {}", from_compact_key(*key), *score);
        }
    }
    for (depth, count) in hist.iter().enumerate() {
        println!("{} {}", depth, count);
        if *count == 0 {
            break;
        }
    }
}

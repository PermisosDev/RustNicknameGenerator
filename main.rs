use rand::Rng;
use std::{
    fs::File,
    io::prelude::*,
    io::{self, BufRead, BufReader, Write},
    path::Path,
    time::Instant,
};

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn main() -> std::io::Result<()> {
    let start = Instant::now();

    let mut rng = rand::thread_rng();

    let prefixeslines = lines_from_file("Prefixes.txt").expect("Could not load lines");

    let sufixlines = lines_from_file("Sufixes.txt").expect("Could not load lines");

    let mut f = File::create("result.txt").expect("Unable to create file");

    for x in 0..1000000 {
        let mut randomprefix = &prefixeslines[rng.gen_range(0, prefixeslines.iter().count())];
        let mut randomsufix = &sufixlines[rng.gen_range(0, sufixlines.iter().count())];
        let mut kekiante = randomprefix.to_string();
        let mut kekiante2 = randomsufix.to_string();
        let mut result2 = kekiante + &kekiante2 + "\n";
        f.write(result2.as_bytes())
            .expect("Unable to write data");
    }
    let elapsed = start.elapsed();
    println!("Millis: {} ms", elapsed.as_millis());
    Ok(())
}

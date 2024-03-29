use rand::Rng;
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Write},
    path::Path,
    time::Instant,
    thread,
};
pub fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn main() -> std::io::Result<()> {
    

    let start = Instant::now();
    let mut f = File::create("result.txt").expect("Unable to create file");
    
    let handle = thread::spawn(move || {
        for _x in 0..1000{   
   
        let mut rng = rand::thread_rng();

        let prefixeslines = lines_from_file("Prefixes.txt").expect("Could not load lines");

        let sufixlines = lines_from_file("Sufixes.txt").expect("Could not load lines");
        let randomprefix = &prefixeslines[rng.gen_range(0, prefixeslines.iter().count())];
        let randomsufix = &sufixlines[rng.gen_range(0, sufixlines.iter().count())];
        let stringprefix = randomprefix.to_string();
        let stringsufix = randomsufix.to_string();
        let result2 = stringprefix + &stringsufix + "\n";
        f.write_all(result2.as_bytes()).expect("Unable to write data");
        }
    });
    
    handle.join().unwrap();
 
    let elapsed = start.elapsed();
    
    println!("Millis: {} ms", elapsed.as_millis());
    Ok(())
}

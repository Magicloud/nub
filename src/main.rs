use std::collections::HashSet;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let mut ls: Vec<_> = buffer.lines().collect();
    let mut uniques = HashSet::new();
    ls.retain(|l| uniques.insert(l.clone()));
    for l in ls {
        println!("{}", l)
    }
    Ok(())
}

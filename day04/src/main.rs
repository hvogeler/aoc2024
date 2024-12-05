use std::path::Path;

use common::{read_test_data, Error};

fn main() -> Result<(), Error> {
    let data = read_test_data(Path::new("./day04/example.dat"))?;
    println!("Example: {}", data);
    Ok(())
}

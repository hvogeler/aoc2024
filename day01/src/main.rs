use std::path::Path;

use common::{read_test_data, Error};

fn main() -> Result<(), Error> {
    let example_data = read_test_data(Path::new("./day01/example.dat"))?;
    println!("Example data: \n{}", example_data);
    Ok(())
}

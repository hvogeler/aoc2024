use std::path::Path;

use common::{read_test_data, Error};

fn main() -> Result<(), Error> {
    let data = read_test_data(Path::new("./day07/example.dat")).unwrap();
    println!("Example Data: \n{}", data);
    Ok(())
}

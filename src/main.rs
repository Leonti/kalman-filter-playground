extern crate csv;
#[macro_use]
extern crate serde_derive;
use csv::Error;

#[derive(Debug, Deserialize)]
struct Record {
    time: usize,
    delta: usize,
    x: f32,
    y: f32,
    z: f32,
    gyro_x: f32,
    gyro_y: f32,
    gyro_z: f32,
}

fn main() -> Result<(), Error> {

    let mut reader = csv::Reader::from_path("axl_gyro.csv")?;

    for record in reader.deserialize::<Record>() {
        let record: Record = record?;
        println!("{:?}", record);
    }

    Ok(())
}

extern crate csv;
#[macro_use]
extern crate serde_derive;
use csv::Error;
// Accel XYZ(m/s^2)  |   Gyro XYZ (rad/s)
// Current g-force, degrees per second
// https://learn.sparkfun.com/tutorials/gyroscope/all
// https://learn.sparkfun.com/tutorials/accelerometer-basics
// https://www.digikey.com.au/en/articles/techzone/2018/jan/apply-sensor-fusion-to-accelerometers-and-gyroscopes
extern crate dcmimu;
use dcmimu::DCMIMU;

#[derive(Debug, Deserialize)]
struct Record {
    time: usize,
    delta: f32,
    x: f32,
    y: f32,
    z: f32,
    gyro_x: f32,
    gyro_y: f32,
    gyro_z: f32,
}

fn main() -> Result<(), Error> {

    let mut dcmimu = DCMIMU::new();
    let mut reader = csv::Reader::from_path("axl_gyro.csv")?;

    for record in reader.deserialize::<Record>() {
        let record: Record = record?;
  //      println!("{:?}", record);

         let (dcm, _gyro_biases) = dcmimu.update((record.gyro_x, record.gyro_y, record.gyro_z),
                            (record.x, record.y, record.z),
                            record.delta / 1000.0);
        println!("Roll: {}; yaw: {}; pitch: {}", dcm.roll, dcm.yaw, dcm.pitch);
        println!("{:?} == {}, {}, {}", dcmimu.all(), dcmimu.roll(), dcmimu.yaw(), dcmimu.pitch());
    }

    Ok(())
}

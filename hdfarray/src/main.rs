use hdf5::{File, Result};
use ndarray::{Array, Array1, Array2};
use ndarray_rand::rand_distr::Uniform;
use ndarray_rand::RandomExt;

fn write_hdf5() -> Result<()> {
    let data: Array2<u32> = Array::random((25, 5), Uniform::new(0, 255));
    let axis: Array1<f32> = Array::random(25, Uniform::<f32>::new(0.0, 255.0));
    let file = File::create("pixels.h5")?; // open for writing
    let group = file.create_group("sensor")?; // create a group
    let _ds = group
        .new_dataset_builder()
        .with_data(&data)
        // finalize and write the dataset
        .create("measurement")?;
    let _ds2 = group
        .new_dataset_builder()
        .with_data(&axis)
        // finalize and write the dataset
        .create("axis")?;
    Ok(())
}

fn read_and_modify_hdf5() -> Result<()> {
    let file = File::open_rw("pixels.h5")?; // open for reading
    let ds = file.dataset("sensor/measurement")?; // open the dataset
    let res: Array2<i32> = ds.read()?;
    println!("{}", res);
    let res2 = 2 * res;
    let _ds2 = file
        .group("sensor")?
        .new_dataset_builder()
        .with_data(&res2)
        .create("mod_measurement")?;
    Ok(())
}

fn main() -> Result<()> {
    write_hdf5()?;
    read_and_modify_hdf5()?;
    Ok(())
}

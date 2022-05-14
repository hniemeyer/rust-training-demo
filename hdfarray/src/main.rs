use hdf5::{File, Result};
use ndarray::{arr2, s, Array, Array1, Array2};
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

fn read_hdf5() -> Result<()> {
    let file = File::open("pixels.h5")?; // open for reading
    let ds = file.dataset("sensor/measurement")?; // open the dataset
    let res: Array2<i32> = ds.read_slice(s![1.., ..])?;
    println!("{}", res);
    Ok(())
}

fn main() -> Result<()> {
    write_hdf5()?;
    read_hdf5()?;
    Ok(())
}

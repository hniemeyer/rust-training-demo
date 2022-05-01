use hdf5::{File, Result};
use ndarray::{arr2, s, Array2, Array};
use ndarray_rand::RandomExt;
use ndarray_rand::rand_distr::Uniform;

fn write_hdf5() -> Result<()> {
    let data: Array2<i32> = Array::random((25, 5), Uniform::new(0, 10));
    let file = File::create("pixels.h5")?; // open for writing
    let group = file.create_group("dir")?; // create a group
    let builder = group.new_dataset_builder();
    let _ds = builder
        .with_data(&data)
        // finalize and write the dataset
        .create("pixels")?;
    Ok(())
}

fn read_hdf5() -> Result<()> {
    let file = File::open("pixels.h5")?; // open for reading
    let ds = file.dataset("dir/pixels")?; // open the dataset
    let res: Array2<i32> = ds.read_slice(s![1.., ..])?;
    println!("{}", res);
    Ok(())
}

fn main() -> Result<()> {
    write_hdf5()?;
    read_hdf5()?;
    Ok(())
}

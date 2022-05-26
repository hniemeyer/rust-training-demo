use ndarray::prelude::*;

fn main() {
    let my_array = array![[1., 2., 3.], [4., 5., 6.],];

    println!("dimensions: {}, {}", my_array.dim().0, my_array.dim().1);
    println!("number of elements: {}", my_array.len());

    println!("{:?}", my_array);

    let a = Array::<f64, _>::linspace(0., 5., 6)
        .into_shape((2, 3))
        .expect("Reshaping failed.");
    println!("{:?}", a);

    // @ is an arithmectic operation
    // &A @ &A produces a new Array
    // B @ A consumes B, updates it with the result, and returns it
    // B @ &A consumes B, updates it with the result, and returns it
    // C @= &A performs an arithmetic operation in place
    println!("{:?}", &a + &my_array);

    println!("{}", a.sum_axis(Axis(0)));

    // https://github.com/rust-ndarray/ndarray/blob/master/README-quick-start.md
    // https://docs.rs/ndarray/latest/ndarray/doc/ndarray_for_numpy_users/index.html
}

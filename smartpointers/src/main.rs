use std::rc::Rc;

// struct MyDefectList {
//     value: i32,
//     next_item: Option<MyDefectList>,
// }

struct MyList {
    value: i32,
    next_item: Option<Box<MyList>>,
}

//"Destructors" in Rust
struct MyData {
    value: i32,
}

impl Drop for MyData {
    fn drop(&mut self) {
        println!("MyData with value =  {} is destroyed", self.value);
    }
}

fn main() {
    // in Rust there are similar smart pointers as in C++
    // Box<T> for allocating values on the heap
    //Rc<T>, a reference counting type that enables multiple ownership

    let mut point_to_val = Box::new(32);
    println!("{}", point_to_val);
    *point_to_val = 12;
    println!("{}", point_to_val);

    //MyList needs a smart pointer inside the option because its size is not known at compile time
    let _my_list = MyList {
        value: 32,
        next_item: Some(Box::new(MyList {
            value: 45,
            next_item: None,
        })),
    };

    println!(
        "{}, {}",
        _my_list.value,
        _my_list.next_item.expect("should not be None").value
    );

    let mut my_vec = vec![
        MyData { value: 1 },
        MyData { value: 2 },
        MyData { value: 3 },
    ];

    my_vec.remove(1);
    my_vec.push(MyData { value: 4 });

    let rc_my_data = Rc::new(MyData { value: 26 });
    println! {"ref count: {}", Rc::strong_count(&rc_my_data)};
    {
        let _x = Rc::clone(&rc_my_data);
        let _y = Rc::clone(&rc_my_data);
        println! {"ref count: {}", Rc::strong_count(&rc_my_data)};
    }
}

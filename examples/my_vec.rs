fn main() {
    //let v: Vec<i32> = my_vec![1;4];
    let v = my_vec![1, 2, 3,];
    println!("{:?}", v);
}

// my_vec! = my_vec! { 1, 2, 3 }; // Vec<i32>
#[macro_export]
macro_rules! my_vec {
    () => { Vec::new() };
    ($elem:expr; $n:expr) => { std::vec::from_elem($elem, $n) };
    ($($x:expr),+ $(,)?) => {
        {
            // let mut temp_vec = Vec::new();
            // $(
            //     temp_vec.push($x);
            // )*
            // temp_vec
            <[_]>::into_vec(Box::new([$($x),*]))
        }
    };
}
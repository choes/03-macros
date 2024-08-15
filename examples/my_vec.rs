use macros::my_vec;

fn main() {
    //let v: Vec<i32> = my_vec![1;4];
    let v = my_vec![1, 2, 3,];
    println!("{:?}", v);
}
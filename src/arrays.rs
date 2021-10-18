use std::mem;
pub fn run(){
    let mut numbers:[i32;5]=[1,2,3,4,5];
    //re-assign value
    numbers[2]=20;
    println!("{:?}",numbers);
    //get single elemet
    println!("{}",numbers[0]);
    //array len
    println!("array length:{}", numbers.len());
    //arrays are stack alolocated
    println!("Array occupies {} bytes",mem::size_of_val(&numbers));
    //get slices
    let slice: &[i32] =&numbers[0..2];
    println!("Slices: {:?}",slice);
}
use std::mem;
pub fn run(){
    let mut numbers:Vec<i32>=vec![1,2,3,4,5];
    //re-assign value
    numbers[2]=20;
    numbers.push(350);
    numbers.push(123);

    println!("{:?}",numbers);
    numbers.pop();
    println!("{:?}",numbers);
    //get single elemet
    println!("{}",numbers[0]);
    //array len
    println!("array length:{}", numbers.len());
    //vectoer are stack alolocated
    println!("vector occupies {} bytes",mem::size_of_val(&numbers));
    //get slices
    let slice: &[i32] =&numbers[0..2];
    println!("Slices: {:?}",slice);
    //looping in vector values
    for x in numbers.iter(){
        println!("{}",x);
    }
    //mute with loop 
    for  x in numbers.iter_mut() {
        *x *=2;
        
    }
    println!("{:?}",numbers);
}
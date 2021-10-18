pub fn run(){
    greeting("Hello","Ravi");
    let d= add(256, 7896);
    println!("{}", d);
    //closure
    let add_nums= |n1:i32,n2:i32| n1+n2;
    println!("C:{}",add_nums(459,246) );
}
fn greeting(greet :&str , name : &str){
    println!("{} {}, nice to meet you!", greet, name);
}
fn add(n1:i32, n2: i32) -> i32{
    n1+n2
}
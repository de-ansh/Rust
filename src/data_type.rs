/*
primitive types--
Integers: u8.i8,U16.i16,u32,i32, u64,i64,u128,i128
Floats:f32,f64
Boolean(bool)
Charecters(char)
Tuples
Arrays
*/
pub fn run(){
    let _x = 3;//i32
    let _y=2.4;//f64
    //addexplicit
    let _z: i64 = 4569757587575;
    //find max
    println!("MAx i32:{}",std::i32::MAX);
    println!("MAx i32:{}",std::i64::MAX);
    //Boolean
    let _is_active:bool = true;
    //get  boolean from  expression
    let _is_greater:bool =10>5;
    let _a1='a';
    let face ='\u{1F600}';
    println!("{:?}",(_x,_y,_z,_is_active,_is_greater,_a1,face));
    
}
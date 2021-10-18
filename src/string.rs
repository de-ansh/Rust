pub fn run(){
    let mut hello = String ::from("Hello");
    //get length
    println!("Length:{}", hello.len());
    hello.push('r');
    hello.push_str("ore");
    //capacitty in bits
    println!("capacity:{}",hello.capacity());
    println!("IS Empty:{}",hello.is_empty());
    println!("Contains 'rore'{}",hello.contains("rore"));
    println!("Replace: {}", hello.replace("rore","rare" ));
    //Looping through string by whitespace 
    for word in hello.split_whitespace(){
        println!("{}",word);
    }
    //string with capacity creation
    let mut s= String::with_capacity(10);
    s.push('a');
    s.push('b');
    s.push('c');
    s.push('d');
    s.push('e');
    println!("{}",s);

    //assertion testing
   // assert_eq!(10,s.len());
   // assert_eq!(20,s.capacity());
    println!("{}", hello);
}
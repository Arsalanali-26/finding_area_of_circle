use std::io;
pub fn area_calcultion_func(){
    let mut radius = String::new();
    println!("Please Enter Radius of Circle");
    io::stdin().read_line(&mut radius).expect(&radius);
    let  radius:f64 = radius.trim().parse().unwrap();
    let square = radius.powf(2.0);    
    let pi = 3.14;
    println!("Your area of Circle is {}",pi*square );

}




fn main(){
    
    /*
    let _sum = 5 + 10;

    // subtraction
    let _difference = 95.5 -4.3;

    // multiplication
    let _product = 4 *30;

    // dvision
    let _quotient = 56.7 / 32.2;
    let _truncated = -5 /3;

    //remainder
    let _remainder = 43 % 5;
     */
    
    //Learning Tuples
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of x, y, z is : {}, {}, {}",x, y,z);

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let _five_hundred = x.0;

    let _six_point_four = x.1;

    let one = x.2;

    println!("the value of one is: {one}");
} 
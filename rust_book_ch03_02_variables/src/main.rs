fn main() {
    /*
      mutable vars...
    */
    // let x = 5; // this will not compile, use "mut" declaration on next line
    let mut x = 5;
    println!("the value of x is {}", x);

    x = 6;
    println!("the value of x is {}", x);

    /*
    constants...
     */
    const PI_SHORT: f32 = 3.14;
    println!("the value of PI_SHORT is {}", PI_SHORT);
    const A_MILL: u32 = 1_000_000;
    println!("the value of A_MILL is {}", A_MILL);

    /*
    shadowing...
     */
    let y = 1;
    println!("the value of y is {}", y);

    let y = y * 2;
    println!("the value of shadowed y is {}", y);

    // changing type using shadowing
    let spaces = "   ";
    let spaces = spaces.len();
    println!("there are {} spaces", spaces);

    // can't do this with a mut var
    // let mut spaces = "   ";
    // spaces = spaces.len();

    // will warn on this, but can do it (variable does not need to be mutable)
    // let mut spaces = "   ";
    // let spaces = spaces.len();

    /*
    integer...
     */
    let _x = 1; // i32 (default, faster than 64)
    let _y: i64 = 2; // i64
    println!("x: {}", x);

    /*
    floating point...
     */
    let _x = 2.0; // f64 (default, double precision, as fast as f32)
    let _y: f32 = 3.0; // f32

    /*
    chars, single quote...
     */
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("x: {}, z: {}, heart_eyed_cat: {}", c, z, heart_eyed_cat);

    /*
    tuple, fixed length, multi-types...
     */
    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);
    let (a, _, _) = tup; // destructuring with ignore convention
    println!("a: {}", a);
    println!("tup second element: {}", tup.1);

    /*
    array, fixed length, single-type...
     */
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    println!("first month: {}", months[0]);
    println!("last month: {}", months[months.len() - 1]);

    let a: [u8; 5] = [1, 2, 3, 4, 5];
    println!("first of a: {}", a[0]);

    let zeros = [0; 5];
    println!("first of zeros: {}", zeros[0]);

    let ones: [u8; 3] = [1;3];
    println!("ones: {:?}", ones);

}

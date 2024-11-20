fn main() {
    /*
    let x: i32 = 1+2+3;
    println!("x = {x}");

    let y: i32 = 1+2*3;
    println!("y = {y}"); */

    for i in 1..=25 {
        if i % 3 == 0 && i % 5 == 0 {
            print!("FizzBozz!\n")
        } else if i % 3 == 0 {
            print!("fizz\n");
        } else if i % 5 == 0 {
            print!("bozz\n")
        } else {
            print!("{}\n", i)
        }
    }
}
//no

fn main() {

    let mut number = "500";

    println!("Number starts at: {}", number);

    let numref = &mut number;

    {
        let number = 100;
        println!("Number is now: {}", number);
        println!("Referenced number is {}:", numref);
        *numref = "300";
    }

    println!("Number is finally: {}", number);
}

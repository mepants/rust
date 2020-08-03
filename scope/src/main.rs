fn main(){
    let mut a =String::from("hello");
 
    let b = {
        let r = &mut a;
        world(r)
    };

    println!("{}", a);
    println!("{}", b);
}

//fn world(_s: &mut String) -> &'static str {
//    "world"
//}

//fn world(_s: &mut String) -> String {
//    String::from("world")
//}

fn world(_s: &mut String) -> &str {
    "world"
}

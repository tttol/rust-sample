fn main() {
    let s1 = String::from("hello deep copy");
    let s2 = s1.clone();

    println!("s1={}", s1);
    println!("s2={}", s2);
}

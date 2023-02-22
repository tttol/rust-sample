fn main() {
    let s1 = String::from("hello shallow copy");
    let s2 = s1;

    println!("s1={}", s1); // s1の所有権がs2に移動したあとにs1を参照するとエラー
}

fn main() {
    let s1 = String::from("bad_copy sample");
    let s2 = s1;

    // println!("{}, world!", s1); // s1の所有権がs2に移動したあとにs1を参照するとエラー
    println!("{}", s2);  // s2を使うことはOK
}

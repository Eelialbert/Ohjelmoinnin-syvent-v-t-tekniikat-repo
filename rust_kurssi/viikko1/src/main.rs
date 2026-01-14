fn main () {
    let kierrokset = 100;

    let mut t1: i32 = 0;
    let mut t2: i32 = 1;
    for _int in 0..kierrokset {
        let t3 = t1 + t2;
        t1 = t2;
        t2 = t3;
        println!("{}", t1);
    }
}

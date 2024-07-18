fn main() {
    //call here 
}

fn asterix_segitiga (n:i32) {

    for i in 1..=n {
        let mut a : String = String::from("");
        let mut to_j = n+n-1;
        for j in 1..=to_j {
            if j > n - i && j < n + i {
                a.push_str("*")
            } else {
                a.push_str(" ")
            }
        }
        println!("{}", a);
    }

}

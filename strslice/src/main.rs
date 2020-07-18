fn main() {
    let s = String::from("WELCOME TO RUST");

    let slice1 = &s[0..7]; //slice of string startig from 0 to 6th index
    let slice2 = &s[8..10];
    let slice3 = &s[11..15];

    println!("string:{}", s);
    println!("slice1:{}", slice1);
    println!("slice2:{}", slice2);
    println!("slice3:{}", slice3);

    let len = s.len();
    for i in 0..len {
        println!("{}", substring(&s, 0, i+1));
    }
}

//Function substring using the concept of slice
fn substring(s: &str, start:usize, end:usize) -> &str {
    let slice = &s[start..end];
    slice
}

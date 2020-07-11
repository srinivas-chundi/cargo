fn main() {
    let mut i = 0;
    //While loop; mostly useful with some boolean conditions
    while i < 10 {
        i += 1;
        println!("SumOf:{} = {}", i, csum(i));
    }

    //Most used one for iterators
    for i in 11..21 {
        println!("SumOf:{} = {}", i, csum(i));
    }

    //Iterate over an array
    let list = [30, 40, 50, 60, 70, 80, 90, 100];
    for e in list.iter() {
        println!("SumOf:{} = {}", e, csum(*e));
    }

    let mut str = String::from("WELCOME");
    let pstr1 = &mut str;
    pstr1.push_str(" TO PUBMATIC");
    println!("In main String:{}", pstr1);

    let str2 = String::from("GOOD");
    display(&str2);
    println!("In main String:{}", str2);
}

fn csum(n: i32) -> i32 {
    let mut i = 0;
    let mut s = 0;

    //Uncondition loop
    loop {
        i += 1;
        s += i;

        if i >= n {
            break s;
        }
    }
}

fn display(s: &String) {
    println!("In display String:{}", s);
}

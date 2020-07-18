fn main() {
    loops_demo();
    array_demo();
    reference_demo();
    change_by_reference_demo();

    multi_reference_demo();
    ownership_demo();
}

fn loops_demo() {
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

fn array_demo() {
    let list: [i32; 5] = [1,2,3,4,5];
    //let list = [1,2,3,4];
    display_list(list);
}


fn display_list(l: [i32; 5]) {
    for i in l.iter() {
        println!("{}", i);
    }
} 

fn reference_demo() {
    let s1 = String::from("WELCOME");
    let len = str_len(&s1);
    println!("The length of '{}' is {} ", s1, len);
}

fn str_len(s: &String) -> usize {
    s.len()
}


fn change_by_reference_demo() {
    let mut msg = String::from("WELCOME");
    let cstr = String::from(" TO ");
    str_cat(&mut msg, &cstr);
    str_cat(&mut msg, &String::from("RUST"));
    println!("In main String:{}", msg);
}

fn str_cat(s: &mut String, cs: &String) {
    s.push_str(cs);
}

fn multi_reference_demo() {
    let mut msg = String::from("WELCOME");

    let s1 = &msg;
    let s2 = &msg;
    println!("{} {}", s1, s2);

    println!("{} {}", s1, s2);
    let s3 = &mut msg;
    s3.push_str(" TO RUST LAND");
    println!("{}", s3);
}

fn ownership_demo() {
    let msg = get_the_message(); // Once we get the message we are the owner

    println!("New message:{}", msg);
}

fn get_the_message() -> String {
    let newmsg = String::from("A NEW WELCOME");

    println!("New message created and will move the ownership to caller..; msg:{}", newmsg);
    //return the newmsg
    newmsg
}

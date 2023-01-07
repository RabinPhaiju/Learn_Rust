#![allow(unused)]

use core::num;
use std::io;
use rand::Rng;
use std::io::{Write,BufReader,BufRead,ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use std::ops::Add; // allows to add generics
use std::collections::HashMap;

fn input_output(){
    println!("What is your name?");
    let mut name = String::new();
    let greeting: &str = "Nice to meet you";
    io::stdin().read_line(&mut name).expect("Didn't Receive Input");

    println!("hello {} {}",name.trim_end(),greeting)
}

fn data_types(){
    const ONE_MIL: u32 = 1_000_000;
    const PI:f32 = 3.1415;
    let age: &str = "67";
    let mut age:u32 = age.trim().parse() // shadowing -> same name with diff data type.
        .expect("Age wasn't assigned a number");
    age +=1;
    println!("I am {} and I want {}",age,ONE_MIL)
}

fn data_types_max(){
    println!("Max i32 : {}",i32::MAX);
    println!("Max i64 : {}",i64::MAX);
    println!("Max u32 : {}",u32::MAX);
    println!("Max u64 : {}",u64::MAX);
    println!("Max u128 : {}",u128::MAX);
    println!("Max usize : {}",usize::MAX);
    println!("Max f32 : {}",f32::MAX);
    println!("Max f64 : {}",f64::MAX);
}

fn boolean(){
    let is_true: bool = true;
    let my_grade:&str = "A";
    print!("is_true: {} and my_grade: {}",is_true,my_grade)
}

fn numbers(){
    let num_1:f32 = 1.11111111111;
    println!("f32: {}",num_1+1.11111111111);
    let num_2:f64 = 1.11111111111;
    println!("f64: {}",num_2+1.11111111111);

    let num_3:u32 = 5;
    let num_4:u32 = 4;
    println!("5+4 = {}",num_3+num_4);
    println!("5-4 = {}",num_3-num_4);
    println!("5*4 = {}",num_3*num_4);
    println!("5/4 = {}",num_3/num_4);
    println!("5%4 = {}",num_3%num_4);
    
    let random_num = rand::thread_rng().gen_range(1..101);
    println!("random_num {}",random_num);
}

fn if_condition(){
    let age:i32 = 8;
    if(age>=1 && age<=18){
        print!("Important Birthday");
    }else if(age == 21 || age == 50){
        println!("Important Birthday");
    }else if(age>=65){
        println!("Important Birthday");
    }else{
        println!("Not Important Birthday");
    }
}

fn ternary_operator(){
    let mut my_age:i32 = 47;
    let can_vote:bool = if my_age>=18{true}else{false};
    println!("can_vote : {}",can_vote);
}

fn match_or_swtich(){
    let age2:i32 = 8;
    match age2{
        1..=18 => println!("Important Birthday"),
        21 | 50 => println!("Important Birthday"),
        65..=i32::MAX => println!("Important Birthday"),
        _ => println!("Not Important Birthday"),
    };

    let my_age:i32 = 18;
    let voting_age:i32 = 18;
    match my_age.cmp(&voting_age) {
        Ordering::Less =>println!("Can't Vote"),
        Ordering::Greater =>println!("Can Vote"),
        Ordering::Equal =>println!("You have rights to vote"),
    };
}

fn arrays_loop(){
    let arr_0:[i32;4] = [1,2,3,4];
    let arr_1 = [1,2,3,4,5,6,7,8,9];
    println!("1st: {}",arr_1[0]);
    println!("len: {}",arr_1.len());

    // loop
    let mut loop_idx = 0;
    loop {
        if arr_1[loop_idx] % 2 == 0{
            loop_idx+=1;
            continue;
        }
        if arr_1[loop_idx] == 9{
            break;
        }
        println!("val : {}",arr_1[loop_idx]);
        loop_idx+=1;
    }

    //while loop
    loop_idx = 0;
    while loop_idx < arr_1.len(){
        println!("while arr : {}",arr_1[loop_idx]);
        loop_idx+=1;
    }

    // for loop
    loop_idx = 0;
    for val in arr_1.iter(){
        println!("forloop val : {}",val);
    }

}

fn tuples(){
    let my_tuple:(u8,String,f64) = (47,"Rabin".to_string(),50_000.00);
    println!("my name {}",my_tuple.1);
    let (v1,v2,v3) = my_tuple;
    println!("v1 {},v2 {} , v3 {}",v1,v2,v3);
}

fn strings(){
    let mut str_1 = String::new();
    str_1.push('A');
    str_1.push_str(" word");
    //split
    for word in str_1.split_whitespace(){
        println!("{}",word);
    }
    //replace
    let str_2 = str_1.replace('A', "Another");
    println!("{}",str_2);

    // sort | deduct duplicate
    let str_3 = String::from("x r t b h k k a m c");
    let mut v1:Vec<char> = str_3.chars().collect();
    v1.sort();
    v1.dedup();
    for char in v1{
        print!("{}",char);
    }
    println!();

    // to_string()
    let str_4 = "Random string";
    let mut str_5:String = str_4.to_string();
    println!("{}",str_5);

    // slice string
    let byte_arr1 = str_5.as_bytes();
    let str_6 = &str_5[0..6]; // dont include 6
    println!(" str len{}",str_6.len());
    str_5.clear();

    // cat
    let str_6 = String::from("just some");
    let str_7 = String::from("words");
    let str_8 = str_6 + &str_7; // now str_6 is no longer available but str_7 is.
    println!("str_8 {}",str_8);

    // unicode
    for char in str_8.bytes(){
        println!("unicode {}",char);
    }

}

fn casting(){
    let int1_u8:u8 = 5;
    let int2_u8:u8 = 4;
    let int3_u32:u32 = (int1_u8 as u32) + (int2_u8 as u32);
    println!("casting {}",int3_u32)
}

fn enums(){
    enum Day{
        Sunday,Monday,Tuesday,Wednusday,Thursday,Friday,Saturday
    }

    impl Day {
        fn is_weekend(&self)->bool{
            match self{
                Day::Saturday | Day::Sunday => true,
                _ => false
            }
        }
    }
    let today:Day = Day::Sunday;
    match today{
        Day::Sunday => println!("Its Sunday"),
        Day::Monday => println!("Its Monday"),
        Day::Tuesday => println!("Its Tuesday"),
        Day::Wednusday => println!("Its Wednusday"),
        Day::Thursday => println!("Its Thursday"),
        Day::Friday => println!("Its Friday"),
        Day::Saturday => println!("Its Saturday"),
    }
    println!("Its today the weekend {}",today.is_weekend());
}

fn vectors_func(){
    // vectors are arrays if they are mutable and can only store value of same type
    let vec1:Vec<i32> = Vec::new();
    let mut vec2:Vec<i32> = vec![1,2,3,4];
    vec2.push(5);
    println!("1st value :{}",vec2[0]);

    let second:&i32 = &vec2[1];
    match vec2.get(1) {
        Some(second) => println!("2nd : {}",second),
        None => println!("No 2nd value"),
    }

    for i in &mut vec2{
        *i *=2;
    }
    for i in &vec2{
        println!("{}",i)
    }

    println!("Vec length {}",vec2.len());
    println!("pop : {:?}",vec2.pop());
}

fn functions(){
    fn get_sum(x:i32,y:i32)->i32{
        x + y // it returns
    }
    println!("{}",get_sum(4, 5));

    fn get_2(x:i32,y:i32)->(i32,i32){
        return (x+y,x-y);
    }
    let (val1,val2) = get_2(6,4);
    println!("{} ,{}",val1,val2);

    fn sum_list(list: &[i32])->i32{
        let mut sum = 0;
        for &val in list.iter(){
            sum += &val;
        }
        sum // return
    }

    let num_list = vec![1,2,3,4];
    println!("sum of list = {}",sum_list(&num_list));
}

fn sum_generics<T:Add<Output = T>>(x:T,y:T)->T{
    return x+y;
}

fn ownerships(){
    // Stack : Stores values in a last in first out format
    // Heap  : When putting data in heap, we request a certain amount of space. Os finds space available and returns an address for that space called a pointer.
    // RULES
        // 1. Each values has a variable that's called its owner.
        // 2. There is only one owner at a time
        // 3. When the owner goes out of scope the value disappears.

    let str1 = String::from("world");
    // let str2 = str1; // str1 no longer exist.
    // println!("Hello {}",str1)
    let mut str3 = str1.clone();
    println!("Hello {}",str1);
    change_string(&mut str3)
}

fn change_string(name:&mut String){
    name.push_str(" is happy");
    println!("Message : {}",name)
}

fn hashmaps_dict(){
    let mut heros = HashMap::new();
    heros.insert("Superman", "Clark Kent");
    heros.insert("Batman", "Bruce Wayne");

    for (k,v) in heros.iter(){
        println!("key : {} - value : {}",k,v);
    }

    println!("length : {}",heros.len());

    // Containe or Includes
    if heros.contains_key(&"Superman"){
        let the_superman = heros.get(&"Superman");
        match the_superman{
            Some(x)=>println!("Superman is hero"),
            None => println!("Superman is not hero")
        }
    }
}

fn structs(){
    struct Customer{
        name:String,
        address:String,
        balance:f32,
    }
    let mut bob = Customer{
        name:String::from("Bob Smith"),
        address:String::from("Main st"),
        balance:34.34
    };
    bob.address = String::from("505 Main St");
    println!("bob name {}, address: {}, balance: {}",bob.name,bob.address,bob.balance)
}

fn traits(){
    const PI:f32 = 3.1415;
    trait Shape {
        fn new(legth:f32,width:f32)->Self; // constructor
        fn area(&self)->f32;
    }
    struct Rectangle{length:f32,width:f32};
    struct Circle {length:f32,width:f32};

    impl Shape for Rectangle{
        fn new(length:f32,width:f32)->Rectangle{
            return Rectangle { length, width }
        }
        fn area(&self)->f32{
            return self.length * self.width;
        }
    }
    impl Shape for Circle{
        fn new(length:f32,width:f32)->Circle{
            return Circle { length, width }
        }
        fn area(&self)->f32{
            return (self.length/2.0).powf(2.0)*PI;
        }
    }

    let rec:Rectangle = Shape::new(10.0,10.0);
    let cir:Circle = Shape::new(10.0,10.0);

    println!("Rec Area : {}",rec.area());
    println!("Cir Area : {}",cir.area());
}



fn main() {

    modules();
    // traits();
    // structs();
    // hashmaps_dict();
    // ownerships();
    // println!("5 + 4 = {}",sum_generics(5.4,4.8));
    // functions();
    // vectors_func();
    // enums();
    // casting();
    // strings();
    // tuples();
    // arrays_loop();
    // match_or_swtich();
    // ternary_operator();
    // if_condition();
    // numbers();
    // boolean();
    // data_types_max();
    // data_types();
    // input_output();
}   

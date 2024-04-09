use chrono::prelude::*;

fn main(){
    get_local_time();
    another_function(12);
    let a = summa();
    println!("Summ is: {a}");
    if_else(5);
    some_range();
    some_elements();
}

fn get_local_time(){
    let local_datetime: DateTime<Local> = Local::now();
    println!("Текущая дата: {}", local_datetime.format("%Y-%m-%d"));
    println!("Текущее время: {}", local_datetime.format("%H:%M:%S"))
}

fn another_function(x: i128){
    println!("the value of x is: {x}");
}

fn summa() -> i32{
    let y = {
        let x = 3;
        x + 1
    };
    y
}

fn if_else(number: i32){
    if number > 5{
        println!("More then five");
    } else if number == 5 {
        println!("Bro, low five!");
    } else if number < 5 {
        println!("Less then 5")
    } else {
        println!("Something another, dude")
    }
}

fn some_range(){
    let mut a: i32 = 0;
    for i in 1..5{
        println!("Now \"i\" is : {} and \"a\" is : {}", i, a);
        a += i
    }
}

fn some_elements(){
    let a = [10,20,30,40,50];

    let b = a.len();
    println!("Len of array is: {b}");

    for el in a {
        println!("the value is: {el}");
    }
}
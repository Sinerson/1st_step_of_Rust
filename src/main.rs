use chrono::prelude::*;

fn main(){
    get_local_time();
    another_function(12);
    let a = summa();
    println!("Summ is: {a}");
    if_else(5);
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
    if number <= 5{
        println!("condition was true");
    } else{
        println!("condition was false");
    }
}
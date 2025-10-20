use std::io;

fn main() {

    println!("What number do you want to count?");

    let mut user_input = String::new();
    io::stdin()
            .read_line(&mut user_input) //uses reference of guess via &
            .expect("Failed to read line");

    let number: u32 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => 0,// _ catches all values
        };
    

    println!("Counting to {number}");

    let mut start = 1;

    
    while start <= number{
        println!("Count {start}");
        start += 1;

    }
    

    // let mut count = 0;
    // 'counting_up: loop {
    //     println!("count = {count}");
    //     let mut remaining = 10;

    //     loop {
    //         println!("remaining = {remaining}");
    //         if remaining == 9 {
    //             break;
    //         }
    //         if count == 2 {
    //             break 'counting_up;
    //         }
    //         remaining -= 1;
    //     }

    //     count += 1;
    // }
    //println!("End count = {count}");

    // let result = loop {
    //     counter += 1;

    //     if counter == 10 {
    //         break counter *2;
    //     }
    // };

    // println!("the result is {result}");
}

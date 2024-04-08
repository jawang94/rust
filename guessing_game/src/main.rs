// use rand::Rng;
// use std::cmp::Ordering;
// use std::io;

fn main() {
    // let secret_number: u32 = rand::thread_rng().gen_range(1..100);

    // loop {
    //     let mut guess = String::new();
    //     println!("Guess the number.");
    //     println!("Please input your guess!");
    //     io::stdin()
    //         .read_line(&mut guess)
    //         .expect("Something went so wrong.");

    //     let guess = match guess.trim().parse::<u32>() {
    //         Ok(num) => num,
    //         Err(_) => continue,
    //     };

    //     println!("You guessed: {guess}");
    //     let test_arr = ["yo"; 3];
    //     println!("Element: {:?}", test_arr);
    //     match guess.cmp(&secret_number) {
    //         Ordering::Less => println!("Too small!"),
    //         Ordering::Greater => println!("Too big!"),
    //         Ordering::Equal => {
    //             println!("BINGO!");
    //             break;
    //         }
    //     }
    // }

    let val = 24;
    let the_ref = &val;
    let de_ref = *the_ref;
    println!("{the_ref} and {de_ref}");

    // let buzz = false;
    // let the_arg = if buzz { Some(13) } else { None };
    // let got_it = another_function(the_arg);
    // println!("Damn we got it: {:?}", got_it.unwrap());
    // let did_not_got_it = another_function(None);
    // println!("Damn we did not got it: {:?}", did_not_got_it.unwrap());
}

fn takes_ownership(some_str: &mut String) {
    some_str.push_str(" yeeee boi");
    println!("gotcha betch {some_str}");
}

// fn another_function(x: Option<u32>) -> Option<u32> {
//     let mut counter = 0;
//     let result = 'counting: loop {
//         counter += 1;

//         if counter == 10 {
//             break 'counting counter * 30;
//         }
//     };
//     let arrgo = [1, 2, 3, 4, 5];
//     for x in arrgo.iter().rev() {
//         println!("x is {x}");
//     }
//     println!("Bingo: {result}");
//     if x.is_some() {
//         println!("Wow, we got an arg {:?}", x);
//         return x;
//     } else {
//         return Some(32);
//     }
// }

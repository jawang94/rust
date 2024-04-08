use std::fs::File;
use std::io::ErrorKind;

// fn main() {
//     let greeting_file_result = File::open("hello.txt");

//     let greeting_file = match greeting_file_result {
//         Ok(file) => file,
//         Err(error) => match error.kind() {
//             ErrorKind::NotFound => match File::create("./src/hello.txt") {
//                 Ok(fc) => fc,
//                 Err(e) => panic!("Problem creating the file: {:?}", e),
//             },
//             other_error => {
//                 panic!("Problem opening the file: {:?}", other_error);
//             }
//         },
//     };
// }

fn main() {
    let _greeting_file = File::open("hello.txt").unwrap_or_else(|error_opening| {
        if error_opening.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error_creating| {
                panic!("Problem creating the file uh oh: {:?}", error_creating);
            })
        } else {
            panic!("Problem opening the file: {:?}", error_opening)
        }
    });
}

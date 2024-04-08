#[derive(Debug)]
struct GenericStruct<T> {
    codeword: T,
    answer: T,
}

impl<T> GenericStruct<T> {
    fn get_codeword(&self) -> &T {
        &self.codeword
    }
}

fn main() {
    let brahh = GenericStruct {
        codeword: 1,
        answer: 2,
    };
    let sekki = GenericStruct {
        codeword: "heeyyy",
        answer: &String::from("yoboseo"),
    };
    println!("yee {:#?} and {:#?}", brahh, sekki);
    println!("testing: {:?}", brahh.get_codeword());
}

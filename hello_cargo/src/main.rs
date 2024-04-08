use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    let mut vec: Vec<i32> = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    println!("The vec {:?}", vec);

    let mut vec2 = vec![1, 2, 3, 4, 5];
    let third: &i32 = &vec2[2];
    let get_third: Option<i32> = match vec2.get(100) {
        Some(val) => Some(*val),
        None => None,
    };
    println!("Third el of vec2 is {third}");
    println!("Get Third el of vec2 is {:?}", get_third);

    vec2.push(6);
    let first = &vec2[0];
    println!("Yee vec2 6 is {first}");

    play_with_vecs();
    play_with_hash_maps();
}

#[derive(Debug)]
enum VecData {
    Int(i32),
    Text(String),
    Float(f32),
}

fn play_with_vecs() {
    let row = vec![
        VecData::Int(3),
        VecData::Text(String::from("yoboseo")),
        VecData::Float(10.12),
    ];

    for x in &row {
        match x {
            VecData::Int(val) => println!("The super vec is {:?}", val),
            VecData::Text(val) => println!("The super vec is {:?}", val),
            VecData::Float(val) => println!("The super vec is {:?}", val),
        }
    }
}

fn play_with_hash_maps() {
    let mut scores = HashMap::new();
    scores.insert("Blue", 10);
    scores.insert("Yellow", 10);
    println!("Dat Hash Map: {:#?}", scores);
    let got_em = scores.get("efawefawe").copied().unwrap_or(0);
    println!("Got em {got_em}");

    let mut test_map = HashMap::new();
    let let_me_live = String::from("should_still");
    test_map.insert("yasss", let_me_live.clone());
    test_map.entry("yasss233").or_insert(String::from("kekeke"));
    println!("let_me_live is alive! {:?}", let_me_live);
    println!("test_map bruv {:#?}", test_map);
}

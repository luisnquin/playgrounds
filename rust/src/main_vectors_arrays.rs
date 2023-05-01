fn main() {
    let mut words = ["hihihi", "hahaha"];

    for ele in words.map(|v| return v) {
        println!("{}", ele)
    }

    words[1] = "aja aja aja";

    println!(
        "first element is: '{}', and the other is: '{}'",
        words[0], words[1]
    );

    let fruits = vec!["banana"];

    let mut with_capacity = vec![2; 5];
    with_capacity.push(55);

    println!("{:?}", with_capacity);

    let mut empty_crate: Vec<String> = Vec::new();
    empty_crate.push("not empty now".to_owned());

    println!("{:?}", empty_crate);
}

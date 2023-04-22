fn main(){
    let somewords = "you are my sweet girl";
    println!("{}",somewords);
    for i in somewords.split_whitespace().rev(){
        print!("{}",i);
    }
    println!("");
    let mut chars:Vec<char> = somewords.chars().collect();
    chars.sort();
    
    chars.dedup();
    println!("{:?}",chars);

    let mut str = String::new();
    for i in chars{
        str.push(i);
        str.push_str(",");
    }
    println!("{}",str);

    let chars_to_trim:&[char] = &[' ',','];
    let trim_chars = str.trim_matches(chars_to_trim);
    println!("{}",trim_chars);

    let alice = String::from("i like dog");
    let bob = alice.replace("dog", "cat");
    println!("{}",alice);
    println!("{}",bob);

}
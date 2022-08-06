use std::{fs, io};

mod trie_tree;

fn main() {
    let input_string = fs::read_to_string("./input").unwrap();

    let mut words: Vec<&str> = input_string.split('\n').collect();
    words.pop();

    let mut trie = trie_tree::Trie::new();
    for (i, word) in words.iter().enumerate() {
        trie.insert(word, i as i32);
    }

    loop {
        let mut input_string = String::new();
        println!("Please input search word(If you want to end to input, input EOF):");
        if let Err(err) = io::stdin().read_line(&mut input_string) {
            panic!("input error: {}", err);
        };
        let input_string = input_string.trim();

        let result = trie.search(input_string);
        if result {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

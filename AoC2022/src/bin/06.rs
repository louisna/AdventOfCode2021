use std::collections::HashSet;

fn main() {
    let content = include_str!("../../inputs/06.txt");

    let chars: Vec<char> = content.chars().collect();
    'large: for (i, window) in chars.windows(14).enumerate() {
        let mut hash = HashSet::new();
        for char in window.iter() {
            if hash.contains(char) {
                continue 'large;
            }
            hash.insert(char);
        }
        println!("Res 1: {}", i + 14);
        return;
    }
}

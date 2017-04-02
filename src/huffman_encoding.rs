use std::collections::HashMap;

// type Node = Option<Box<HuffmanNode>>;

// struct HuffmanNode {
//     left: Node,
//     right: Node,
//     value: BitVec,
// }

fn get_frequencies(text: &str) -> HashMap<char, usize> {
    let mut frequencies_by_letter = HashMap::new();

    for letter in text.chars() {
        let entry = frequencies_by_letter.entry(letter).or_insert(0);
        *entry += 1;
    }

    frequencies_by_letter
}

#[cfg(test)]
mod tests {
    use huffman_encoding::get_frequencies;
    use std::collections::HashMap;

    #[test]
    fn test_get_frequencies() {
        let frequencies = get_frequencies("hello");
        assert_eq!(frequencies.get(&'h'), Some(&1));
        assert_eq!(frequencies.get(&'e'), Some(&1));
        assert_eq!(frequencies.get(&'l'), Some(&2));
        assert_eq!(frequencies.get(&'o'), Some(&1));
    }
}
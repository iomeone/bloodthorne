use bit_vec::BitVec;
use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

type Node = Option<Box<HuffmanNode>>;

#[derive(Clone, Eq, PartialEq)]
struct HuffmanNode {
    left: Node,
    right: Node,
    count: usize,
    value: BitVec,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for HuffmanNode {
    fn cmp(&self, other: &HuffmanNode) -> Ordering {
        // Notice that the we flip the ordering here
        other.count.cmp(&self.count)
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for HuffmanNode {
    fn partial_cmp(&self, other: &HuffmanNode) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl HuffmanNode {
    fn new(count: usize) -> HuffmanNode {
        HuffmanNode {
            left: None,
            right: None,
            count: count,
            value: BitVec::new(),
        }
    }

    fn from(text: &str) -> HuffmanNode {
        let frequencies_by_letter = get_frequencies(text);
        let mut priority_queue = make_priority_queue(&frequencies_by_letter);
        HuffmanNode::build_tree(&mut priority_queue)

        // TODO: assign bits
    }

    fn build_tree(priority_queue: &mut BinaryHeap<HuffmanNode>) -> HuffmanNode {
        while priority_queue.len() > 1 {
            let first = priority_queue.pop().unwrap();
            let second = priority_queue.pop().unwrap();
            let first_count = first.count;
            let second_count = second.count;

            let parent = HuffmanNode {
                left: Some(Box::new(first)),
                right: Some(Box::new(second)),
                count: first_count + second_count,
                value: BitVec::new(),
            };
            priority_queue.push(parent);
        }

        priority_queue.pop().unwrap()
    }

    // fn assign_bits(node: &mut HuffmanNode) {
    //     if let Some(ref left) = node.left {
    //         left.value.push(false);
    //         HuffmanNode::assign_bits(&mut left);
    //     }
    //     if let Some(ref right) = node.right {
    //         right.value.push(true);
    //         HuffmanNode::assign_bits(&mut right);
    //     }
    // }
}

type frequency_pair = Option<(char, usize)>;

fn make_priority_queue(frequencies: &HashMap<char, usize>) -> BinaryHeap<HuffmanNode> {
    let mut binary_heap = BinaryHeap::new();

    for (key, value) in frequencies.iter() {
        binary_heap.push(HuffmanNode::new(*value));
    }

    binary_heap
}

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
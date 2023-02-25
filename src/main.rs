use rand::Rng;
mod heap_sort;
use trie::trie::Trie;
mod trie;

fn main() {
    /*
    let mut rng = rand::thread_rng();
    let mut vec: Vec<i32> = (0..10).map(|_| rng.gen_range(1..21)).collect();
    heap_sort::heap_sort::heapsort(&mut vec);
    println!("{:?}", vec);
    */

/*
['aab', 'defgab', 'abcde', 'aabcde', 'bbbbbbbbbb', 'jabjjjad']
*/

    let mut trie = Trie::new();

    trie.insert("aab");
    println!("{}", trie.has_prefix("aab"));
    trie.insert("aac");
    println!("{}", trie.has_prefix("aac"));
    trie.insert("aacghgh");
    println!("{}", trie.has_prefix("aacghgh"));
}

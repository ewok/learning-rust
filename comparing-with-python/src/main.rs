fn main() {
    // # python:
    // numbers = []
    // for i in range(1, 100):
    //   numbers.append(i)
    let mut numbers = vec![];
    for i in 1..100 {
        numbers.push(i);
    }
    println!("{:?}", numbers);

    // # python:
    // numbers = [i for i in range(1, 100)]
    numbers = (1..100).collect();
    println!("{:?}", numbers);

        // # python:
    // with open("words.txt") as f:
    //   for line in f:
    //     print(line)
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let input = File::open("words.txt").unwrap();
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        println!("{}", line.unwrap());
    }

    // # python:
    // words = [word for word in open("words.txt")
    let words: Vec<_> = BufReader::new(File::open("words.txt").unwrap())
        .lines()
        .filter_map(Result::ok)
        .collect();
    println!("{:?}", words);

    // # python
    // words_length = { word: len(word) for word in words }
    use std::collections::HashMap;
    let words_length: HashMap<String, usize> = words.iter()
        .map(|word| (word.to_string(), word.len()))
        .collect();
    println!("{:?}", words_length);

    // # python:
    // from collections import defaultdict
    // word_lengths = defaultdict(set)
    // for word in words:
    //   word_lengths[len(word)].add(word)
    use std::collections::HashSet;
    let mut word_lengths: HashMap<usize, _> = HashMap::new();
    for word in words {
        word_lengths.entry(word.len()).or_insert(HashSet::new()).insert(word);
    }
    println!("{:?}", word_lengths[&4]);

}

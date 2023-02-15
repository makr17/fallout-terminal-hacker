use std::collections::{HashMap, HashSet};
use std::io;

fn main() {
    let mut words = get_words();
    while words.len() > 1 {
	let guess = optimal(&words);
	println!("{} is optimal", guess);
	let count = get_count();
	words = filter_words(&guess, count, &words);
	println!("{:?}", words);
    }
}

fn get_input() -> String {
    let stdin = io::stdin();
    let mut input = String::new();
    let _read = stdin.read_line(&mut input);
    input.pop();
    input
}

fn get_words() -> HashSet<String> {
    let mut words = HashSet::new();
    println!("Enter words, one per-line, empty line when done.");
    loop {
	let input = get_input();
	if input.is_empty() {
	    break;
	}
	words.insert(input);
    }
    words
}

fn get_count() -> u32 {
    println!("What was the reported count?");
    let count = get_input();
    count.parse::<u32>().unwrap()
}

fn optimal(words: &HashSet<String>) -> String {
    let mut remains: Vec<&String> = words.iter().collect();
    remains.sort_unstable();
    let len = remains.len();
    let mut grid = vec![vec![0; len]; len];
    for i in 0..len {
	for j in 0..len {
	    grid[i][j] = compare_words(&remains[i], &remains[j]);
	}
    }
    let mut intersects: HashMap<&String, usize> = HashMap::new();
    for i in 0..len {
	// count distinct intersection counts for each word
	let mut tmp: HashSet<u32> = HashSet::new();
	for j in 0..len {
	    tmp.insert(grid[i][j]);
	}
	intersects.insert(remains[i], tmp.len());
    }
    let mut sorted: Vec<(&&String, &usize)> = intersects.iter().collect();
    sorted.sort_unstable_by(|a,b| a.1.cmp(&b.1).reverse());
    sorted.first().unwrap().0.clone().to_string()
}

#[test]
fn test_optimal() {
    let words: HashSet<String> = HashSet::from([
        "aerial".to_string(),
        "babied".to_string(),
        "bagged".to_string(),
        "backer".to_string(),
        "ballad".to_string(),
        "bemoan".to_string(),
        "calves".to_string(),
        "canyon".to_string(),
        "citrus".to_string(),
        "decked".to_string(),
        "denial".to_string(),
        "dimmer".to_string(),
        "faucet".to_string(),
        "fasten".to_string(),
        "ferret".to_string(),
        "gambit".to_string(),
        "garden".to_string(),
        "genial".to_string(),
        "healer".to_string(),
        "heaven".to_string(),
        "harden".to_string(),
        "jagged".to_string(),
        "killer".to_string()
    ]);
    assert_eq!(optimal(&words), "bagged".to_string());
}

fn filter_words(guess: &String, count: u32, words: &HashSet<String>) -> HashSet<String> {
    let mut remains = HashSet::new();
    for word in words.iter() {
	if word == guess {
	    continue;
	}
	if compare_words(guess, word) == count {
	    remains.insert(word.clone());
	}
    }
    remains
}

#[test]
fn test_filter_words() {
    let words: HashSet<String> = HashSet::from([
        "aerial".to_string(),
        "babied".to_string(),
        "bagged".to_string(),
        "backer".to_string(),
        "ballad".to_string(),
        "bemoan".to_string(),
        "calves".to_string(),
        "canyon".to_string(),
        "citrus".to_string(),
        "decked".to_string(),
        "denial".to_string(),
        "dimmer".to_string(),
        "faucet".to_string(),
        "fasten".to_string(),
        "ferret".to_string(),
        "gambit".to_string(),
        "garden".to_string(),
        "genial".to_string(),
        "healer".to_string(),
        "heaven".to_string(),
        "harden".to_string(),
        "jagged".to_string(),
        "killer".to_string()
    ]);
    let expect: HashSet<String> = HashSet::from([
        "aerial".to_string(),
        "citrus".to_string(),
        "denial".to_string(),
        "genial".to_string()
    ]);
    assert_eq!(
	filter_words(&"bagged".to_string(), 0, &words),
	expect
    );
}

fn compare_words(word: &String, guess: &String) -> u32 {
    let chars: Vec<char> = word.chars().collect();
    let mut matches = 0;
    for (i,c) in guess.chars().enumerate() {
	if chars[i] == c {
	    matches += 1;
	}
    }
    matches
}

#[test]
fn test_compare_words() {
    assert_eq!(compare_words(&"ferret".to_string(), &"heaven".to_string()), 2);
    assert_eq!(compare_words(&"bagged".to_string(), &"backer".to_string()), 3);
}

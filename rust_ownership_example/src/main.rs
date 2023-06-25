use std::clone;

fn main() {
    let sentence1 = String::from("The quick brown fox");
    let sentence2 = String::from("jumps over the lazy dog");

    //using borrowing with "&"
    let merged_sentence = merge_sentences(&sentence1, &sentence2);
    println!("Merged sentence using borrowing: {}", merged_sentence);

    //using cloning to create a copy of the string
    let cloned_sentence = merged_sentence.clone();
    println!("Cloned the merged sentence: {}", cloned_sentence);

    //passing the ownership of merged_sentence
    ownership(merged_sentence);
    
    /*
    the following line will not work compile because the ownership of merged_sentence is passed to the function "ownership"
    println!("The merged sentence is: {}", merged_sentence);
    */
}

//function to merge two sentences
fn merge_sentences(sentence1: &str, sentence2: &str) -> String {
    let mut merged = String::from(sentence1);
    merged.push(' ');
    merged.push_str(sentence2);
    merged
}

//function to take ownership of the string
fn ownership(msg: String) {
    println!("Ownership is transferred: {}", msg);
}

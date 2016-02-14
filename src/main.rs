mod words;
extern crate rand;

use rand::{thread_rng, Rng};

fn main(){
    // generate random number generator
    let mut rng = thread_rng();

    // a mutable list of strings we've selected as random words
    let mut random_words = vec![]; 

    loop{
        // stop looping if we already have 4 words
        if random_words.len() == 4{
            break;
        }
        // get a random number between 0 and len() of WORDS
        let i = rng.gen_range(0,words::WORDS.len());
        // get the word at the index
        let word = words::WORDS[i];

        // check if the word we just got is part of the list already
        // continue looping if so
        if random_words.contains(&word){
            continue;
        }
        // add it to the array otherwise
        random_words.push(&word);
    }

    // join and print
    println!("{}",random_words.join(""));
}

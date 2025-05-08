use std::time::Instant;
use std::thread;
use compile::{process_data, Trees, Scope};

fn main() {
    let now = Instant::now();
    
    let handles: Vec<_> = vec![
        thread::spawn(|| process_data(Trees::Trie, Scope::Line)),
        thread::spawn(|| process_data(Trees::Trie, Scope::Word)),
        thread::spawn(|| process_data(Trees::Suffix, Scope::Line)),
        thread::spawn(|| process_data(Trees::Suffix, Scope::Word)),
        thread::spawn(|| process_data(Trees::NGramIndex, Scope::Line)),
        thread::spawn(|| process_data(Trees::NGramIndex, Scope::Word)),
    ];

    for handle in handles {
        handle.join().expect("Thread panicked");
    }
    
    let time_taken = now.elapsed().as_secs_f32();
    eprintln!("Time taken to process document - {}", time_taken);
}

> [!todo] Checklist
----------
| What? | what-sss? |
| -------------- | --------------- |
| [x] Sceletal Structure(1 day)| - pre-processing(SEARCH_SCOPE) [Compile] |
|  | - match statments for deserialization(ENUMS) [Runtime] |
|  | - tidy up the UI using ratatui [Runtime] |
|  .............................| ....................................................| 
| [-] Data Stuctures(3 day)| - implement the suffix tree|
|  | - implement the inverted index  |

----------
----------
1. partioning not to reserial...
2. live index
----------
----------

Steps to handling serialization
1. test out the build.rs and serialize the stored date 
2. recover the serialized data and use it 

> [!check] Serialization Answer
```Rust
// for the build.rs 
use std::{fs::File, io::Write};
use bincode::config;
use unicode_segmentation::UnicodeSegmentation;
use std::path::Path;

fn main() {
    let a = "hi there niggas i have bubble gum and a can of whipass, and i just ran out of bubble gum";
    let point = a.unicode_words().collect::<Vec<_>>();

    let serialized = bincode::encode_to_vec(point, config::standard()).unwrap();

    let out_path = Path::new("serialized.bin");

    let mut new = File::create(out_path).unwrap();
    new.write_all(&serialized).unwrap();

    eprintln!("serialized = {:?}", serialized)
}

//for the main.rs 
use std::fs;
use bincode::config;

fn main() {

    let contents = fs::read("serialized.bin").unwrap();
    let deserialized:(Vec<String>,_) = bincode::decode_from_slice(&contents, config::standard()).unwrap();
    println!("The contents you wrote are \n {:#?}", deserialized);
}
```

----------
----------

Steps to handling tokenization
1. use UnicodeSegmentation(for words)
2. use split('\n')(for lines)

> [!check] Tokenization Answer
```Rust
//for tokenizing words
dataset.unicode_words().collect::<Vec<_>>();

//for tokenizing words
dataset.split('\n').collect::<Vec<_>>();
```

----------
----------

use std::fs::File;
use std::io::Read;

use memmap::Mmap;

fn main(){
    let mut file = File::open("test.txt").unwrap();
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).unwrap();
    let mmap = unsafe { Mmap::map(&file).unwrap()  };
    assert_eq!(&contents[..], &mmap[..]);

    println!("{:?}", mmap.to_str());
}
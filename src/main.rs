use std::path::Path;

mod parsers;
mod prompt;
mod sorter;

fn main() {
    let sorter = sorter::Sorter::new("{author}/{title}.{ext}");
    let scan_path = Path::new("/home/ralim/Downloads/ebooks/");
    let library_path = Path::new("/mnt/RustTank/eBooks/");
    sorter.sort_recursively(scan_path, library_path);
}

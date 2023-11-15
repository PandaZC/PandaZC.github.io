use std::io::Read;

// fn main() {
//     println!("Hello, world!");
// }


fn main() {
    let mut file = std::fs::File::open(r"C:\Users\zhang\Desktop\RUST学习\examples\file_learn\src\data.txt").unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    println!("{}", contents);

    file.read(buf)
}
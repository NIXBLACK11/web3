fn main() {
    // data types, array, vectors
    println!("Hello, world!");
    let ans = sum(5, 10);
    println!("{}", ans);
    test_string();
    let vect = vec![1, 2, 3];
    println!("{:?}", vect);

    let arr = [1, 2, 3];
    println!("{}", arr.len());

    // conditions and loops
    loops_conditionals();

    // Owner ship rules

    ownership_rules();

    let name = String::from("Siddharth");
    println!("{}", get_len(name));

    // This line gives error as the ownersshiop of the name value has been transferred to the s variable
    // println!("{}", name);

    // Possible solution
    /*
    let (length, name) = get_len2(name);
    println!("{}", length);
    println!("{}", name);
    */

    // one good ear is to transfer omli tje read onli referense using &
    println!("{}", get_len3(&name));

}

fn sum(a: u32, b: u32) -> u32 {
    return a+b;
}

fn test_string() {
    let name = String::from("Siddharth");
    println!("{}", name);
}

// Loops and conditionals
fn loops_conditionals() {
    let s = String::from("Test string ");
    for ch in s.chars() {
        if ch==' ' {
            println!();
        } else {
            print!("{}", ch);
        }
    }
}

// Ownership
// Rust says if the owner of a variable goes out of scope the value gets deleted, but if more than two point to the same value we get dangling pointers.
fn ownership_rules() {
    let name = String::from("Siddharth Rana");
    // let name2 = name;
    let name2 = name.clone();

    println!("{}", name);
}

fn get_len(s: String) -> usize {
    return s.len();
}

fn get_len2(s: String) -> (usize, String) {
    return (s.len(), s);
}

fn get_len3(s: &String) -> usize {
    return s.len();
}
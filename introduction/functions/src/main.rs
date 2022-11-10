fn function(n: i32) -> i32 {
    n + 1; // error -> ()
    return n + 1; // it works -> 2
    n + 1 // best practice -> 2
}

fn main() {
    println!("{}", function(1));
}

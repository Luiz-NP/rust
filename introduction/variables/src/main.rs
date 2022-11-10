fn main() {
    let n1 = 1; // imutable
    let mut n2 = 1; // mutable

    println!("{n1}, {n2}"); // 1, 1

    // new scope
    {
        let n1 = 2; // shadowing
        n2 = 2; // changing

        println!("{n1}, {n2}"); // 2, 2
    }

    println!("{n1}, {n2}"); // 1, 2
}

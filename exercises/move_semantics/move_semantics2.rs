// move_semantics2.rs
// Make me compile without changing line 13!
// Execute `rustlings hint move_semantics2` for hints :)


fn main() {
    let vec0 = Vec::new();

    // Answer 1) make another, separate version of the data in vec0.
    //let vec0Copy = vec0.clone();
    //let mut vec1 = fill_vec(vec0Copy);

    // Answer 2) make the function borrow its argument instead of taking ownership.
    //let mut vec1 = fill_vec(&vec0);

    // Answer 3) make mutable to allow the function borrow and modify directly the argument.
    let mut vec0 = vec0;
    let mut vec1 = fill_vec(&mut vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

// Answer 1) no changes to the function
//fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
//    let mut vec = vec;
//
//    vec.push(22);
//    vec.push(44);
//    vec.push(66);
//
//    vec
//}

// Answer 2) make the function borrow its argument and copy its data
//fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
//    let mut vec = vec.clone();
//
//    vec.push(22);
//    vec.push(44);
//    vec.push(66);
//
//    vec
//}

// Answer 3) make the function mutably borrow and its argument, modify directly, return nothing.
fn fill_vec(vec: &mut Vec<i32>) -> Vec<i32> {
    vec.push(22);
    vec.push(44);
    vec.push(66);

    Vec::new()
}

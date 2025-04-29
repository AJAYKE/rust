// Ownership --> one variable should have only one owner, when the variable goes out of scope, memory is freed on heap

pub fn owner() {
    let a = String::from("hello");
    let b = a;

    println!("a = {}", a); // this will throw error, because a is moved to b
    println!("b = {}", b); // this will work, because b is the owner of the string
}


// Moving --> When the variable is passed to a function or it is assigned to some other variable, the ownership is moved to that variable, and the original variable is no longer valid

fn moving() {
    let a = String::from("hello");
    let b = takes_ownership(a);
    // println!("a = {}", a); // this will throw error, because a is moved to the function
    println!("b = {}", b); // this will work, because b is the owner of the string
}

fn takes_ownership(s1: String) -> String {
    println!("s = {}", s1);
    return s1;
}
// Borrowing --> it is like pointers in c, when we pass the variable to a function, we can pass the pointer of the variable, so that the original variable is not moved, and we can use it afterwards

fn borrowing() {
    let a = String::from("hello");
    let b = &a; // here we are passing the pointer of the variable, so that we can use it afterwards
    println!("a = {}", a); // this will work, because a is not moved
    println!("b = {}", b); // this will work, because b is the pointer of the variable
}

// Mutable Borrowing --> when we pass the variable to a function, we can pass the mutable references of the variable, so that we can change the value of the original variable, but we can only have one mutable reference at a time othewise it will throw error.

fn mutable_borrowing() {
    let mut a = String::from("hello");
    let b = &mut a; // here we are passing the mutable reference of the variable, so that we can change the value of the original variable
    b.push_str(" world");
    // println!("a = {}", a); // this will not work, because it should have only one mutable reference at a time
    println!("b = {}", b); // this will work, because b is the mutable reference of the variable
}
// clone -> to create a deep copy of a variable and reassign to other variable

fn clone() {
    let a = String::from("hello");
    let b = a.clone(); // here we are creating a deep copy of the variable
    println!("a = {}", a); // this will work, because a is not moved
    println!("b = {}", b); // this will work, because b is the deep copy of the variable
}

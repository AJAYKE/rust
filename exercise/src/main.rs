use traits::{Secondary, Summary, User};

mod is_even;
mod fibonacci;
mod len_of_string;
mod structs;
mod enums;
mod packages;
mod vectors;
mod hashmaps;
mod iterators;
mod traits;
mod multithreading;

fn main() {
    println!("Running test");
    let num = 4;
    println!("Is {} even? {}", num, is_even::run(num));

    let num = 6;
    println!("The {}th number in the fibonacci sequence is {}", num, fibonacci::fibonacci(num));


    let var = String::from("hey there i am ajay");
    let len = len_of_string::len_of_the_string(&var); // here we are passing the pointer of the varible, if we pass the variable directly we cant use it afterwards, as it will be moved completely to the child, or we can use `var.clone()` to create a deepcopy of var and pass it
    println!("the length of the string '{}' is {}",var, len);

    let user = structs::User{
        first_name: String::from("Ajay"),
        last_name: String::from("Kumar"),
        age: 24,
    };
    println!("{}",user.get_full_name());


    let rect: structs::Rect = structs::Rect {
        width: 10,
        height: 20,
    };
    println!("the area and perimeter of the rectanges with shapes 10*20 is {} and {} respectively", rect.area(), rect.perimeter());


    let rect =  enums::Shape::Rectangle(12.123, 11.31);
    let circle = enums::Shape::Circle(10.0);

    println!("The area of the rectangle is {}", enums::calcualte_area(rect));
    println!("The area of the circle is {}", enums::calcualte_area(circle));


    let index = enums::find_first_a(String::from("first occurence of a in this string is at"));

    match index {
        Some(index) => println!("The first occurence of a in the string is at {}", index),
        None => println!("There is no a"),
    }


    let path = String::from("./src/test.txt");
    let contents = enums::read_a_file(path);
    match contents {
        Ok(contents) => println!("The contents of the file are '{}'", contents),
        Err(e) => println!("Error reading file: {}", e),
    }

    println!("the current local time is {}", packages::get_local_time());
    println!("the current utc time is {}", packages::get_utc_time());

    vectors::getting_started_with_vectors();

    let mut nums: Vec<i32> = vec![1, 2, 3, 4, 5, 5, 6, 7];
    vectors::filter_evens(&mut nums);
    println!("{:?}",nums);

    
    hashmaps::getting_started_with_hashmaps();

    hashmaps::hashmap_with_vectors_from_vector_tupes(vec![("yo","hi"),("yo","bye"),("haha","hehe"),("rambo","gunther")]);

    iterators::getting_srarted_with_iterators();

    let user1: User = traits::User{name:String::from("Ajay"), age:24};

    println!("{}", user1.summarise());
    println!("{}", user1.yoboi());
    traits::checkin(user1);


    multithreading::calculate_huge_sum();
}


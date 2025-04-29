use std::fs;

pub enum Shape{
    Rectangle(f64, f64),
    Circle(f64),
}

pub fn calcualte_area(shape: Shape) -> f64 {
    match shape {
        Shape::Rectangle(width,height) => width*height,
        Shape::Circle(radius) => 3.14*radius*radius,
    }
}

pub fn find_first_a(var: String) -> Option<i32> {
    for (index,char) in var.chars().enumerate(){
        if char == 'a' {
            return Some(index as i32);
        }
    }
    return None;
}

pub fn read_a_file(path: String) -> Result<String, String> {
    let contents = fs::read_to_string(path);
    match contents {
        Ok(contents) => Ok(contents),
        Err(e) => Err(String::from(format!("Error reading file: {}", e))),
    }
}
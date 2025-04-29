pub trait Summary{
    fn summarise(&self)->String; // this is like an interface or abstract method
}

pub trait Secondary{
    fn yoboi(&self) -> u32 {
        return 23;
    }
}

pub struct User{
   pub name: String,
    pub age: u32,
}


impl Summary for User {
    fn summarise(&self)-> String {
        return format!("User {} is {} years old", self.name, self.age)
    }
}

impl Secondary for User{
    fn yoboi(&self) -> u32 {
        return self.age
    }
}

pub fn checkin(yo:impl Secondary) {
    println!("checkin using impl inside func {}", yo.yoboi())
}
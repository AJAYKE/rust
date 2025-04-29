pub struct User {
    pub first_name: String,
    pub last_name: String,
    pub age: u32,
}

impl User {
    pub fn get_full_name(&self) -> String {
        return format!("{} {} and he is {} years old", self.first_name, self.last_name, self.get_age());
    }

    pub fn get_age(&self) -> u32 {
        return self.age;
    }
}

pub struct Rect {
    pub width: u32,
    pub height: u32,
}

impl Rect {
    pub fn perimeter(&self) -> u32 {
        return 2*self.width +2*self.height;
    }

    pub fn area(&self) -> u32 {
        return self.width * self.height;
    }

}
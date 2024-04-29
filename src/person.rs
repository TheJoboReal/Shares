pub struct Person {
    pub name: String,
    pub debt: f32,
}


impl Person {
    pub fn new(name: String, debt: f32) -> Person {
        Person{
            name,
            debt,
        }
    }
}


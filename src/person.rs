
struct Person {
    name: String,
    debt: f32,
}

fn build_person(name_: String, debt_: f32) -> Person {
    Person{
        name: name_,
        debt: debt_,
    }
}
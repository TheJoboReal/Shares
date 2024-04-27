use Person;

struct Persons{
    totalDebt: f32,
    persons: Vec<Person>,
}

fn build_persons(totalDebt_: f32, persons_: Vec<Person>) -> Persons {
    Persons{
        totalDebt: totalDebt_,
        persons: persons_,
    }
}


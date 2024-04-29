mod person;
mod persons;

use person::Person;
use persons::Persons;

fn main() {
    let mut p1 = Person::new("John".to_string(), 150.0);
    let mut p2 = Person::new("Kasper".to_string(), 40.0);
    let mut p3 = Person::new("Runa".to_string(), 20.0);
    let mut people = vec![p1, p2, p3];

    let mut persons = Persons::new(people);

    persons.printPersons();
}


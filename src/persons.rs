use crate::person::Person;

pub struct Persons{
    totalDebt: f32,
    persons: Vec<Person>,
}

impl Persons {
    pub fn new(persons: Vec<Person>) -> Persons {
        Persons{
            persons,
            totalDebt: 0.0,
    }
}

    // fn updateTotalDebt(&self, totalDebt_: f32) {
    //     let mut debt_temp : f32 = 0.0;
    //     for element in self.persons.totalDebt{
    //         debt_temp += element;
    //     }
    //     totalDebt_ = debt_temp;
    // }

    // fn updatePersonDebt(&self) {
    //     let mut equalShare : f32  = self.totalDebt / self.persons.len();
    //     for element in self.persons {
    //         self.persons[element].debt = equalShare - self.persons[element].debt;
    //     }
    // }

    pub fn printPersons(&self){
        for element in &self.persons{
            println!("Name: {}", element.name);
            println!("Debt: {}", element.debt);
        }
    }
}
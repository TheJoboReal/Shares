use crate::person::Person;

pub struct Persons{
    pub total_debt: f32,
    pub persons: Vec<Person>,
}

impl Persons {
    pub fn new(persons: Vec<Person>) -> Persons {
        Persons{
            persons,
            total_debt: 0.0,
    }
}

    pub fn update_total_debt(&mut self) {
        let mut debt_temp : f32 = 0.0;
        for element in &self.persons{
            debt_temp += element.debt;
        }
        self.total_debt = debt_temp;
    }

    // fn updatePersonDebt(&self) {
    //     let mut equalShare : f32  = self.totalDebt / self.persons.len();
    //     for element in self.persons {
    //         self.persons[element].debt = equalShare - self.persons[element].debt;
    //     }
    // }

    pub fn print_persons(&self){
        for element in &self.persons{
            println!("Name: {}", element.name);
            println!("Debt: {}", element.debt);
        }
    }
}
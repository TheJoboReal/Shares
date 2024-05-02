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

    pub fn add_person(&mut self, person: Person){
        self.persons.push(person);
    }

    pub fn update_total_debt(&mut self) {
        let mut debt_temp : f32 = 0.0;
        for element in &self.persons{
            debt_temp += element.debt;
        }
        self.total_debt = debt_temp;
    }

    pub fn update_person_share(&mut self) {
         let mut equal_share : f32  = self.total_debt / self.persons.len() as f32;
         for person in &mut self.persons {
             person.share = equal_share - person.debt;
         }
     }

    pub fn print_persons(&self){

        for element in &self.persons{
            println!("Name: {}", element.name);
            println!("Debt: {}", element.debt);
        }
    }

    pub fn print_persons_share(&self){
        for element in &self.persons{
            println!("Name: {}", element.name);
            println!("Share: {}", element.share);
        }
    }

    
    pub fn what_owed(&self){
        let mut owed: Vec<Person> = Vec::new();
        let mut owes: Vec<Person> = Vec::new();

        // Separate the people who owe money from the people who are owed money
        for person_i in &mut self.persons{
            if person_i.share > 0.0 {
                owed.push(person_i);
            } else if person_i.share < 0.0 {
                owes.push(person_i);
            }
        }

        // Iterate through people who owe money and people who are owed money
        // and print out the transactions
        for person_i in &owed{
            for person_j in &owes{
                if person_i.share.abs() > person_j.share.abs(){
                    println!("{} owes {} {}", person_j.name, person_i.name, person_j.share.abs());
                    person_i.share += person_j.share;
                    person_j.share = 0.0;
                } else if person_i.share.abs() < person_j.share.abs(){
                    println!("{} owes {} {}", person_j.name, person_i.name, person_i.share.abs());
                    person_j.share += person_i.share;
                    person_i.share = 0.0;
                } else {
                    println!("{} owes {} {}", person_j.name, person_i.name, person_i.share.abs());
                    person_i.share = 0.0;
                    person_j.share = 0.0;
                }
            }
        }
    }
}
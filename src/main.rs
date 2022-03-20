use std::collections::HashMap;
use std::io;

fn get_input() -> Option<String> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => Some(input.trim().to_owned()),
        Err(_) => None,
    }
}

#[derive(Debug, Clone)]
pub struct Student {
    name: String,
    age: u8,
}

impl Student {}

pub struct Students {
    class: HashMap<String, Student>,
}

impl Students {
    fn new() -> Self {
        Self {
            class: HashMap::new(),
        }
    }

    fn add_student(&mut self, student: Student) {
        self.class.insert(student.name.clone(), student);
    }

    fn delete(&mut self, name: &str) -> Option<Student> {
        self.class.remove(name)
    }

    fn edit(&mut self, name: &str, age: u8) -> Option<&Student> {
        let student = self.class.get_mut(name)?;
        student.age = age;
        Some(student)
    }

    fn view_all(&self) -> Vec<&Student> {
        self.class.values().collect()
    }
}

mod manager {
    use crate::*;
    pub fn add_student(students: &mut Students) {
        println!("1. Add Student");
        println!("Name of student: ");
        let name = get_input().unwrap();
        println!("Age of student: ");
        let age = get_input().unwrap();
        let parse_age = match age.parse::<u8>() {
            Ok(age) => age,
            Err(_) => {
                println!("Invalid age");
                return;
            }
        };
        let student = Student {
            name: name,
            age: parse_age,
        };
        students.add_student(student);
        println!("Done!");
    }

    pub fn view_all(students: &Students) {
        println!("2. View all students");
        for student in students.view_all() {
            println!("{:?}", student);
        }
    }

    pub fn edit_student(students: &mut Students) {
        println!("3. Edit Student");
        println!("Name of student: ");
        let name = get_input().unwrap();
        println!("Age of student: ");
        let age = get_input().unwrap().parse::<u8>();
        let parse_age = match age {
            Ok(age) => age,
            Err(_) => {
                println!("Age must be a number");
                return;
            }
        };
        match students.edit(&name, parse_age) {
            Some(student) => println!("Edited student: {:?}", student),
            None => println!("Student not found"),
        }
    }

    pub fn delete_student(students: &mut Students) {
        println!("4. Delete Student");
        println!("Name of student: ");
        let name = get_input().unwrap();
        match students.delete(&name) {
            Some(student) => println!("Deleted student: {:?}", student),
            None => println!("Student not found"),
        }
    }
}

enum Manager {
    AddStudent,
    ViewStudent,
    EditStudent,
    DeleteStudent,
}

impl Manager {
    fn show() {
        println!("----- Manager -----");
        println!("1. Add Student");
        println!("2. View Student");
        println!("3. Edit Student");
        println!("4. Delete Student");
        println!("--------------------");
    }
    fn choice(input: &str) -> Option<Manager> {
        match input {
            "1" => Some(Manager::AddStudent),
            "2" => Some(Manager::ViewStudent),
            "3" => Some(Manager::EditStudent),
            "4" => Some(Manager::DeleteStudent),
            _ => None,
        }
    }
}
fn main() {
    let mut students = Students::new();
    Manager::show();
    loop {
        println!("Choose an option: ");
        let input = get_input();
        match input {
            Some(input) => {
                let choice = Manager::choice(&input);
                match choice {
                    Some(choice) => match choice {
                        Manager::AddStudent => manager::add_student(&mut students),
                        Manager::ViewStudent => manager::view_all(&students),
                        Manager::EditStudent => manager::edit_student(&mut students),
                        Manager::DeleteStudent => manager::delete_student(&mut students),
                    },
                    None => return,
                }
            }
            None => println!("Invalid Input"),
        }
    }
}

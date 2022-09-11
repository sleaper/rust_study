use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let mut db: HashMap<String, Vec<String>> = HashMap::new();

    println!("Type 'Add <name> to <department>' to add an employee");
    println!("Type 'List <department>' to list the employees of a department");
    println!("Type 'All' to list all employees by department");
    println!("Type 'Quit' to quit");

    let stdin = io::stdin();
    // TODO: read more about reading input lines
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let mut words = line.split_whitespace();
        let command = words.next().unwrap();
        match command {
            "Add" => {
                let name = words.next().unwrap();
                let to = words.next().unwrap();
                let department = words.next().unwrap();
                if to != "to" {
                    println!("Expected 'to' but got '{}'", to);
                    continue;
                }

                // Get or create here vector with employees
                if db.contains_key(department) {
                    db.get_mut(department).unwrap().push(name.to_string());
                } else {
                    // TODO: Maybe ask here if we want to create ne department
                    db.insert(department.to_string(), vec![name.to_string()]);
                }

                println!("{:?}", db);
                println!("Adding {} to {}", name, department);
            }
            "List" => {
                // FIX: List "" // no text throw error
                let department = words.next().unwrap();
                match db.get(department) {
                    Some(employees) => {
                        println!("Employees of {}: {:?}", department, employees);
                    }
                    None => {
                        println!("No employees in {}", department);
                    }
                }
            }
            "All" => {
                let mut all_employees: Vec<String> = Vec::new();
                for (department, employees) in &db {
                    employees.iter().for_each(|employee| {
                        all_employees.push(format!("{}: {}", department, employee));
                    });
                }
                println!("Listing all employees by department");
            }
            "Quit" => {
                println!("Quitting");
                break;
            }
            _ => {
                println!("Unknown command '{}'", command);
            }
        }
    }
}

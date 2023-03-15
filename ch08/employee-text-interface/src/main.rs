// Using a hash map and vectors, create a text interface to allow a user to add employee names
// to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” 
// Then let the user retrieve a list of all people in a department or all people in the company by 
// department, sorted alphabetically.
use std::io;
use std::collections::HashMap;

fn main() {
    let mut employees = HashMap::new();
    
    employees = add_employee(String::from("Sally"), String::from("Engineering"), employees);
    employees = add_employee(String::from("Amir"), String::from("Sales"), employees);
    employees = add_employee(String::from("GLaDOS"), String::from("Engineering"), employees);

    loop {
        println!("Available commands: add, show, list, exit");
        let command = read_string();

        if command == "exit" {
            break;
        } else if command == "add" {
            println!("Name of employee: ");
            let name = read_string();

            println!("Department of employee: ");
            let department = read_string();

            employees = add_employee(name, department, employees);
            println!("{:?}", employees);
        } else if command == "show" {
            println!("Show employees in this department: ");
            let search_term = read_string();
    
            let in_searched_department = list_employees_in_department(&search_term, &employees);
            println!("{:?}", in_searched_department);
        } else if command == "list" {
            list_all_employees_by_department(&employees);
        }
    }
}

fn add_employee(name: String, department: String, mut employees: HashMap<String, String>) -> HashMap<String, String> {
    employees.insert(name, department);

    employees
}

fn list_employees_in_department(department: &String, employees: &HashMap<String, String>) -> Vec<String> {
    let mut employees_in_department: Vec<String> = Vec::new();
    for (name, dep) in employees {
        if dep == department {
            employees_in_department.push(name.to_string());
        }
    }

    employees_in_department
}

fn list_all_employees_by_department(employees: &HashMap<String, String>) {
    let mut employees_to_vec: Vec<(&String, &String)> = employees.iter().collect();

    employees_to_vec.sort_by(|name, dep| name.1.cmp(&dep.1));
    println!("{:?}", employees_to_vec);
}

fn read_string() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("can not read user input");

    input.trim().to_string()
}

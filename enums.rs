// Unlike C, Enums in Rust can be names, tuple structs or just structs.
// It can be any of the possible variants. Each variants can optionally have data associated with it.
// For example Level below.
// 

enum Employee {
    // Unit like structus
    Engineer,
    Manager,
    // Tuple structures
    Level(i32),
    Salary(i32),
    // or like structures
    Info { name: String, id: i32 }
}

// A function which takes an Employee enum as argument

fn validate(e: Employee) {
    match e {
        Employee::Engineer  => println!("is an engineer."),
        Employee::Manager   => println!("is a manager."),
        Employee::Level(i)  => println!("is at level {}.", i),
        Employee::Salary(i) => println!("has a salary {}.", i),
        Employee::Info { name, id } => {
            println!("{} is an employee here, whose id is {}", name, id);
        },
    }
}

fn main() {
    let ravi = Employee::Engineer;
    let chhaya = Employee::Manager;
    let arpit = Employee::Info {name: "Arpit".to_owned(), id: 123};
    let saloni = Employee::Salary(100000);
    let satya = Employee::Level(100);

    validate(ravi);
    validate(chhaya);
    validate(arpit);
    validate(saloni);
    validate(satya);
}
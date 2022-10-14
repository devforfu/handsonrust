use std::io::stdin;

fn main() {
    println!("What is your name?");
    
    let mut visitors = Visitor::party();
    
    loop {
        let your_name = what_is_your_name();
        let known_visitor = visitors.iter().find(|v| v.name == your_name);

        match known_visitor {
            Some(visitor) => visitor.greet_visitor(),
            None => {
                if your_name.is_empty() {
                    break;
                } else {
                    println!("{} is not on the visitor list.", your_name);
                    visitors.push(Visitor::new(&your_name, VisitorAction::Probation, 0));
                }
            }
        }
    }

    println!("The final list of visitors:");
    println!("{:#?}", visitors);
}

#[derive(Debug)]
struct Visitor {
    name: String,
    action: VisitorAction,
    age: i8,
}

#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNote { note: String },
    Refuse,
    Probation,
}

impl Visitor {
    fn new(name: &str, action: VisitorAction, age: i8) -> Self {
        Self {
            name: name.to_lowercase(),
            action,
            age
        }
    }

    fn greet_visitor(&self) {
        match &self.action {
            VisitorAction::Accept => println!("Welcome to the tree house, {}", self.name),
            VisitorAction::AcceptWithNote { note } => {
                println!("Welcome to the treehouse, {}", self.name);
                println!("{}", note);
                if self.age < 21 {
                    println!("Do not serve alcohol to {}", self.age);
                }
            }
            VisitorAction::Probation => println!("{} is now a probation member", self.name),
            VisitorAction::Refuse => println!("Do not allow {} in!", self.name),
        }
    }

    fn party() -> std::vec::Vec<Visitor> {
        vec![
            Visitor::new("Bert", VisitorAction::Accept, 45),
            Visitor::new("Steve", VisitorAction::AcceptWithNote {
                note: String::from("Lactose-free milk is in the fridge")
            }, 15),
            Visitor::new("Fred", VisitorAction::Refuse, 30),
        ]
    }
}


fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");
    your_name.trim().to_lowercase()
}


use std::io::stdin;

#[derive(Debug)]
struct Visitor {
    name: String,
    action: VisitorAction,
    age: i8,
}
impl Visitor {
    fn new(name: &str, action: VisitorAction, age: i8) -> Self {
        Self {
            name: name.to_lowercase(),
            action,
            age,
        }
    }
    fn greet_visitor(&self) {
        match &self.action {
            VisitorAction::Accept => println!("Welcome to the tree house {}", self.name),
            VisitorAction::AcceptWithNote { note } => {
                println!("Welcome to the tree house {}", self.name);
                println!("{}", note);
                if self.age < 21 {
                    println!("Dont serve alcohol to {}", self.name);
                }
            }
            VisitorAction::Probation => println!("{} is on probation period", self.name),
            VisitorAction::Refuse => println!("Do not allow {} in!", self.name),
        }
    }
}

#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNote { note: String },
    Refuse,
    Probation,
}
fn main() {
    let mut visitor_list = vec![
        Visitor::new("Bert", VisitorAction::Accept, 45),
        Visitor::new(
            "Steve",
            VisitorAction::AcceptWithNote {
                note: String::from("Lactose-free milk is in the fridge"),
            },
            15,
        ),
        Visitor::new("Fred", VisitorAction::Refuse, 30),
    ];
    loop {
        println!("Hello, What is your name?");
        let name = what_your_name();
        let known_visitor = visitor_list.iter().find(|visitor| visitor.name == name);
        match known_visitor {
            Some(data) => data.greet_visitor(),
            None => {
                if name.is_empty() {
                    break;
                } else {
                    visitor_list.push(Visitor::new(name.as_str(), VisitorAction::Probation, 0));
                }
            }
        }
    }
    println!("Visitors List:");
    println!("{:#?}", visitor_list);
}

fn what_your_name() -> String {
    let mut buffer: String = String::new();
    stdin()
        .read_line(&mut buffer)
        .expect("error occurred while fetching user input");
    buffer.trim().to_lowercase()
}

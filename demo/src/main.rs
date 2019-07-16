#[derive(Debug, Clone)]
struct User { username: String }

#[derive(Debug)]
struct CoffeeMachine {
    water_tank: u32,
    is_on: bool,
    owner: &User
}

impl CoffeeMachine {
    pub fn new(owner: &User) -> CoffeeMachine {
        CoffeeMachine { owner, water_tank: 0, is_on: false }
    }
}

fn main() {
let user = User { username: "Georges".to_owned() };
let coffee_machine = CoffeeMachine::new(&user) ;
let coffee_machine2 = CoffeeMachine::new(&user) ;

println!("{:?}", coffee_machine)
}

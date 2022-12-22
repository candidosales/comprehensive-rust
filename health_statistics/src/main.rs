struct User {
    name: String,
    age: u32,
    weight: f32,
}

impl User {
    fn new(name: String, age: u32, weight: f32) -> Self {
        Self {
            name: String::from(name),
            age,
            weight,
        }
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn age(&self) -> u32 {
        self.age
    }

    fn weight(&self) -> f32 {
        self.weight
    }

    fn set_age(&mut self, new_age: u32) {
        self.age = new_age
    }

    fn set_weight(&mut self, new_weight: f32) {
        self.weight = new_weight
    }
}

fn main() {
    let bob = User::new(String::from("Bob"), 32, 155.2);
    println!("I'm {} and my age is {}", bob.name(), bob.age());
}

#[test]
fn test_weight() {
    let bob = User::new(String::from("Bob"), 32, 155.2);
    assert_eq!(bob.weight(), 155.2);
}

#[test]
fn test_set_age() {
    let mut bob = User::new(String::from("Bob"), 32, 155.2);
    assert_eq!(bob.age(), 32);
    bob.set_age(33);
    assert_eq!(bob.age(), 33);
}

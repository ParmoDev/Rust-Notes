// Conditionals - Used to check the condition of something and act on it

pub fn run() {
    let age: u8 = 22;
    let check_id: bool = false;
    let knows_person_of_age: bool = true;

    // If/Else   And          or
    if age >= 21  && check_id || knows_person_of_age {
        println!("You are old enough to drink in america!")
    } else if age < 21 && check_id {
        println!("You are not old enough to drink in america.")
    } else {
        println!("I'll need to see your ID")
    }

    // Shorthand If
    let is_of_age = if age >= 21 { true } else { false };
    println!("Is Of Age: {}", is_of_age)
}
struct City {
    description: String,
    // residents: u64,
    is_coastal: bool,
    // 👉 TODO add a field here for is_coastal: bool
    //
    // 💡 HINT: this will cause other compiler errors.
    //    They will tell you what other changes need to happen!
}

fn new_city(residents: u64, is_coastal: bool) -> City {
    let constal_string: &str = if is_coastal { "*costal*" } else { "*non-coastal*" };

    City {
        description: format!("a {} city of approximately {} residents", constal_string, residents),
        // residents,
        is_coastal,
    }

} 

fn main() {
    let rustville: City = new_city(100000, false);

    println!("This city can be described as: {}", rustville.description);

    if rustville.is_coastal {
        println!("It is a coastal city.");
    } else {
        println!("It is not a coastal city.");
    }
}

fn main() {
//class 121
let mut current_meal = String::new();

add_flour(&mut current_meal);
show_my_meal(&current_meal);

}
//meal: String
//mut meal : String
//meal: &String
//meal: &mut String
fn add_flour(meal: &mut String) {
    meal.push_str("Add flour");
}

fn show_my_meal(meal: &String) {
    println!("Meal steps: {}", meal);
}



/*
    //Class 118 Project
    let is_concert = true;
    let is_event = is_concert;
    println!("{} , {}", is_concert, is_event);

    let suchi = "Salmon";
    let dinner = suchi;
    println!("{} , {}", suchi, dinner);

    let sushi = String::from("Salmon");
    let dinner = sushi;
    //println!("{} , {}", sushi, dinner);

    // Atribui o valor de dinner a sushi, mas sushi não é mais válido
    let fish = eat_meal(dinner);
    println!("{},", fish);

    // Class 118 Project fora do main
    fn eat_meal(mut meal: String) -> String {
        meal.clear(); // Limpa o valor de meal
        meal
    }

    //Class 115
    let cake = bake_cake();
    println!(" I now have a {}.", cake);
}
fn bake_cake() -> String {
    String::from("Chocolate Mouse")
}

*/

/*
   // Class 113
    let burger = String::from("Cheeseburger");
    add_fries(burger);
}
    //Class 113 Fora do main
    fn add_fries(mut meal: String) { 
        meal.push_str(" and fries");
        println!("{}", meal);
    }
*/

/*  Class 112
    let ice_cream = "Cookies and Cream";
    let dessert = ice_cream;
    println!("{}, {}", ice_cream, dessert);
*/

    /* Class 110
    let my_stack_value = 2;
    let my_integer_reference = &my_stack_value; // my_integer_reference is a reference to my_stack_value
    // my_integer_reference is a reference to my_stack_value (*) is optional
    println!("{}", *my_integer_reference);

    let my_heap_value = String::from("Sandro");
    let my_heap_reference = &my_heap_value;
    //Rust cacth the value automatically for all references
    println!("{}", my_heap_reference);
     */
/*  Class 109
    let my_stack_value = 2;
    let my_integer_reference = &my_stack_value; // my_integer_reference is a reference to my_stack_value

    let my_heap_value = String::from("Sandro");
    let my_heap_reference = &my_heap_value;
*/

/*
Class 108
let person = String::from("Sandro");
let genius = person.clone(); // person is copied to genius, both are valid.
print!("This is {}", person);
 */

/* Class 107
    let person = String::from("Sandro");

    drop(person);// person now is invalid, deleted from memory
*/
/*
    //Class 106
    let person = String::from("Sandro");
    //person is moved to genius, person is no longer valid.
    let genius = person;
*/

/*
//Class 105
let mut name = String::from("Sandro");
println!("Name: {}", name);
name.push_str(" Oliveira Reis");
println!("Name: {}", name);
 */
/*
//Class 104
    let text = String::new();
    let candy = String::from("Kit Kat");
*/

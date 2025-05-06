fn main() {
    // Class 109
    let my_stack_value = 2;
    let _my_integer_reference = &my_stack_value; // my_integer_reference is a reference to my_stack_value

    let my_heap_value = String::from("Sandro");
    let _my_heap_reference = &my_heap_value;
}

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

fn main() {
    //Class 107
    let person = String::from("Sandro");
    
    drop(person);// person now is invalid, deleted from memory
}
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

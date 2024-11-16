use serde::{Deserialize, Serialize};
use serde_json::{Result, Value, json};
use std::path::PathBuf;
use std::env;
use std::fs::File;
use std::io::{Read, Write};
mod structure;
use structure::PersonData;
use rand::Rng;

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u8,
    taskdone: bool,
}

fn get_current_working_dir() -> std::io::Result<PathBuf> {
    env::current_dir()
}

pub fn get_person_details(data: String) -> (String, u8, bool) {
    // Deserialize the JSON data into a Person struct
    let person: Person = serde_json::from_str(&data).unwrap();

    
    // Print the deserialized Person struct for debugging
    println!("Deserialized Person data: {:?}", person);

    // Return the fields as a tuple
    (person.name, person.age, person.taskdone)
}


pub fn print_sample_json() -> Result<()> {
    println!("{:?}",get_current_working_dir());

    let mut file: File = File::open("src/data.json").expect("File not found");
    let mut json_data = String::new();
    file.read_to_string(&mut json_data).expect("Failed to read the file");

    // Step 2: Parse the JSON data into the `Person` struct
    let mut person:Value = serde_json::from_str(&json_data)?;

    // Step 3: Print all values in the JSON data
    println!("Name: {}", person.get("name").and_then(Value::as_str).unwrap_or("Unknown"));
    // println!("Age: {}", person.age);
    // println!("Active: {}", person.isActive);
    // println!("Address:");
    // println!("  Street: {}", person.address.street);
    // println!("  City: {}", person.address.city);
    // println!("  State: {}", person.address.state);
    // println!("  Postal Code: {}", person.address.postalCode);

    // for phone in &person.phoneNumbers {
    //     println!("Phone [{}]: {}", phone.r#type, phone.number);
    // }

    // // // Step 4: Update the age to a random value between 1 and 50
    // let mut rng = rand::thread_rng();
    // person.age = rng.gen_range(1..=50); 
    // println!("Updated Age: {}", person.age);

    // Step 5: Serialize the updated `Person` struct back to JSON
    let updated_json = serde_json::to_string_pretty(&person)?; // Use `to_string_pretty` for formatted output

    // Step 6: Write the updated JSON back to the file
    let mut file = File::create("src/data.json").expect("Failed to create file");
    file.write_all(updated_json.as_bytes()).expect("Failed to write to file");

    Ok(())

}

pub fn main() {
    // Create a Person struct and serialize it to JSON
    let person = Person {
        name: "John".to_string(),
        age: 24,
        taskdone: false,
    };

    let serialized_person_data = serde_json::to_string(&person).unwrap();
    println!("Serialized Person data: {}", serialized_person_data);

    // Get the details from the deserialized JSON
    let (name, age, taskdone) = get_person_details(serialized_person_data);

    // Print the returned values
    println!("Name: {}, Age: {}, Task Done: {}", name, age, taskdone);
    println!("Sample Data fn Called =========================");

    match print_sample_json() {
        Ok(_) => println!("JSON data updated successfully."),
        Err(err) => eprintln!("Error updating JSON data: {}", err),
    };



let data1 = r#"
    {
        "name": "Alice",
        "age": 30,
        "is_active": true
    }
"#;

let data2: Value = json!({
    "name": "Alice",
    "age": 30,
    "is_active": true
});

println!("data1: {}", data1);

let name = data2.get("name").and_then(Value::as_str).unwrap_or("Unknown");
let age = data2.get("age").and_then(Value::as_i64).unwrap_or(0);
let is_active = data2.get("is_active").and_then(Value::as_bool).unwrap_or(false);

println!("Name: {}", name);
println!("Age: {}", age);
println!("Is Active: {}", is_active);

}

use std::{
    collections::{HashMap, HashSet},
    thread::current,
};
fn main() {
    test_hashmap();
    test_hashset();
}

fn test_hashmap() {
    let mut stock_list: HashMap<String, f32> = HashMap::new();
    println!("{}", stock_list.len());

    // is_empty() method to check whether the hashmap is empty or not.
    println!("{:?}", stock_list.is_empty());

    //Inserting elements into the hashmap
    stock_list.insert("Nvidia".to_string(), 1.00);
    stock_list.insert("Google".to_string(), 12.00);
    stock_list.insert("Microsoft".to_string(), 120.00);

    // This hash sign (#) inside the print statement helps to do the preety print like proper formatting and all.
    println!("{:#?}", stock_list);

    // to remove the element from the hashmap
    stock_list.remove(&("Nvidia".to_string()));
    println!("{:#?}", stock_list);

    // We can call insert again to overrite the value of the particular key.
    stock_list.insert("Nvidia".to_string(), 19.00);

    // We can use this syntax or way to not override the existing values if the key already exists in the hashmap.
    stock_list.entry("Meta".to_string()).or_insert(120.00);

    // Iterating over the hashmap
    for (ticker, current_value) in stock_list {
        println!("{} is trading at {}", ticker, current_value);
    }

    // drain method is used to clear the hashmap and return all the key value pairs as an iterator and keeps the allocated memory for the reuse. Clear will also clear the hashmap and keeps the memory reserved but it won't return anything.
}

fn test_hashset() {
    let mut planet_list1 = HashSet::from(["Mercury", "Venus", "Earth", "Mars"]);

    for planet in &planet_list1 {
        println!("Planet :-> {}", planet);
    }

    let mut planet_list2 = HashSet::from(["Mars", "Jupiter", "Saturn", "Uranus", "Neptune"]);

    // Now I want to merge the both planet lists i.e planet_list1 and planet_list2

    let diff_planet = planet_list1.difference(&planet_list2);
    for planet in diff_planet {
        println!("Unique planets : {}", planet);
    }

    let planet_symmetric_difference = planet_list1.symmetric_difference(&planet_list2);
    for planet in planet_symmetric_difference {
        println!("Symmetric difference's planet is {}", planet);
    }

    // Inserting items in the hashset
    planet_list1.insert("Saturn");
    planet_list1.insert("Uranus");
    planet_list1.insert("Neptune");

    for planet in planet_list1{
        println!("Planets from the planet_list1 is : {}",planet);
    }
    

}

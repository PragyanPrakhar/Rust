fn main() {
    test_iterators();
}

fn test_iterators() {
    let fav_foods = vec!["Paneer", "Chicken", "Biryani", "Egg", "Milk", "Mutton"];

    let dry_fruits = vec!["Almonds", "Cashewnuts", "Walnut", "Anjeer"];

    // We are having the reference to the iterator of the vector named fav_foods.
    let mut foods_iter = fav_foods.iter();

    for food in foods_iter.clone() {
        println!("{}", food);
    }

    // If we will do foods_iter.next() then it will just skip the first element and move to the second element.And we can do it any number of times then we can skip as many items we want according to our need by calling it any number of times.
    let item01 = foods_iter.next(); // It is used to return the first item from the iterator.
    println!("First item in the iterator is: {}", item01.unwrap());

    // Chaining to iterate simulatenously.
    let aggregate_foods = fav_foods.iter().chain(&dry_fruits);

    for food in aggregate_foods.clone() {
        println!("Food in Aggregate food is {}", food);
    }

    // Taking an iterator and turning back into the collection type

    let all_foods: Vec<&&str> = aggregate_foods.collect();
    println!("Collected collection is {:?}", all_foods);
    // Then again we can get a iterator from the collected collection type.

    let fav_foods_string = fav_foods.iter().map(|e| String::from(*e)); // NOTE :-> Dereferencing has been done here.

    // Mapping over each food and adding food in the last of the each food name.
    let new_fruits = fav_foods_string.map(|mut e| {
        e.push_str("food");
        return e;
    });

    new_fruits.clone().for_each(|e| println!("{:?}", e));

    // to get the last element from the iterator
    println!("last element :-> {:?}", new_fruits.clone().last().unwrap());

    // Step by is used to step over the number of items
    let stepby = new_fruits.clone().step_by(2);

    println!("After step-by by 2 {:?}", stepby);

    // Using Zip method
    let first_names = vec!["Pragyan", "Rishah", "Shreya", "Rajesh"];

    let first_names_string = first_names.iter().map(|e| e.to_string());

    let last_names = vec!["Prakhar", "Sinha", "Sinha", "Sinha"];
    let last_names_string = last_names.iter().map(|e| e.to_string());

    let full_names = first_names_string.zip(last_names_string);
    for (first, last) in full_names.clone() {
        println!("{} {}", first, last);
    }

    // Enumerate method
    for (index, value) in full_names.clone().enumerate() {
        println!("Index: {:?} and value: {:?}", index, value);
    }

    // It will skip the first item , like if we are iterating over the rows and we want to skip the header row then in that case we can use this method.
    full_names
        .skip(1)
        .for_each(|e| println!("Didn't skip : {}", e.0));

    let foods = vec![("Potatoes", 10), ("Strwaberries", 25), ("Burger", 16)];

    // Fold function like it is same as reduce in the python but it is bit different syntax wise.
    let food_quantity = foods.clone().iter().fold(0u32, |a, e| a + e.1);

    println!("Your total Food Quantity is :-> {:?}", food_quantity);
}

fn main(){
    test_match_int();
    test_match_string();
    test_match_array();
    test_match_enum();
}

enum Vehicle {
    Car(String),
    Bike(String),
    Truck(u32),
}

fn test_match_enum() {
    let my_vehicle = Vehicle::Car("Porsche".to_string());

    match my_vehicle {
        Vehicle::Car(name) => println!("You have a car: {}", name),
        Vehicle::Bike(name) => println!("You have a bike: {}", name),
        Vehicle::Truck(weight) => println!("Truck with capacity: {} kg", weight),
    }
}


fn test_match_array(){
    let prices : [u32;3]=[50000,343000,42000];
    match prices[0..=1]{
        [50000,343000] => println!("You have reasonable priced cars"),
        _=>println!("You don't have reasonalbly priced cars")
    }


}

fn test_match_string(){
    let car_manufacturer : &str = "Porsche";
    match car_manufacturer{
        "Hyundai"=> println!("Yes yes !! You have matched correctly , This is Hyundai only."),
        "Porsche"=>println!("Processing Porsche Vehicle , da da da ...."),
        _=> println!("This is the fallback, When nothing matches this is returned.")
    }
}

fn test_match_int(){
    let myage=22;
    let y=5;
    match myage{
        // We can also write the statement without enclosing them inside curly braces if there is only one statement which is to be executed. 
        1..=22 if y==5  =>{
            println!("The pattern is matched and the age is {0} with y =  5",myage);
        }
        1..=22 if y!=5 =>{
            println!("The pattern is matched and the age is between 1 to 22 with y not equal to 5")
        }
        1..=22 =>{
            println!("The pattern is matched")
        }
        // 23.. This will cover all the numbers from 23 to the limit of signed 32 bit integer.
        /* 25 | 26 =>{
            println!("Using pipe character to match the values which is of either 25 or 26")
        } */
        23.. =>{
            println!("You have now achieved whatever you have wanted !!")
        }
        //This is the default fallback , like no any pattern is matched then this will be executed.
        _ =>{
            println!("This is the default which is returned when nothing matches otherwise rust will give error.")
        }
    }
}
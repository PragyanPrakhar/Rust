use std::cell::Cell;
fn main() {
    let mut my_person = new_person();
    my_person.walk_metres(20);
    my_person.walk_metres(30);

    println!(
        "First name : {0} , last name : {1} , birth month :{2} , birth year :{3} and visited europe : {4} and metres walked is {5}",
        my_person.first_name.get(), my_person.last_name, my_person.birth_month, my_person.birth_year , my_person.visited_europe ,
        my_person.metres_walked
        
    );

    // let my_vehicle=new_vehicle();
    // Static Methods
    let mut my_vehicle:Vehicle=Vehicle :: create_vehicle();
    my_vehicle.paint(VehicleColor::Blue);
    println!("Vehicle Manufacturer is {:?} , model is {:?} , Year is {:?} and the color of the vehicle is {:?}",my_vehicle.manufacturer,my_vehicle.model,my_vehicle.year,my_vehicle.color);

    let my_tuple_vehicle=new_vehicletuple();
    println!("Manufacturer is {0} , Model is {1} and year is {2}",my_tuple_vehicle.0,my_tuple_vehicle.1,my_tuple_vehicle.2);
}

#[allow(dead_code)]
// struct by default is immutable , If we will use mut like let mut p1 then it will allow it to be mutable , but all the fields of the struct will be mutable. Therefore to make particular type as mutable we have to use special type of the struct i.e Cell
struct Person<'p> {
    //Import the cell.
    // With the 'p we have declare the lifetime.
    first_name: Cell<&'p str>,  // Making the first name as mutable only
    last_name: String,
    birth_year: u16,
    birth_month: u8,
    visited_europe:bool,
    metres_walked:u32
}

#[derive(Debug)]
#[allow(dead_code)]

enum VehicleColor {
    Silver,
    Blue,
    Red,
    Black,
    White,
    Green
}

#[derive(Debug)]
struct Vehicle{
    manufacturer:String,
    model:String,
    year:u16,
    color:VehicleColor,
}

// Methods on for the struct Vehicle.
// Static methods is something that is implemented on the struct itself.
impl Vehicle{
    // Method to change the color of the vehicle.
    fn paint(&mut self,new_color:VehicleColor){
        self.color = new_color;
    }

    // We are associating this function to the Vehicle Only.
    fn create_vehicle()->Vehicle{
        let new_vehicle:Vehicle=Vehicle{manufacturer:"default".to_string(),model:"default".to_string(),year:1990,color:VehicleColor::Red};
        return new_vehicle;
    }
}

impl Person<'static>{
    fn walk_metres(&mut self , metres:u32){
        self.metres_walked+=metres;
    }
}


#[derive(Debug)]
struct VehicleTuple(String,String,u16);


fn new_vehicletuple()->VehicleTuple{
    return VehicleTuple("Mercedes".to_string(),"Mercedes-benz".to_string(),2026);
}
  
fn new_vehicle()->Vehicle{
    let mut v1=Vehicle{
        manufacturer:"Mercedes".to_string(),
        model:"merceded-benz".to_string(),
        year:2027,
        color:VehicleColor::Silver,
    };

    v1.paint(VehicleColor::Red);
    return v1;
}



fn new_person() -> Person<'static> {
    let p1 = Person {
        // We have used cell because we have used cell<string> as the datatype for this above as we want only this to be mutable.
        first_name: Cell::from("Pragyan"),   
        last_name: "Prakhar".to_string(),
        birth_year: 2003,
        birth_month: 10,
        visited_europe:false,
        metres_walked:200
    };
    p1.first_name.set("Rishabh");


    return p1;
}


// All functions defind in an impl block are called associated functions because they're associated with the type named after the impl.

fn main() {
    create_person();
}

// Traits are like objects in which certain methods are defined without body , We can compare it with interfaces in java.

struct Person<PetType:Animal,PetType2: Animal + Dangerous> { // We can replace colon with where
    first_name: String,
    pet: PetType,
    pet2:PetType2
}

trait Animal {
    fn make_sound(&self)->();  //trait
}

trait NotDangerous{}
trait Dangerous{}


struct Dog {}
impl NotDangerous for Dog{}
impl Animal for Dog{
    fn make_sound(&self)->() {
        println!("Dog is barking !!!!!");
    }
} // impl trait with type , which associates the dog with the trait Animal.

#[allow(dead_code)]
struct Cat {}
impl NotDangerous for Cat{}

impl Animal for Cat{
    fn make_sound(&self)->() {
        println!("Cat is meowing !!!!!");
    }
}

#[allow(dead_code)]
struct Bear {}
impl Dangerous for Bear{}
impl Animal for Bear{
    fn make_sound(&self)->() {
        println!("Bear is roaring !!!!!");
    }
}

#[allow(dead_code)]

struct Tiger {}

pub fn create_person() {
    let pet1=Dog{};
    let pet2=Cat{};
    let pet3=Bear{};
    let p1=Person{first_name:"Pragyan".to_string(),pet:pet1,pet2:pet3};
}

impl Dog {
    fn bark(&self) {
        println!("Bark !!!!");
    }
}

struct Character {
    hit_points: u16,
}

fn main(){
    test_closures();    
}

struct Person{
    first_name:String,
    last_name:String
}

fn test_closures(){
    // Simplest Example of Closures
    let add=  |x:i8,y:i8|{
        println!("Returning some added texts {0}",x+y);
        x+y
    }; 
    // Whenever we see the pipe (||) and it is not used for any logical comparison then there is a good chance that it is a closure.
    // This is how a closure is defined(i.e using ||).
    let subtract  = |x:i32,y:i32|{
        println!("The values which are to be subtracted is {0} and {1}",x , y);
        x-y
    };

    subtract(25,16);
    add(-3,-5);
    // We don't have to specify the data type of the input parameter in the closure.

    let mut p1=Person{first_name:"Pragyan".to_string(),last_name:"Prakhar".to_string()};
    let mut change_name = || p1.last_name="Sinha".to_string();
    change_name();
    println!("{}",p1.last_name);
}
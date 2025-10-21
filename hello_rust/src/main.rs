use std::io;

pub mod helpers;
fn main() {
    println!("Hello, world!");
    data_types();
    type_coercion();
    statement_and_expression();
    conditional_expression();
    while_loop();
    test_for();
    test_loop();
}

fn data_types(){
    let x:()=();
    println!("{:?}",x) ; // Unit Type -> Printing the type of the x.

    let signed_integer:i8=-5;
    let unsigned_integer:u8=215; 
    let floting_point_value:f32=232.5;
    let i_am_genius=true;
    println!("{:?}",floting_point_value);
    println!("{:?}",signed_integer);
    println!("{:?}",unsigned_integer); 
    println!("Yes It is {res} that I am genius",res=i_am_genius);

    println!("Calling Child 2");
    // sub_helper::child2();

    // let x:i8=32;
    // x=21; Variables are immutable by default in rust.It will give errir.

    // If we want to make any variable mutable then we have to use mut explicitly.
    // let mut y:u8=200;
    // y=21;
    // println!("Changed value of y is {:?}",y);

    let first_name: &'static str="Pragyan";
    println!("First name is {}",first_name);

    let name : (&str,&str,u8)=("Pragyan","Prakhar",22 as u8);
    println!("{:?}",name); // If we will try to print without :? then it will give the error because the formatter don't know how to print the output so applying :? will use the debugging type output instead of the default formatter for printing the data.

    let ages:[i8;3]=[22,23,24]; // Array Data Type
    println!("{:?}",ages);

    // Slice Data Type
    let sub_ages:&[i8]=&ages[0..2]; // So by default the last index of the slice is excluded by default , but if we want to include that then we can simply use the =sign before that like this "&ages[0..=2]". 
    println!("{:?}",sub_ages);

    // Utilizing the functions and Modules 
    // If we will not use this function then rust will giver error
    #[allow(dead_code)]
    let result=helpers::wrapper::get_full_name("Pragyan", "Prakhar");
    println!("Hello from {res}",res=result)

}

fn type_coercion(){
    let x:f64=213.4;
    let y:i8=x as i8 - 123;
    println!("Subtracted Number is {num}",num=y);
}

// Important Concept -> Differentiating between statement and expression

fn statement_and_expression(){
    let y={
        let x=3;
        x+1
    };
    println!("The value of y is {0}",y);

}

// Control flow And Conditonal Expressions

// fn conditional_expression(){
//     let age_to_drive: u8=16u8;
//     println!("Enter the age of the users :-> ");
//     let mut myinput= String::new();
//     std::io::stdin().read_line(&mut myinput).unwrap();
//     // #[allow(dead_code)]
//     let age=myinput.parse::<u8>().unwrap();
//     if age >= age_to_drive{
//         println!("Issuing driver's license because the driver is old enough !!");
//     }
//     else{
//         print!("The age is not above 18");
//     }

// }

fn conditional_expression() {
    let age_to_drive: u8 = 16u8;
    println!("Enter the age of the user :-> ");

    let mut myinput = String::new(); // ✅ create a mutable String
    std::io::stdin().read_line(&mut myinput).unwrap();

    // ✅ Trim newline before parsing
    let age: u8 = myinput.trim().parse().unwrap_or(0);

    if age >= age_to_drive {
        println!("Issuing driver's license because the driver is old enough!");
    }
    else if age==16 || age>14{
        println!("You are just on the verge of being the old enough ! Wait one year longer.")
    } 
    else {
        println!("The age is not above 16.");
    }

    let driver_license=if age>=16{true} else {false};
    println!("Driver license is {0}",driver_license);
}
fn while_loop(){
    let age_to_drive=16u8;
    let mut current_age=0u8;
    while current_age < age_to_drive{
        println!("Waiting");
        current_age+=1;
    }
}

fn test_loop(){
    let mut x=0u8;
    loop {
        println!("Hello from Pragyan !")
        if x > 5{
            break;
        }
        x+=1;
    }
}

fn test_for(){
    let ages:[i32;5]=[14,15,16,17,18];
    let age_to_drive=16i32;
    for age in ages{
        if age >= age_to_drive{
            println!("The current is {0} is eligible for driving",{age});
        }
        else{
            println!("You need to wait a bit for the driving.");
        }
    }
}


fn main(){
    test_for();
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
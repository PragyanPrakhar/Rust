fn main() {
    let result = test_option();
    println!("Result is {:?}", result.unwrap());

    let string_result=test_option_string();
    println!("Result is {:?}",string_result.unwrap());

    let  character_type_result=test_option_character_type();
    
    if character_type_result.is_some(){
        println!("User has selected some character type.");
    }
    else{
        println!("User has not selected any character type yet.")
    }
    println!("Character type is {:?}",character_type_result.unwrap().to_string());
}

enum CharacterType{
    Archer,
    Warrior,
    Mage,
    Legend
}

//Rust doesn't know to take a input and convert them into them into the string.
impl ToString for CharacterType{
    fn to_string(&self)->String{
        match self{
            CharacterType::Archer=>"Archer",
            CharacterType::Warrior=>"Warrior",
            CharacterType::Mage=>"Mage",
            CharacterType::Legend=>"Legend"
        }.to_string()
    }
}

fn test_option_character_type()->Option<CharacterType>{
    let mut chartype:Option<CharacterType>=None;
    chartype=Some(CharacterType::Legend);
    return chartype;
}  

fn test_option() -> Option<u8> {
    let mut opt1: Option<u8> = None;
    opt1=Some(10);  // Some results Option type
    return opt1;
}

fn test_option_string()->Option<String>{
    let mut opt1 : Option<String>=None;
    opt1=Some("Pragyan Prakhar".to_string());
    return opt1;
}

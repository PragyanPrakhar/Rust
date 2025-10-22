fn main(){
    let battery_percentage=battery_checker();
    if battery_percentage.is_some(){
        println!("Batteryu percentage is avilable , below is the battery percentage !");
        println!("Battery percentage is {:?}",battery_percentage.unwrap())
    }
    else{
        println!("Battery Percentage is not available !! ")
    }
}

fn battery_checker()->Option<u8>{
    let mut battery_percent=None;
    battery_percent=Some(97);
    return battery_percent;
}
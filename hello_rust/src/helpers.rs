// pub mod sub_helper;
pub mod wrapper{
    // use super::sub_helper::subhelper;
    // Functions and Modules in Rust
    pub fn get_full_name(first:&str , last:&str)->String{
        let full_name=format!("{0} {1}",first,last);
            // subhelper::child2() ;
        return full_name;
}
}

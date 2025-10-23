    fn main(){
        test_vec_int();
        test_vec_string();
        test_vec_car();
    }

    pub fn test_vec_int(){
        let mut my_ints:Vec<i32>=Vec::new();

        // Pushing elements into the vector.
        my_ints.push(30);
        my_ints.push(-32);
        my_ints.push(332);
        my_ints.push(-42);
        my_ints.push(-342);

        println!("{:?}",my_ints);

        // finding the size and capacity of the vector
        println!("The length of the vector is {:?}",my_ints.len());
        println!("The capacity of the vector is {:?}",my_ints.capacity());

        // finding the slice of the vector
        println!("Sliced items in vec is : {:?}",&(&my_ints).as_slice()[0..5]);
        // [3..] [..3] [0..=3]

        // getting an element from the particular index
        println!("Getting an element from the particular index {:?}",my_ints.get(4));
    }

    fn test_vec_string(){
        let first_names=vec!["Pragyan","Sanskar","Vidya","Shivam","Vaibhav","Sakshi"];
        for first_name in first_names.as_slice(){
            println!("Processing : {}",first_name);
        }

        // This code will give error because we have already consumed the value of the vector above in the for loop.
        // The one solution to this problem is that we can use .as_slice() method in the above for loop.
        println!("Getting the element is : {:?}",first_names);

    }

    #[derive(Debug)]
    struct Car{
        manufacturer:String,
        model:String,
    }

    fn test_vec_car(){
        // Here if we will try to do this (like repeating the car 10 times) then it will give error because the copy method for the struct is not defuined yet.
        // let car_list=vec![Car{manufacturer:"Porsche".to_string() , model:"Panamera".to_string()};10];
        let name_list=vec!["Pragyan Prakhar";10];
        println!("{:?}",name_list);

        let mut car_list:Vec<Car>=vec![];
        let mut car_lot2:Vec<Car>=vec![];

        for _  in 1..=100u8{
            car_list.push(Car{manufacturer:"Porsche".to_string(),model:"Panarema".to_string()});
        }

        for _  in 1..=100u8{
            car_lot2.push(Car{manufacturer:"Porsche".to_string(),model:"Sonata".to_string()});
        }
        println!("The car list is {:?}",car_list);
        println!("The size of the car is {:?}",car_list.len());
        println!("The size of the car lot 2 is {:?}",car_lot2.len());
        println!("The capacity of the car is {:?}",car_list.capacity());
        println!("The capacity of the car lot 2 is {:?}",car_lot2.capacity());

        //Append method appends all the items of the other into the self leaving otehr as empty.
        car_list.append(&mut car_lot2);
        println!("The size of the car lot 2 is {:?}",car_lot2.len());
        println!("The capacity of the car lot 2 is {:?}",car_lot2.capacity());

        // insert method
        car_list.insert(0,Car{manufacturer:"Lamborghini".to_string(),model:"Aventador".to_string()});

        // Remove item
        car_list.remove(0);

        // Retain method
        // This function makes closure and tells retain every car whose manufacturer is porsche.
        car_list.retain(|e:&Car|{if e.manufacturer=="Porsche" {return true;} else {return false;}});
        println!("{:?}",car_list);

        // There is reserve method 
        // It is used to reseve the extra memory.
        car_list.reserve(5000);
    }


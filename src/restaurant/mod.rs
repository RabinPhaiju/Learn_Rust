// Crates: Modules that produce a library or executable
// Modules : Organize and handle privacy
// Packages : Build, test and share Crates
// Paths : A way of naming an item such as a struct, function

mod pizza_order{
    pub struct Pizza{ // pub -> public
        pub dough: String,
        pub cheese:String,
        pub topping:String
    }

    impl Pizza{
        pub fn lunch(topping:&str)->Pizza{
            Pizza {
                dough:String::from("regular dough"),
                cheese: String::from("Mozzarella"),
                topping:String::from(topping),
            }
        }
    }
    pub mod help_customer{
        fn seat_at_table(){
            println!("Customer seated at table");
        }
        fn serve_customer(cust_pizza:super::Pizza){
            println!("The customer is server regular pizza with {}",cust_pizza.topping)
        }
        pub fn take_order(){
            seat_at_table();
            let cust_pizza :super::Pizza = super::Pizza::lunch("veggies");
            serve_customer(cust_pizza)
        }
    }
}

pub fn order_food(){
    crate::restaurant::pizza_order::help_customer::take_order()
}
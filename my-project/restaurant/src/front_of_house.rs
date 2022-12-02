pub mod hosting {
    pub fn add_to_waitlist() {
        println!("added to waitlist");
    }
    pub fn seat_at_table() {
        println!("Client seated");
    }
}

pub mod serving {
    pub fn take_order() {
        println!("Oder taken");
    }
    pub fn serve_order() {
        println!("Order served");
    }
    pub fn take_payment() {
        println!("Payment taken");
    }
}

pub trait Summary {
    // Method with default Behavior
    fn summarize(&self) -> String {
        String::from("Read more ....")
    }
    // Method without default behavior
    fn summarize_author(&self) -> String;
}
use std::collections::hash_map::HashMap;
pub fn add_to_map(my_map : &mut HashMap<i32, String>, k: i32, val: String) {
    let _ = my_map.insert(k,val);
}

pub fn get_from_map (the_map: &mut HashMap<i32, String>, k: i32) -> Option<&String> {
    the_map.get(&k)
}

pub struct Memory_map {
    internal_map : HashMap<i32, String>
}

impl Memory_map {

    pub fn new() -> Self {
        Memory_map{ internal_map : HashMap::new() }
    }

    pub fn add_record(&mut self, k: i32, v:String) {
        self.internal_map.insert(k,v);
    }

    pub fn get_value<'a>(&'a self, k:i32) -> Option<&'a String> {
        self.internal_map.get(&k)
    }
}

impl Iterator for Memory_map {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}


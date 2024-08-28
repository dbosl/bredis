use std::collections::HashMap;

#[derive(Debug)]
pub struct Store {
    pub ttl: u32,
    pub size: u32,
    pub data: HashMap<String, u32>,
    pub eviction_policy: String
}

impl Store {
    pub fn new(ttl: u32, size: u32)-> Self {
        let data = HashMap::new();
        Self { ttl, size, data, eviction_policy: "lru".to_string() } 
    }

    pub fn get_value(self, k: &str) -> u32 {
       *self.data.get(k).unwrap() 
    }

    pub fn cache(&mut self, k: String, v: u32) {
        println!("{}, succesfully cached", k);
        let _ = self.data.insert(k, v);
    }

    pub fn cached(self, k: &str) -> bool {
        self.data.contains_key(k)
    }
    
    pub fn remove_from_store(&mut self, k: &str) {
       if self.data.contains_key(k)  {
           let _ = self.data.remove(k);
       } else {
           println!("{k} is not in store")
       }
    }
}

fn main() {
    let mut store = Store::new(100, 200);
    store.cache("nana".to_string(), 33);
    println!("{:?}", store);
}

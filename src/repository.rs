use std::rc::Rc;

use nannou::rand::random_range;
use redis::Commands;

pub struct RedisMessageRepository {
    pub client: Rc<redis::Client>,
}


impl RedisMessageRepository {
    pub fn get_random(&self) -> String {
        let mut con = self.client.get_connection().unwrap();
        let keys: Vec<String> = con.keys("*").unwrap();
        let random_index = random_range(0, keys.len());
        let key = keys.get(random_index).unwrap();
        let message: String = con.get(key).unwrap();
        message
    }
}



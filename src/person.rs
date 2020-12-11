use rocket_contrib::json::{Json, JsonValue};
use mytable::{TableTrait, Varchar};


#[derive(Debug, Copy, Clone)]
pub struct Person {
    pub id: usize,
    pub name: Varchar<20>,
    pub age: u32,
}


impl TableTrait for Person {
    fn id(&self) -> usize {
        self.id
    }

    fn set_id(&mut self, id: usize) {
        self.id = id;
    }
}


impl Person {
    pub fn new(name: &str, age: u32) -> Self {
        Self {
            id: 0,
            name: Varchar::<20>::new(name),
            age
        }
    }

    pub fn as_json(&self) -> JsonValue {
        json!({
            "id": self.id,
            "name": self.name.to_string(),
            "age": self.age
        })
    }

    pub fn from_json(j: &Json<JsonValue>) -> Self {
        Self {
            id: j["id"].as_u64().unwrap() as usize,
            name: Varchar::<20>::new(j["name"].to_string().as_str()),
            age: j["age"].as_u64().unwrap() as u32
        }
    }
}

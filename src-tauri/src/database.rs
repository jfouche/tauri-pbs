use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc, Mutex,
};

#[derive(Debug)]
pub struct Item {
    id: usize,
    part_number: String,
    name: String,
}

impl Item {
    fn new(id: usize) -> Self {
        Item {
            id,
            part_number: "xxxxxxxx".into(),
            name: String::new(),
        }
    }
}

#[derive(Default)]
pub struct Database {
    counter: AtomicUsize,
    items: Arc<Mutex<Vec<Arc<Mutex<Item>>>>>,
}

impl Database {
    pub fn create_item(&mut self) -> Arc<Mutex<Item>> {
        let id = self.counter.fetch_add(1, Ordering::Relaxed);
        let item = Arc::new(Mutex::new(Item::new(id)));

        match self.items.lock() {
            Ok(mut vec) => vec.push(Arc::clone(&item)),
            Err(err) => {
                panic!("create_item failed : {}", err)
            }
        }

        item
    }
}

#[cfg(test)]
mod tests {

    use super::Database;

    #[test]
    fn create_item() {
        let mut db = Database::default();
        let item = db.create_item();
        println!("Item : {:?}", item);
    }

    #[test]
    fn update_item() {
        let mut db = Database::default();
        let mut item = db.create_item();
        let mut item = item.lock().unwrap();
        (*item).name = "coucou".into();
    }
}

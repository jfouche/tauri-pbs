#![allow(dead_code)]

use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc, Mutex,
};

#[derive(Debug)]
struct ItemImpl {
    id: usize,
    part_number: String,
    name: String,
}

impl ItemImpl {
    fn new(id: usize) -> Self {
        ItemImpl {
            id,
            part_number: "xxxxxxxx".into(),
            name: String::new(),
        }
    }
}

#[derive(Debug)]
pub struct Item(Arc<Mutex<ItemImpl>>);

impl Item {
    fn new(id: usize) -> Self {
        Item(Arc::new(Mutex::new(ItemImpl::new(id))))
    }

    pub fn set_name(&mut self, name: impl Into<String>) {
        let mut item = self.0.lock().expect("Item::set_name can't lock item");
        item.name = name.into();
    }

    /// Get a copy of the name
    ///
    pub fn get_name(&self) -> String {
        self.0
            .lock()
            .expect("Item::get_name can't lock item")
            .name
            .clone()
    }
}

impl Clone for Item {
    fn clone(&self) -> Self {
        Item(Arc::clone(&self.0))
    }
}

#[derive(Default)]
pub struct Database {
    counter: AtomicUsize,
    items: Arc<Mutex<Vec<Item>>>,
}

impl Database {
    pub fn create_item(&mut self) -> Item {
        let id = self.counter.fetch_add(1, Ordering::Relaxed);
        let item = Item::new(id);

        match self.items.lock() {
            Ok(mut vec) => vec.push(item.clone()),
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
        item.set_name("MY_NAME");
        assert_eq!("MY_NAME", item.get_name());
        println!("Item : {:?}", item);
    }
}

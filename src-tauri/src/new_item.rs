use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct NewItem<'a> {
    pn: &'a str,
    name: &'a str,
}

impl NewItem<'_> {
    /// Check the item to verify if it's valid or not
    ///
    /// Returns `Ok()` if it's valid
    ///
    /// Returns an `Err(String)` with the text indicating why it's invalid
    fn check(&self) -> Result<(), String> {
        if self.pn.is_empty() {
            Err("PN is empty".into())
        } else if self.name.is_empty() {
            Err("Name is empty".into())
        } else {
            Ok(())
        }
    }
}

pub mod cmd {

    use tauri::State;

    use crate::TauriDatabase;

    use super::*;

    #[tauri::command]
    pub async fn new_part_number(
        new_item: NewItem<'_>,
        db: State<'_, TauriDatabase>,
    ) -> Result<String, String> {
        println!("new_part_number - 1");
        new_item.check()?;
        println!("new_part_number - 2");
        let mut db = db.0.lock().unwrap();
        let mut item = db.create_item();
        item.set_name(new_item.name);
        Ok(format!("Item crated: {:?}", item))
    }
}

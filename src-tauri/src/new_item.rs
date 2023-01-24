use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct NewItem<'a> {
    pn: &'a str,
    name: &'a str,
}

impl NewItem<'_> {
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
    use super::*;

    #[tauri::command]
    pub async fn new_part_number(new_item: NewItem<'_>) -> Result<String, String> {
        println!("new_part_number - 1");
        new_item.check()?;
        println!("new_part_number - 2");
        Ok(format!("Item crated: {:?}", new_item))
    }
}

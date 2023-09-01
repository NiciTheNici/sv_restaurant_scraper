use crate::Menu;

#[derive(Debug)]
pub struct Restaurant {
    pub name: String,
    pub link: String,
    pub menus: Vec<Menu>,
}

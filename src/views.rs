#[derive(Debug, Clone, Copy, PartialEq)]
pub enum View {
    Menu,
    Option1,
    Option2,
}

impl Default for View {
    fn default() -> Self {
        View::Menu
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MenuOption {
    Option1,
    Option2,
    Option3,
}

pub mod profile;

#[derive(Clone)]
pub enum Views {
    /// select profile screen
    SelectProfile,
    /// view profile screen
    ViewProfile,
    /// create profile screen
    CreateProfile,
}

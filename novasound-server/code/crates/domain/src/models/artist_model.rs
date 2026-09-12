#[derive(Debug)]
pub struct Artist {
    pub id: String,
    pub name: String,
    pub image_path: String,
}

#[derive(Debug)]
pub struct CreateArtist {
    pub name: String,
}

#[derive(Debug)]
pub struct UpdateArtist {
    pub name: String,
}

impl CreateArtist {
    pub fn validate(&self) -> Result<(), ValidationErrors> {
        validate_name(&self.name)
    }
}

impl UpdateArtist {
    pub fn validate(&self) -> Result<(), ValidationErrors> {
        validate_name(&self.name)
    }
}

fn validate_name(name: &str) -> Result<(), ValidationErrors> {
    if has_non_empty_name(name) {
        Ok(())
    } else {
        Err(ValidationIssue::new("name", "required", "Artist name cannot be empty").into())
    }
}
use crate::rules::has_non_empty_name;
use crate::validation::{ValidationErrors, ValidationIssue};

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

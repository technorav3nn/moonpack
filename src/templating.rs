use ramhorns::Content;

#[derive(Content)]
pub struct ChunkTemplate {
    pub path: String,
    pub name: String,
    pub code: String,
}

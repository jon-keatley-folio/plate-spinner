pub enum Commands {
    Create,
    Edit,
    Pause,
    Spin,
    Delete,
    NoAction,
}

pub struct InfoItem {
    pub key: String,
    pub description: String,
}

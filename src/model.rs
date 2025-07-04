
#[allow(dead_code)]
#[derive(Debug)]
pub struct User {
    pub id: usize,
    pub avatar: String,
    pub name: String,
    pub room: Option<Room>,
    pub address: String,
    pub sex: bool,
    pub ip: String,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Room {
    pub id: usize,
    pub room_img_url: String,
    pub title: String,
    pub hot: usize,
    pub is_live: bool,
    pub tags: Vec<Tag>,
}

#[derive(Debug)]
pub enum Tag {
    A,
    B,
    C,
    D,
    E,
}

impl From<usize> for Tag {
    fn from(value: usize) -> Self {
        match value {
            0 => Tag::A,
            1 => Tag::B,
            2 => Tag::C,
            3 => Tag::D,
            4 => Tag::E,
            num => panic!("vaild tag num: {}", num),
        }
    }
}

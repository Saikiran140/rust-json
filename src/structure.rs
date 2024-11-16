use serde:: {Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)
]
pub struct Address {
    pub street: String,
    pub city: String,
    pub state: String,
    pub postalCode: u32,
}

#[derive(Serialize, Deserialize, Debug)
]
pub struct PhoneNumber {
    pub r#type: String,
    pub number: String,
}

#[derive(Serialize, Deserialize, Debug)
]
pub struct SocialMedia {
    pub twitter: String,
    pub linkedin: String,
}

#[derive(Serialize, Deserialize, Debug)
]
pub struct Hobbies {
    pub indoor: Vec<String>,
    pub outdoor: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PersonData {
    pub name: String,
    pub age: u8,
    pub isActive: bool,
    pub email: String,
    pub address: Address,
    pub phoneNumbers: Vec<PhoneNumber>,
    pub favoriteNumbers: Vec<u32>,
    pub isVerified: bool,
    pub profilePicture: Option<String>,
    pub socialMedia: SocialMedia,
    pub hobbies: Vec<Hobby>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Hobby {
    Simple(String),
    Complex(Hobbies),
}
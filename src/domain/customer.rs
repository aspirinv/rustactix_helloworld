    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize)]
    pub struct Customer {
        pub name: String,
        pub id: u32
    }

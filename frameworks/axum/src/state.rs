use crate::entries::TextEntry;

const MONGODB_DATABASE: &str = "axum";
const MONGODB_COLLECTION: &str = "texts";

pub struct MongoAppState {
    client: mongodb::Client,
}

impl MongoAppState {
    pub fn new(client: mongodb::Client) -> MongoAppState {
        MongoAppState {
            client,
        }
    }
    pub fn client(&self) -> mongodb::Collection<TextEntry> {
        return self
            .client
            .database(MONGODB_DATABASE)
            .collection::<TextEntry>(MONGODB_COLLECTION);
    }
}

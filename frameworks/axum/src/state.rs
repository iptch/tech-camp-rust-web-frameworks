use crate::entries::TextEntry;

pub struct MongoAppState {
    client: mongodb::Client,
    collection: String,
    database: String,
}

impl MongoAppState {
    pub fn new(client: mongodb::Client, database: String, collection: String) -> MongoAppState {
        MongoAppState {
            client,
            database,
            collection,
        }
    }
    pub fn client(&self) -> mongodb::Collection<TextEntry> {
        return self
            .client
            .database(&self.database)
            .collection::<TextEntry>(&self.collection);
    }
}

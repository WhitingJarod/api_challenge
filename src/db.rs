use std::{
    collections::HashMap,
    env,
    sync::{Arc, RwLock},
};

use mongodb::{bson::doc, options::ClientOptions, Client, Database};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Serialize, Deserialize, Clone)]
pub struct MovieRecord {
    record_id: String,
    title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    rating: Option<f64>,
}

#[derive(Clone)]
pub struct DB {
    data: Arc<RwLock<HashMap<String, Vec<MovieRecord>>>>,
    client: Database,
}
impl DB {
    pub async fn init() -> Result<Self> {
        let mut client_options = ClientOptions::parse(env::var("MONGODB_URI")?).await?;
        client_options.app_name = Some("Movie Tracker Backend".to_string());
        let client = Client::with_options(client_options)?;

        let client = client.database("watched-movies");
        let mut data = HashMap::new();

        // list collections, where each is a user
        for user in client.list_collection_names(None).await? {
            let mut col = Vec::new();
            let collection = client.collection::<MovieRecord>(&user);
            let mut cursor = collection.find(None, None).await?;
            while cursor.advance().await? {
                col.push(cursor.deserialize_current()?);
            }
            data.insert(user, col);
        }
        Ok(DB {
            data: Arc::new(RwLock::new(data)),
            client,
        })
    }

    pub async fn post_movie(
        &self,
        user: &str,
        title: String,
        rating: Option<f64>,
    ) -> Result<MovieRecord> {
        let collection = self.client.collection::<MovieRecord>(user);
        let uuid = Uuid::new_v4().to_string();
        let movie = MovieRecord {
            record_id: uuid,
            title,
            rating,
        };
        collection.insert_one(movie.clone(), None).await?;
        self.data
            .write()
            .map_err(|e| e.to_string())?
            .entry(user.to_string())
            .or_insert_with(Vec::new)
            .push(movie.clone());
        Ok(movie)
    }

    pub async fn get_movies(&self, user: &str) -> Result<Vec<MovieRecord>> {
        Ok(self
            .data
            .read()
            .map_err(|e| e.to_string())?
            .get(user)
            .ok_or(String::from("no records for user"))?
            .clone())
    }

    pub async fn get_movie(&self, user: &str, record_id: &str) -> Result<MovieRecord> {
        Ok(self
            .data
            .read()
            .map_err(|e| e.to_string())?
            .get(user)
            .ok_or(String::from("no records for user"))?
            .iter()
            .find(|m| m.record_id == record_id)
            .ok_or(String::from("no matching record"))?
            .clone())
    }

    pub async fn put_movie(
        &self,
        user: &str,
        record_id: &str,
        title: String,
        rating: Option<f64>,
    ) -> Result<()> {
        let movie = MovieRecord {
            record_id: record_id.to_string(),
            title: title.clone(),
            rating,
        };
        let filter = doc! { "record_id": record_id };
        self.client
            .collection(user)
            .replace_one(filter, movie.clone(), None)
            .await?;

        let mut data = self.data.write().map_err(|e| e.to_string())?;
        let data = data
            .get_mut(user)
            .ok_or(String::from("no records for user"))?;
        for record in data {
            if record.record_id == record_id {
                record.rating = rating;
                record.title = title;
                return Ok(());
            }
        }
        Err("no matching record".into())
    }

    pub async fn delete_movie(&self, user: &str, record_id: &str) -> Result<()> {
        let filter = doc! { "record_id": record_id };
        self.client
            .collection::<MovieRecord>(user)
            .delete_one(filter, None)
            .await?;
        let mut data = self.data.write().map_err(|e| e.to_string())?;
        let data = data
            .get_mut(user)
            .ok_or(String::from("no records for user"))?;
        let index = data
            .iter()
            .position(|m| m.record_id == record_id)
            .ok_or(String::from("no matching record"))?;
        data.remove(index);
        Ok(())
    }
}

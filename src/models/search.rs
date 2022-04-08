
use serde::{Serialize, Deserialize};
use meilisearch_sdk::{document::*, client::*, search::SearchResult};
use anyhow::Result;

#[derive(Serialize, Deserialize, Debug)]
pub struct Licenceholder {
    id: f64,
    holder: String,
}

// That trait is required to make a struct usable by an index
impl Document for Licenceholder {
    type UIDType = f64;

    fn get_uid(&self) -> &Self::UIDType {
        &self.id
    }
}

pub async fn meili_search(query: &str) -> Result<Vec<SearchResult<Licenceholder>>> {
        let client = Client::new("http://10.13.100.16:7700", "secret");
        let result = client.index("candata")
            .search()
            .with_query(query)
            .execute::<Licenceholder>()
            .await
            .expect("cannot get meilidata");

        println!("{:?}", result.hits);

        Ok(result.hits)
}
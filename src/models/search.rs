
use serde::{Serialize, Deserialize};
use meilisearch_sdk::{document::*, client::*, search::SearchResult};
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct Licenceholder {
    id: f64,
    holder: String,
    website: String,
}

// That trait is required to make a struct usable by an index
impl Document for Licenceholder {
    type UIDType = f64;

    fn get_uid(&self) -> &Self::UIDType {
        &self.id
    }
}

pub async fn meili_search(query: &str) -> Result<Vec<Licenceholder>> {
    let mut fresh_formatted_results = Vec::new();
                 
    let client = Client::new("http://10.13.100.16:7700", "secret");
    let result = client.index("candata")
        .search()
        .with_query(query)
        .execute::<Licenceholder>()
        .await?;
    for res in result.hits {
        fresh_formatted_results.push(res.result);
    }
    Ok(fresh_formatted_results)
}
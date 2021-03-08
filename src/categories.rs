use serde::{Deserialize, Serialize};

use crate::client::Client;
use crate::errors::Result;

#[derive(Serialize)]
struct GetCategoriesRequest {}

#[derive(Deserialize, Debug, Clone)]
pub struct GetCategoriesResponse {
    /// An array of all of the transaction categories used by Plaid.
    pub categories: Vec<Category>,
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Category {
    /// An identifying number for the category. category_id is a Plaid-specific identifier and does not necessarily correspond to merchant category codes.
    pub category_id: String,
    /// `place` for physical transactions or `special` for other transactions such as bank charges.
    pub group: String,
    /// A hierarchical array of the categories to which this category_id belongs.
    pub hierarchy: Vec<String>,
}

impl Client {
    /// Get Categories
    ///
    /// Send a request to the /categories/get endpoint to get detailed information on categories returned by Plaid.
    pub async fn get_categories(&self) -> Result<GetCategoriesResponse> {
        self.send_request("categories/get", &GetCategoriesRequest {})
            .await
    }
}

#[cfg(test)]
mod tests {
    use crate::client::tests::get_test_client;

    #[tokio::test]
    async fn test_get_categories() {
        let client = get_test_client();
        let categories_resp = client.get_categories().await.unwrap();
        assert_eq!(categories_resp.categories[0].category_id, "10000000");
        assert_eq!(categories_resp.categories[0].group, "special");
    }
}

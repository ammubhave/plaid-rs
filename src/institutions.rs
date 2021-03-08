use serde::{Deserialize, Serialize};

use crate::client::Client;
use crate::errors::Result;

#[derive(Deserialize, Debug, Clone)]
pub struct Institution {
    /// Unique identifier for the institution
    pub institution_id: String,
    /// The official name of the institution
    pub name: String,
    /// A list of the Plaid products supported by the institution
    /// Possible values: assets, auth, balance, identity, investments, liabilities, payment_initiation, transactions, credit_details, income, deposit_switch
    pub products: Vec<String>,
    /// A list of the country codes supported by the institution.
    /// Possible values: US, GB, ES, NL, FR, IE, CA
    pub country_codes: Vec<String>,
    /// The URL for the institution's website
    pub url: Option<String>,
    /// Hexadecimal representation of the primary color used by the institution
    pub primary_color: Option<String>,
    /// Base64 encoded representation of the institution's logo
    pub logo: Option<String>,
    /// A partial list of routing numbers associated with the institution. This list is provided for the purpose of looking up institutions by routing number. It is not comprehensive and should never be used as a complete list of routing numbers for an institution.
    pub routing_numbers: Option<Vec<String>>,
    /// Indicates that the institution has an OAuth login flow. This is primarily relevant to institutions with European country codes.
    pub oauth: bool,
}

#[derive(Serialize)]
struct GetInstitutionsRequest<'a> {
    client_id: &'a str,
    secret: &'a str,
    count: i32,
    offset: i32,
    country_codes: &'a [&'a str],
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<GetInstitutionsOptions>,
}

#[derive(Serialize)]
pub struct GetInstitutionsOptions {
    /// Filter the Institutions based on which products they support.
    /// Possible values: assets, auth, balance, identity, investments, liabilities, payment_initiation, transactions, credit_details, income, deposit_switch
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub products: Vec<String>,
    /// Specify an array of routing numbers to filter institutions.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub routing_numbers: Vec<String>,
    /// Limit results to institutions with or without OAuth login flows. This is primarily relevant to institutions with European country codes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth: Option<bool>,
    /// When true, return the institution's homepage URL, logo and primary brand color.
    pub include_optional_metadata: bool,
}

impl Default for GetInstitutionsOptions {
    fn default() -> Self {
        Self {
            products: Vec::new(),
            include_optional_metadata: false,
            oauth: None,
            routing_numbers: Vec::new(),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct GetInstitutionsResponse {
    /// A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive.
    pub request_id: String,
    /// A list of Plaid Institution
    pub institutions: Vec<Institution>,
    /// The number of institutions returned
    pub total: i32,
}

#[derive(Serialize)]
struct GetInstitutionByIdRequest<'a> {
    client_id: &'a str,
    secret: &'a str,
    institution_id: &'a str,
    country_codes: &'a [&'a str],
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<GetInstitutionByIdOptions>,
}

#[derive(Serialize)]
pub struct GetInstitutionByIdOptions {
    /// When true, return an institution's logo, brand color, and URL. When available, the bank's logo is returned as a base64 encoded 152x152 PNG, the brand color is in hexadecimal format. The default value is false.
    pub include_optional_metadata: bool,
    /// If true, the response will include status information about the institution. Default value is false.
    pub include_status: bool,
}

impl Default for GetInstitutionByIdOptions {
    fn default() -> Self {
        Self {
            include_optional_metadata: false,
            include_status: false,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct GetInstitutionByIdResponse {
    request_id: String,
    institution: Institution,
}

#[derive(Serialize)]
struct SearchInstitutionsRequest<'a> {
    client_id: &'a str,
    secret: &'a str,
    query: &'a str,
    country_codes: &'a [&'a str],
    products: &'a [&'a str],
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<SearchInstitutionsOptions>,
}

#[derive(Serialize)]
pub struct SearchInstitutionsOptions {
    include_optional_metadata: bool,
    // account_filter:
    #[serde(skip_serializing_if = "Option::is_none")]
    oauth: Option<bool>,
}

impl Default for SearchInstitutionsOptions {
    fn default() -> Self {
        Self {
            include_optional_metadata: false,
            oauth: None,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct SearchInstitutionsResponse {
    request_id: String,
    institutions: Vec<Institution>,
}

impl Client {
    /// Get details of an institution.
    ///
    /// Returns a JSON response containing details on a specified financial institution currently supported by Plaid.
    ///
    /// * `institution_id` - The ID of the institution to get details about
    /// * `country_codes` - Specify an array of Plaid-supported country codes this institution supports, using the ISO-3166-1 alpha-2 country code standard. Possible values: US, GB, ES, NL, FR, IE, CA
    /// * `options` - Specifies optional parameters for /institutions/get_by_id.
    pub async fn get_institution_by_id(
        &self,
        institution_id: &str,
        country_codes: &[&str],
        options: Option<GetInstitutionByIdOptions>,
    ) -> Result<GetInstitutionByIdResponse> {
        self.send_request(
            "institutions/get_by_id",
            &GetInstitutionByIdRequest {
                client_id: &self.client_id,
                secret: &self.secret,
                institution_id,
                country_codes,
                options,
            },
        )
        .await
    }

    /// Get details of all supported institutions.
    ///
    /// Returns a JSON response containing details on all financial institutions currently supported by Plaid. Because Plaid supports thousands of institutions, results are paginated.
    /// This data changes frequently. If you store it locally on your system, be sure to update it regularly.
    ///
    /// * `count` - The total number of Institutions to return.
    /// * `offset` - The number of Institutions to skip.
    /// * `country_codes` - Specify an array of Plaid-supported country codes this institution supports, using the ISO-3166-1 alpha-2 country code standard.
    /// * `options` - An optional object to filter /institutions/get results.
    pub async fn get_institutions(
        &self,
        count: i32,
        offset: i32,
        country_codes: &[&str],
        options: Option<GetInstitutionsOptions>,
    ) -> Result<GetInstitutionsResponse> {
        self.send_request(
            "institutions/get",
            &GetInstitutionsRequest {
                client_id: &self.client_id,
                secret: &self.secret,
                count,
                offset,
                country_codes,
                options,
            },
        )
        .await
    }

    /// Search institutions.
    ///
    /// Returns a JSON response containing details for institutions that match the query parameters, up to a maximum of ten institutions per query.
    ///
    /// * `query` - The search query. Institutions with names matching the query are returned
    /// * `products` - Filter the Institutions based on whether they support all products listed in products.
    /// * `country_codes` - Specify an array of Plaid-supported country codes this institution supports, using the ISO-3166-1 alpha-2 country code standard.
    /// * `options` - An optional object to filter /institutions/search results.
    pub async fn search_institutions(
        &self,
        query: &str,
        products: &[&str],
        country_codes: &[&str],
        options: Option<SearchInstitutionsOptions>,
    ) -> Result<SearchInstitutionsResponse> {
        self.send_request(
            "institutions/search",
            &SearchInstitutionsRequest {
                client_id: &self.client_id,
                secret: &self.secret,
                query,
                products,
                country_codes,
                options,
            },
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::tests::{get_test_client, SANDBOX_INSTITUTION_QUERY};

    #[tokio::test]
    async fn test_get_institutions() {
        let client = get_test_client();

        let resp = client.get_institutions(2, 1, &["US"], None).await.unwrap();
        assert_eq!(resp.institutions.len(), 2);

        let resp = client
            .get_institutions(
                2,
                1,
                &["US"],
                Some(GetInstitutionsOptions {
                    include_optional_metadata: true,
                    ..Default::default()
                }),
            )
            .await
            .unwrap();
        assert_eq!(resp.institutions.len(), 2);
        for institution in &resp.institutions {
            assert_eq!(institution.url.is_some(), true);
            assert_ne!(institution.url.as_ref().unwrap().len(), 0);
        }

        let resp = client
            .get_institutions(
                2,
                1,
                &["GB"],
                Some(GetInstitutionsOptions {
                    oauth: Some(true),
                    ..Default::default()
                }),
            )
            .await
            .unwrap();
        assert_eq!(resp.institutions.len(), 2);

        let resp = client.get_institutions(2, 1, &[], None).await;
        assert_eq!(resp.is_err(), true);

        let resp = client
            .get_institutions(
                1,
                0,
                &["US"],
                Some(GetInstitutionsOptions {
                    routing_numbers: vec!["021200339".to_string(), "052001633".to_string()],
                    ..Default::default()
                }),
            )
            .await
            .unwrap();
        assert_eq!(resp.institutions.len(), 1);
    }

    #[tokio::test]
    async fn test_search_institutions() {
        let client = get_test_client();

        let resp = client
            .search_institutions(SANDBOX_INSTITUTION_QUERY, &["transactions"], &["US"], None)
            .await
            .unwrap();
        assert!(resp.institutions.len() > 0);

        let resp = client
            .search_institutions(
                SANDBOX_INSTITUTION_QUERY,
                &["transactions"],
                &["US"],
                Some(SearchInstitutionsOptions {
                    include_optional_metadata: true,
                    ..Default::default()
                }),
            )
            .await
            .unwrap();
        assert!(resp.institutions.len() > 0);
        for institution in &resp.institutions {
            assert_eq!(institution.url.is_some(), true);
            assert_ne!(institution.url.as_ref().unwrap().len(), 0);
        }

        let resp = client
            .search_institutions(SANDBOX_INSTITUTION_QUERY, &["transactions"], &[""], None)
            .await;
        assert_eq!(resp.is_err(), true);
    }

    #[tokio::test]
    async fn test_get_institutions_by_id() {
        let client = get_test_client();

        let resp = client
            .get_institution_by_id("ins_12", &["US"], None)
            .await
            .unwrap();
        assert!(resp.institution.products.len() > 0);

        let resp = client
            .get_institution_by_id(
                "ins_12",
                &["US"],
                Some(GetInstitutionByIdOptions {
                    include_optional_metadata: true,
                    ..Default::default()
                }),
            )
            .await
            .unwrap();
        assert!(resp.institution.products.len() > 0);
        assert_eq!(resp.institution.url.is_some(), true);
        assert_ne!(resp.institution.url.as_ref().unwrap().len(), 0);

        let resp = client.get_institution_by_id("ins_12", &[], None).await;
        assert_eq!(resp.is_err(), true);
    }
}

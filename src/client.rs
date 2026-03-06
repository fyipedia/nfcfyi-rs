use crate::types::*;

const DEFAULT_BASE_URL: &str = "https://nfcfyi.com/api";

/// Async client for the NFCFYI API.
pub struct Client {
    base_url: String,
    http: reqwest::Client,
}

impl Client {
    /// Creates a new client with the default base URL.
    pub fn new() -> Self {
        Self {
            base_url: DEFAULT_BASE_URL.to_string(),
            http: reqwest::Client::new(),
        }
    }

    /// Creates a new client with a custom base URL.
    pub fn with_base_url(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            http: reqwest::Client::new(),
        }
    }

    async fn get<T: serde::de::DeserializeOwned>(&self, path: &str) -> Result<T, NfcFyiError> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self.http.get(&url).send().await?;
        if !resp.status().is_success() {
            return Err(NfcFyiError::Api {
                status: resp.status().as_u16(),
                body: resp.text().await.unwrap_or_default(),
            });
        }
        Ok(resp.json().await?)
    }

    /// Search across NFC chips, standards, and glossary terms.
    pub async fn search(&self, query: &str) -> Result<SearchResult, NfcFyiError> {
        let encoded = urlencoding(query);
        self.get(&format!("/search/?q={}", encoded)).await
    }

    /// Get details for an NFC chip by slug.
    pub async fn chip(&self, slug: &str) -> Result<ChipDetail, NfcFyiError> {
        self.get(&format!("/chip/{}/", slug)).await
    }

    /// Get details for an NFC chip family by slug.
    pub async fn chip_family(&self, slug: &str) -> Result<ChipFamilyDetail, NfcFyiError> {
        self.get(&format!("/chip-family/{}/", slug)).await
    }

    /// Get details for an NFC standard by slug.
    pub async fn standard(&self, slug: &str) -> Result<StandardDetail, NfcFyiError> {
        self.get(&format!("/standard/{}/", slug)).await
    }

    /// Get details for an NFC operating mode by slug.
    pub async fn operating_mode(&self, slug: &str) -> Result<OperatingModeDetail, NfcFyiError> {
        self.get(&format!("/operating-mode/{}/", slug)).await
    }

    /// Get details for an NDEF type by slug.
    pub async fn ndef_type(&self, slug: &str) -> Result<NdefTypeDetail, NfcFyiError> {
        self.get(&format!("/ndef-type/{}/", slug)).await
    }

    /// Get details for an NFC use case by slug.
    pub async fn use_case(&self, slug: &str) -> Result<UseCaseDetail, NfcFyiError> {
        self.get(&format!("/use-case/{}/", slug)).await
    }

    /// Get a glossary term by slug.
    pub async fn glossary_term(&self, slug: &str) -> Result<GlossaryTerm, NfcFyiError> {
        self.get(&format!("/glossary/{}/", slug)).await
    }

    /// Compare two NFC chips.
    pub async fn compare(&self, slug_a: &str, slug_b: &str) -> Result<CompareResult, NfcFyiError> {
        self.get(&format!("/compare/?a={}&b={}", slug_a, slug_b)).await
    }

    /// Get a random NFC chip.
    pub async fn random(&self) -> Result<ChipDetail, NfcFyiError> {
        self.get("/random/").await
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

fn urlencoding(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => c.to_string(),
            ' ' => "+".to_string(),
            _ => format!("%{:02X}", c as u32),
        })
        .collect()
}

use crate::io::fs::files::read_file_content;
use crate::parsers::html::conversion::{HtmlToMarkdownParser, ParserConfig};
use reqwest::blocking::Client as SyncClient;
use reqwest::Client as AsyncClient;
use serde_json::Value;

pub struct AsyncTikaClient {
    pub server_url: String,
    client: AsyncClient,
}

impl AsyncTikaClient {
    pub fn new(server_url: &str) -> Self {
        AsyncTikaClient {
            server_url: server_url.to_string(),
            client: AsyncClient::new(),
        }
    }

    pub async fn get_recursive_metadata_buffer(&self, buffer: &[u8]) -> Vec<Value> {
        let response = self
            .client
            .put(&format!("{}/rmeta/ignore", self.server_url))
            .header("Accept", "application/json")
            .body(buffer.to_vec())
            .send()
            .await
            .unwrap();

        response.json::<Vec<Value>>().await.unwrap()
    }

    pub async fn get_recursive_metadata_file(&self, path: &str) -> Vec<Value> {
        let buffer = read_file_content(path).unwrap();
        self.get_recursive_metadata_buffer(&buffer).await
    }

    pub async fn get_recursive_content_html_buffer(&self, buffer: &[u8]) -> Vec<String> {
        let response = self
            .client
            .put(&format!("{}/rmeta/html", self.server_url))
            .header("Accept", "application/json")
            .body(buffer.to_vec())
            .send()
            .await
            .unwrap();

        let json_data = response.json::<Vec<Value>>().await.unwrap();

        json_data
            .iter()
            .filter_map(|data| {
                if let Some(content) = data["X-TIKA:content"].as_str() {
                    Some(content.to_string())
                } else {
                    None
                }
            })
            .collect()
    }

    pub async fn get_recursive_content_html_file(&self, path: &str) -> Vec<String> {
        let buffer = read_file_content(path).unwrap();
        self.get_recursive_content_html_buffer(&buffer).await
    }

    pub async fn get_recursive_content_markdown_buffer(
        &self,
        buffer: &[u8],
        output_links: bool,
        output_images: bool,
    ) -> Vec<String> {
        self.get_recursive_content_html_buffer(buffer)
            .await
            .iter()
            .map(|html| {
                HtmlToMarkdownParser::new(
                    ParserConfig::new(None, output_links, output_images),
                    &html,
                )
                .to_markdown()
            })
            .collect()
    }

    pub async fn get_recursive_content_markdown_file(
        &self,
        path: &str,
        output_links: bool,
        output_images: bool,
    ) -> Vec<String> {
        let buffer = read_file_content(path).unwrap();
        self.get_recursive_content_markdown_buffer(&buffer, output_links, output_images)
            .await
    }
}

pub struct SyncTikaClient {
    pub server_url: String,
    client: SyncClient,
}

impl SyncTikaClient {
    pub fn new(server_url: &str) -> Self {
        SyncTikaClient {
            server_url: server_url.to_string(),
            client: SyncClient::new(),
        }
    }

    pub fn get_recursive_metadata_buffer(&self, buffer: &[u8]) -> Vec<Value> {
        let response = self
            .client
            .put(&format!("{}/rmeta/ignore", self.server_url))
            .header("Accept", "application/json")
            .body(buffer.to_vec())
            .send()
            .unwrap();

        response.json::<Vec<Value>>().unwrap()
    }

    pub fn get_recursive_metadata_file(&self, path: &str) -> Vec<Value> {
        let buffer = read_file_content(path).unwrap();
        self.get_recursive_metadata_buffer(&buffer)
    }

    pub fn get_recursive_content_html_buffer(&self, buffer: &[u8]) -> Vec<String> {
        let response = self
            .client
            .put(&format!("{}/rmeta/html", self.server_url))
            .header("Accept", "application/json")
            .body(buffer.to_vec())
            .send()
            .unwrap();

        let json_data = response.json::<Vec<Value>>().unwrap();

        json_data
            .iter()
            .filter_map(|data| {
                if let Some(content) = data["X-TIKA:content"].as_str() {
                    Some(content.to_string())
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn get_recursive_content_html_file(&self, path: &str) -> Vec<String> {
        let buffer = read_file_content(path).unwrap();
        self.get_recursive_content_html_buffer(&buffer)
    }

    pub fn get_recursive_content_markdown_buffer(
        &self,
        buffer: &[u8],
        output_links: bool,
        output_images: bool,
    ) -> Vec<String> {
        self.get_recursive_content_html_buffer(buffer)
            .iter()
            .map(|html| {
                HtmlToMarkdownParser::new(
                    ParserConfig::new(None, output_links, output_images),
                    &html,
                )
                .to_markdown()
            })
            .collect()
    }

    pub fn get_recursive_content_markdown_file(
        &self,
        path: &str,
        output_links: bool,
        output_images: bool,
    ) -> Vec<String> {
        let buffer = read_file_content(path).unwrap();
        self.get_recursive_content_markdown_buffer(&buffer, output_links, output_images)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_server_url() -> String {
        "http://tika-alb-1540561742.us-east-2.elb.amazonaws.com".to_string()
    }

    // test basic html file
    #[tokio::test]
    async fn test_async_get_recursive_metadata_buffer() {
        let buffer = b"<html><body>Hello, world!</body></html>";
        let client = AsyncTikaClient::new(&get_server_url());
        let metadata = client.get_recursive_metadata_buffer(buffer).await;
        assert_eq!(metadata[0]["Content-Encoding"], "ISO-8859-1");
        assert_eq!(metadata[0]["Content-Length"], "39");
    }

    #[test]
    fn test_sync_get_recursive_metadata_buffer() {
        let buffer = b"<html><body>Hello, world!</body></html>";
        let client = SyncTikaClient::new(&get_server_url());
        let metadata = client.get_recursive_metadata_buffer(buffer);
        assert_eq!(metadata[0]["Content-Encoding"], "ISO-8859-1");
        assert_eq!(metadata[0]["Content-Length"], "39");
    }

    #[tokio::test]
    async fn test_async_get_recursive_metadata_buffer_pdf() {
        let path = std::env::var("CARGO_MANIFEST_DIR").unwrap() + "/resources/test1.pdf";
        let client = AsyncTikaClient::new(&get_server_url());
        let metadata = client.get_recursive_metadata_file(&path).await;

        assert_eq!(metadata[0]["Content-Type"], "application/pdf");
        assert_eq!(
            metadata[0]["signature:name"],
            "Government Publishing Office"
        );
    }

    #[test]
    fn test_sync_get_recursive_metadata_buffer_pdf() {
        let path = std::env::var("CARGO_MANIFEST_DIR").unwrap() + "/resources/test1.pdf";
        let client = SyncTikaClient::new(&get_server_url());
        let metadata = client.get_recursive_metadata_file(&path);

        assert_eq!(metadata[0]["Content-Type"], "application/pdf");
        assert_eq!(
            metadata[0]["signature:name"],
            "Government Publishing Office"
        );
    }

    #[tokio::test]
    async fn test_async_get_recursive_content_html_buffer() {
        let buffer = b"<html><body>Hello, world!</body></html>";
        let client = AsyncTikaClient::new(&get_server_url());
        let content = client.get_recursive_content_html_buffer(buffer).await;
        assert!(content[0].contains("<title></title>\n</head>\n<body>Hello, world!</body>"));
    }

    #[test]
    fn test_sync_get_recursive_content_html_buffer() {
        let buffer = b"<html><body>Hello, world!</body></html>";
        let client = SyncTikaClient::new(&get_server_url());
        let content = client.get_recursive_content_html_buffer(buffer);
        assert!(content[0].contains("<title></title>\n</head>\n<body>Hello, world!</body>"));
    }

    #[tokio::test]
    async fn test_async_get_recursive_content_html_file() {
        let path = std::env::var("CARGO_MANIFEST_DIR").unwrap() + "/resources/test1.pdf";
        let client = AsyncTikaClient::new(&get_server_url());
        let content = client.get_recursive_content_html_file(&path).await;
        assert!(content[0].contains("DEPARTMENT OF AGRICULTURE"));
    }

    #[test]
    fn test_sync_get_recursive_content_html_file() {
        let path = std::env::var("CARGO_MANIFEST_DIR").unwrap() + "/resources/test1.pdf";
        let client = SyncTikaClient::new(&get_server_url());
        let content = client.get_recursive_content_html_file(&path);
        assert!(content[0].contains("DEPARTMENT OF AGRICULTURE"));
    }

    #[tokio::test]
    async fn test_async_get_recursive_content_markdown_buffer() {
        let buffer = b"<html><body><h1>Test</h1><p>Hello, world!</p></body></html>";
        let client = AsyncTikaClient::new(&get_server_url());
        let content = client
            .get_recursive_content_markdown_buffer(buffer, true, true)
            .await;
        assert_eq!(content[0], "# Test\n\nHello, world!\n");
    }

    #[test]
    fn test_sync_get_recursive_content_markdown_buffer() {
        let buffer = b"<html><body><h1>Test</h1><p>Hello, world!</p></body></html>";
        let client = SyncTikaClient::new(&get_server_url());
        let content = client.get_recursive_content_markdown_buffer(buffer, true, true);
        assert_eq!(content[0], "# Test\n\nHello, world!\n");
    }

    #[tokio::test]
    async fn test_async_get_recursive_content_markdown_file() {
        let path = std::env::var("CARGO_MANIFEST_DIR").unwrap() + "/resources/test1.pdf";
        let client = AsyncTikaClient::new(&get_server_url());
        let content = client
            .get_recursive_content_markdown_file(&path, true, true)
            .await;
        assert!(content[0].contains("DEPARTMENT OF AGRICULTURE"));
    }

    #[test]
    fn test_sync_get_recursive_content_markdown_file() {
        let path = std::env::var("CARGO_MANIFEST_DIR").unwrap() + "/resources/file1.html";
        let client = SyncTikaClient::new(&get_server_url());
        let content = client.get_recursive_content_markdown_file(&path, true, true);
        assert!(content[0].contains("# Our blog\n\nWhat we're reading, thinking, and doing.\n"));
    }

    #[test]
    fn test_sync_get_recursive_content_markdown_file_pdf() {
        let path = std::env::var("CARGO_MANIFEST_DIR").unwrap() + "/resources/test1.pdf";
        let client = SyncTikaClient::new(&get_server_url());
        let content = client.get_recursive_content_markdown_file(&path, true, true);
        assert!(content[0].contains("DEPARTMENT OF AGRICULTURE"));
    }

    #[test]
    fn test_sync_get_recursive_content_markdown_file_docx() {
        let path = std::env::var("CARGO_MANIFEST_DIR").unwrap() + "/resources/test1.docx";
        let client = SyncTikaClient::new(&get_server_url());
        let content = dbg!(client.get_recursive_content_markdown_file(&path, true, true));
        assert!(content[0].contains("\n\n**Regulatory Impact Analysis**\n\n"));
    }
}

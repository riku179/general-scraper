use crate::crawler::{format, Crawler, WebFetcher};
use crate::entity::{Content, Source};
use anyhow::Result;
use chrono::Utc;
use sha1::{Digest, Sha1};

pub async fn kick(source: Source) -> Result<(Vec<Content>, Vec<String>)> {
    let crawler = Crawler::new(WebFetcher::new(), source.last_accessed_urls.clone());
    let (artifacts, accessed_urls) = crawler.crawl(&source.selectors).await?;

    let contents = format(artifacts, vec!["title", "body", "link", "source_url"])?
        .into_iter()
        .map(|row| {
            let mut hasher = Sha1::new();
            hasher.update(format!("{}{}", row[2], Utc::now().to_rfc3339()));

            Ok(Content {
                id: format!("{:x}", hasher.finalize()),
                url: (*row[2]).clone(),
                source_id: source.id,
                title: (*row[0]).clone(),
                // TODO: fix overhead
                body: (*row[1]).clone(),
                created_at: Utc::now(),
            })
        })
        .collect::<Result<Vec<Content>>>()?;

    Ok((contents, accessed_urls))
}

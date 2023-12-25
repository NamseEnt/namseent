use crate::*;
use futures::future::try_join_all;

pub async fn load_all_typefaces() -> Result<()> {
    let default_typefaces = [
        (
            "NotoSansKR-Black",
            crate::Url::parse("bundle:__system__/font/Ko/NotoSansKR-Black.woff2").unwrap(),
        ),
        (
            "NotoSansKR-Bold",
            crate::Url::parse("bundle:__system__/font/Ko/NotoSansKR-Bold.woff2").unwrap(),
        ),
        (
            "NotoSansKR-Light",
            crate::Url::parse("bundle:__system__/font/Ko/NotoSansKR-Light.woff2").unwrap(),
        ),
        (
            "NotoSansKR-Medium",
            crate::Url::parse("bundle:__system__/font/Ko/NotoSansKR-Medium.woff2").unwrap(),
        ),
        (
            "NotoSansKR-Regular",
            crate::Url::parse("bundle:__system__/font/Ko/NotoSansKR-Regular.woff2").unwrap(),
        ),
        (
            "NotoSansKR-Thin",
            crate::Url::parse("bundle:__system__/font/Ko/NotoSansKR-Thin.woff2").unwrap(),
        ),
        (
            "NotoColorEmoji",
            crate::Url::parse("bundle:__system__/font/NotoColorEmoji.woff2").unwrap(),
        ),
    ];

    try_join_all(
        default_typefaces
            .into_iter()
            .map(|(typeface_name, url)| async move {
                let bytes = get_file_from_bundle_with_cached(&url)
                    .await
                    .map_err(|error| anyhow!("Could not fetch {}: {}", url, error))?;

                crate::system::typeface::register_typeface(typeface_name, &bytes);
                Ok::<(), anyhow::Error>(())
            }),
    )
    .await?;

    Ok(())
}

async fn get_file_from_bundle_with_cached(url: &crate::Url) -> Result<Vec<u8>> {
    let file = match crate::cache::get(url.as_str()).await? {
        Some(cached_file) => cached_file.to_vec(),
        None => {
            let file = crate::file::bundle::read(url.clone()).await?;
            crate::cache::set(url.as_str(), &file).await?;
            file
        }
    };
    Ok(file)
}

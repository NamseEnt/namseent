use crate::*;
use futures::future::try_join_all;

pub async fn load_all_typefaces() -> Result<()> {
    let default_typefaces = [
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
        (
            "NotoSansKR-Black",
            crate::Url::parse("bundle:__system__/font/Ko/NotoSansKR-Black.woff2").unwrap(),
        ),
    ];

    // for filename in [
    //     "NotoSansKR-Thin.woff2",
    //     "NotoSansKR-Light.woff2",
    //     "NotoSansKR-Medium.woff2",
    //     "NotoSansKR-Black.woff2",
    //     "NotoSansKR-Regular.woff2",
    //     "NotoSansKR-Bold.woff2",
    //     "NotoSansKR-Black.ttf",
    // ] {
    //     let file_path = format!("/bundle/__system__/font/Ko/{filename}");
    //     eprintln!("before open file {file_path}");
    //     let result = std::fs::File::open(&file_path);
    //     eprintln!("{file_path} result: {:?}", result);
    // }

    // for (typeface_name, url) in default_typefaces.into_iter() {
    //     let bytes = get_file_from_bundle_with_cached(&url)
    //         .await
    //         .map_err(|error| anyhow!("Could not fetch {}: {}", url, error))?;

    //     crate::system::typeface::register_typeface(typeface_name.to_string(), bytes).await?;
    // }

    try_join_all(
        default_typefaces
            .into_iter()
            .map(|(typeface_name, url)| async move {
                eprintln!("typeface_name start");
                let bytes = get_file_from_bundle_with_cached(&url)
                    .await
                    .map_err(|error| {
                        eprintln!("error: {:?}", error);
                        anyhow!("Could not fetch {}: {}", url, error)
                    })?;

                println!("bytes: {:?}", bytes.len());

                crate::system::typeface::register_typeface(typeface_name.to_string(), bytes)
                    .await?;

                println!("typeface_name: {:?}", typeface_name);

                Ok::<(), anyhow::Error>(())
            }),
    )
    .await?;

    Ok(())
}

async fn get_file_from_bundle_with_cached(url: &crate::Url) -> Result<Vec<u8>> {
    let file = crate::file::bundle::read(url.clone()).await?;
    Ok(file)
}

use crate::*;
use futures::future::try_join_all;

pub async fn load_all_typefaces() -> Result<()> {
    // TODO
    // let default_typefaces = [
    //     (
    //         "NotoSansKR-Bold",
    //         "__system__/font/Ko/NotoSansKR-Bold.woff2",
    //     ),
    //     (
    //         "NotoSansKR-Light",
    //         "__system__/font/Ko/NotoSansKR-Light.woff2",
    //     ),
    //     (
    //         "NotoSansKR-Medium",
    //         "__system__/font/Ko/NotoSansKR-Medium.woff2",
    //     ),
    //     (
    //         "NotoSansKR-Regular",
    //         "__system__/font/Ko/NotoSansKR-Regular.woff2",
    //     ),
    //     (
    //         "NotoSansKR-Thin",
    //         "__system__/font/Ko/NotoSansKR-Thin.woff2",
    //     ),
    //     ("NotoColorEmoji", "__system__/font/NotoColorEmoji.woff2"),
    //     (
    //         "NotoSansKR-Black",
    //         "__system__/font/Ko/NotoSansKR-Black.woff2",
    //     ),
    // ];

    // try_join_all(
    //     default_typefaces
    //         .into_iter()
    //         .map(|(typeface_name, path)| async move {
    //             let bytes = get_file_from_bundle_with_cached(path)
    //                 .await
    //                 .map_err(|error| {
    //                     eprintln!("error: {error:?}");
    //                     anyhow!("Could not fetch {}: {}", path, error)
    //                 })?;

    //             crate::system::typeface::register_typeface(typeface_name.to_string(), bytes)
    //                 .await?;

    //             Ok::<(), anyhow::Error>(())
    //         }),
    // )
    // .await?;

    Ok(())
}

async fn get_file_from_bundle_with_cached(url: &str) -> Result<Vec<u8>> {
    let file = crate::file::bundle::read(url).await?;
    Ok(file)
}

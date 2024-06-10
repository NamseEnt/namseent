use super::*;

pub enum DataFetch<T> {
    Loading,
    Err(String),
    Ok(T),
}

pub type OptionDataFetch<T> = Option<DataFetch<T>>;

pub fn option_data_fetch<'a, T: 'a, C: Component + 'a>(
    ctx: &ComposeCtx,
    fetch: &'a OptionDataFetch<T>,
    wh: Wh<Px>,
    on_ok: impl FnOnce(&'a T) -> C,
) {
    let Some(fetch) = fetch else {
        return;
    };
    match fetch {
        DataFetch::Loading => ctx.add(typography::center_text(
            wh,
            "로딩 중...",
            Color::WHITE,
            16.int_px(),
        )),
        DataFetch::Err(err) => ctx.add(typography::center_text(
            wh,
            format!("로딩 실패: {err}"),
            Color::WHITE,
            16.int_px(),
        )),
        DataFetch::Ok(data) => ctx.add(on_ok(data)),
    };
}

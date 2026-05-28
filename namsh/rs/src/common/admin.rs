use forte_sdk::*;

pub fn verify(headers: &::http::HeaderMap) -> bool {
    let Ok(expected) = std::env::var("NAMSH_ADMIN_TOKEN") else {
        return false;
    };
    let Some(raw) = headers.get(http_header::AUTHORIZATION) else {
        return false;
    };
    let Ok(value) = raw.to_str() else {
        return false;
    };
    let Some(token) = value.strip_prefix("Bearer ") else {
        return false;
    };
    constant_time_eq(token.trim().as_bytes(), expected.as_bytes())
}

fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;
    }
    diff == 0
}

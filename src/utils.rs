use std::env;

pub(crate) fn is_flatpak() -> bool {
    env::var("FLATPAK_ID").is_ok()
}

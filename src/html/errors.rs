use htmlfn::core::Content;
use super::core::other;

pub fn not_found<'a>(path: &'a str) -> impl Content + 'a {
    other("en",
        "404 Not Found",
        elements!(
            h1() {
                "404 Not Found"
            }
            p() {
                tt() { path }
                " not found on this server."
            }
        )
    )
}


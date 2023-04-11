use bary::{Server};
#[bary::bary_app(secret_key = "VGhpcyBpcyBhIHBsYWNlIGZvciB5b3VyIGFkISBMT0w=")]
pub fn load(bary: Server) {
    bary.start().expect("Server errored");
}
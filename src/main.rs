use bary::{Server};
#[bary::bary_app]
pub fn load(bary: Server) {
    bary.start().expect("Server errored");
}
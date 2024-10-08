use chuchi::{get_json, Chuchi};

#[get_json("/api/version")]
fn version() -> String {
	env!("CARGO_PKG_VERSION").into()
}

pub fn routes(server: &mut Chuchi) {
	server.add_route(version);
}

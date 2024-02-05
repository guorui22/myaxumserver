use rspc::Router;

#[tokio::main]
async fn main() {
    let router = <Router>::new()
        .query("version", |t| t(|ctx, input: ()| env!("CARGO_PKG_VERSION")))
        .build();
}
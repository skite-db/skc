#[tokio::main]
async fn main() -> skc_error::Result<()> {
    skc::cli::main().await
}

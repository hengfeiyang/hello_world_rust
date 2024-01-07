use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum MyError {
    #[snafu(display("Failed to open file: {}", path))]
    OpenFile { path: String, source: std::io::Error },
    #[snafu(display("Failed to read data from file: {}", path))]
    ReadData { path: String, source: std::io::Error },
}
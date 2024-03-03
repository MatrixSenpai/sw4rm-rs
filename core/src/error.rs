#[derive(Debug)]
pub enum Error {
    StdReadError(std::io::Error),
    YamlError(serde_yaml::Error),
    JsonError(serde_json::Error),
}
impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self { Self::StdReadError(value) }
}
impl From<serde_yaml::Error> for Error {
    fn from(value: serde_yaml::Error) -> Self { Self::YamlError(value) }
}
impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self { Self::JsonError(value) }
}

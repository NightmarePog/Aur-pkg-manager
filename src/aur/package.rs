use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RpcResponse {
    pub results: Vec<RpcPackage>,
}

#[derive(Debug, Deserialize)]
pub struct RpcPackage {
    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Version")]
    pub version: String,

    #[serde(rename = "Maintainer")]
    pub maintainer: Option<String>,

    #[serde(rename = "PackageBase")]
    pub package_base: String,
}

use std::{fs::read_to_string, path::Path};

use description_generator::{
    type_description_generator::{
        DeclarePartGenerator, PropertyPartGenerator, TypeDescriptionGenerator,
    },
    type_mapper::TypeMapper,
};
use json::json::Json;
use serde::de::value::Error;

#[derive(Debug, Clone)]
pub struct SourceValidator {
    src: String,
}
#[derive(Debug, PartialEq, Eq)]
pub enum TypeGenSource {
    File(FileSource),
}

#[derive(Debug, PartialEq, Eq)]
pub struct FileSource {}
impl FileSource {
    fn new(src: &str) -> Self {
        FileSource {}
    }
}
impl SourceValidator {
    pub fn new(src: impl Into<String>) -> Self {
        Self { src: src.into() }
    }
    pub fn check_input(&self) -> Option<TypeGenSource> {
        Some(TypeGenSource::File(FileSource::new(&self.src)))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn 入力されたsrcからsrcの種類を判定するjson_fine版() {
        let src = "input.json";
        let sut = SourceValidator::new(src);

        assert_eq!(
            sut.check_input().unwrap(),
            TypeGenSource::File(FileSource::new(src))
        );
    }
}

struct ReadSource {}

struct RemoteSource<D, P, M>
where
    D: DeclarePartGenerator<Mapper = M>,
    P: PropertyPartGenerator<M>,
    M: TypeMapper,
{
    generator: TypeDescriptionGenerator<D, P, M>,
    config: RemoteConfig,
}

impl<D, P, M> RemoteSource<D, P, M>
where
    D: DeclarePartGenerator<Mapper = M>,
    P: PropertyPartGenerator<M>,
    M: TypeMapper,
{
    fn create_req(config: &RemoteSourceConfig) -> reqwest::RequestBuilder {
        let req = reqwest::Client::new();
        let req = match &config.method {
            Some(HttpMethod::Get) => req.get(&config.url),
            Some(HttpMethod::Post) => req.post(&config.url),
            None => req.get(&config.url),
        };
        let req = match &config.basic_auth {
            Some(auth) => {
                let username = std::env::var(&auth.username).unwrap();
                let password = std::env::var(&auth.password).unwrap();
                req.basic_auth(username, Some(password))
            }
            None => req,
        };
        let req = match &config.bearer_auth {
            Some(auth) => {
                let token = std::env::var(&auth.token).unwrap();
                req.bearer_auth(token)
            }
            None => req,
        };
        req.header(reqwest::header::CONTENT_TYPE, "application/json")
            .header(reqwest::header::USER_AGENT, "RustProgram")
    }
    async fn gen_all(&self) -> reqwest::Result<()> {
        //for src in &self.config.sources {
        //let req = Self::create_req(src);
        //let res = req.send().await?.text().await?;
        //println!("res : {:#?}", res);
        //let json = serde_json::from_str::<serde_json::Value>(&res)
        //.expect(&format!("Error res :{}", res));
        //let json = Json::from(json);
        //let type_structures = json.into_type_structures(&src.name);
        //let content = self.generator.generate_concat_define(type_structures);
        //let dist_path = format!("{}/{}.{}", s, src.name, self.extension.to_str());
        //let dist_path = PathStructure::new(dist_path, self.extension)
        //.to_snake_path_consider_with_wellknown_words();
        //FileStructer::new(content, dist_path).new_file()
        //}
        Ok(())
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct RemoteConfig {
    sources: Vec<RemoteSourceConfig>,
}

impl RemoteConfig {
    fn from_file_path(path: impl AsRef<Path>) -> std::io::Result<Self> {
        let config = read_to_string(path)?;
        Ok(serde_json::from_str(config.as_str()).unwrap())
    }
    fn from_direct(source: String) -> Result<Self, serde_json::Error> {
        serde_json::from_str(&source)
    }
}
#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct RemoteSourceConfig {
    name: String,
    method: Option<HttpMethod>,
    url: String,
    #[serde(rename = "basicAuth")]
    basic_auth: Option<BasicAuthConfig>,
    #[serde(rename = "bearerAuth")]
    bearer_auth: Option<BearerAuthConfig>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
enum HttpMethod {
    Get,
    Post,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct BasicAuthConfig {
    username: String,
    password: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct BearerAuthConfig {
    token: String,
}

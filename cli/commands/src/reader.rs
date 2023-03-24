use std::{collections::HashMap, fs::read_to_string, path::Path};

use description_generator::{
    type_description_generator::{
        DeclarePartGenerator, PropertyPartGenerator, TypeDescriptionGenerator,
    },
    type_mapper::TypeMapper,
};
use json::json::Json;
use serde::{de::value::Error, Deserialize};
use serde_json::Value;
use sf_df::{extension::Extension, fileconvertor::PathStructure, fileoperator::is_dir};

#[derive(Debug, Clone)]
pub struct SourceValidator {
    src: String,
}
#[derive(Debug, PartialEq, Eq)]
pub enum TypeGenSource {
    File(FileSource),
    Dir(DirSource),
}
impl TypeGenSource {
    pub fn new(src: &str, extension: impl Into<Extension>) -> TypeGenSource {
        if Self::is_dir(src) {
            return TypeGenSource::Dir(DirSource::new_with_extension(src, extension));
        }
        TypeGenSource::File(FileSource::new(src))
    }
    fn is_dir(src: &str) -> bool {
        is_dir(src)
    }
    fn from_config_str(s: &str) -> Result<Self, serde_json::Error> {
        let value = serde_json::from_str::<Value>(s)?;
        match value.get("src") {
            Some(value) => {
                let root = match value.get("root") {
                    Some(Value::String(s)) => s.as_str(),
                    _ => todo!(),
                };
                let extension = value
                    .get("extension")
                    .map(|v| match v {
                        Value::String(s) => s.as_str(),
                        _ => todo!(),
                    })
                    .unwrap_or_else(|| "json");
                Ok(TypeGenSource::new(root, extension))
            }
            None => todo!(),
        }
    }
    //fn new(src: &str) -> Self {
}

//#[derive(Debug, PartialEq, Eq)]
//pub struct ConfigSource {
//src: TypeGenSource,
//}
//impl ConfigSource {
//fn from_str(s: &str) -> Result<Self, serde_json::Error> {
//let value = serde_json::from_str::<Value>(s)?;
//match value.get("src") {
//Some(value) => {
//let Some(root) = value.get("root").map(|v| v.to_string())else{
//todo!()
//};
//let extension = value
//.get("extension")
//.map(|v| v.to_string())
//.unwrap_or_else(|| "json".to_string());
//Ok(TypeGenSource::new(&root, extension))
//}
//None => todo!(),
//}
//}
////fn new(src: &str) -> Self {
////ConfigSource {}
////}
//}
#[derive(Debug, PartialEq, Eq)]
pub struct DirSource {
    root: String,
    extension: Extension,
}
impl DirSource {
    fn new(src: &str) -> Self {
        DirSource {
            root: src.to_string(),
            extension: Extension::Json,
        }
    }
    fn new_with_extension(src: &str, extension: impl Into<Extension>) -> Self {
        DirSource {
            root: src.to_string(),
            extension: extension.into(),
        }
    }
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
        if self.is_dir() {
            return Some(TypeGenSource::Dir(DirSource::new(&self.src)));
        }
        Some(TypeGenSource::File(FileSource::new(&self.src)))
    }
    fn is_dir(&self) -> bool {
        is_dir(&self.src)
    }
}
#[cfg(test)]
mod tests {
    use sf_df::{extension::Extension, fileconvertor::PathStructure};

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
    #[test]
    fn 入力されたsrcからsrcの種類を判定するdir版() {
        let src = "src";
        let sut = SourceValidator::new(src);

        assert_eq!(
            sut.check_input().unwrap(),
            TypeGenSource::Dir(DirSource::new(src))
        );
    }
    #[test]
    fn 設定ファイルにはsrcの指定が必須() {
        let config_src = r#"{"src":{"root":"./","extension":"json"}}"#;

        let sut = TypeGenSource::from_config_str(config_src).unwrap();

        assert_eq!(
            sut,
            TypeGenSource::Dir(DirSource::new_with_extension("./", "json"))
        );
    }
    //#[test]
    //fn 入力されたsrcからsrcの種類を判定するconfig_file版() {
    //let src = "tg-config.json";
    //let sut = SourceValidator::new(src);

    //assert_eq!(
    //sut.check_input().unwrap(),
    //TypeGenSource::Config(ConfigSource::new(src))
    //);
    //}
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

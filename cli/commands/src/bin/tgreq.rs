use std::{fs::read_to_string, path::Path};

use clap::Parser;
use description_generator::{
    type_description_generator::{
        DeclarePartGenerator, PropertyPartGenerator, TypeDescriptionGenerator,
    },
    type_mapper::TypeMapper,
};
use json::json::Json;
use reqwest::{header::CONTENT_TYPE, Client, RequestBuilder};
use rust::generator_builder::RustTypeDescriptionGeneratorBuilder;
use sf_df::{
    extension::Extension,
    fileconvertor::{FileStructer, PathStructure},
};
use tg::command::Cli;

#[tokio::main]
async fn main() {
    Cli::parse().exec().await;
    //let generator = RustTypeDescriptionGeneratorBuilder::new()
    //.declare_part_pub_all()
    //.build();
    //let req = Requestor::from_path("config.json", generator, "rs");
    //req.gen_all().await.unwrap();
}

struct Requestor<D, P, M>
where
    D: DeclarePartGenerator<Mapper = M>,
    P: PropertyPartGenerator<M>,
    M: TypeMapper,
{
    config: RequestConfig,
    generator: TypeDescriptionGenerator<D, P, M>,
    extension: Extension,
}

impl<D, P, M> Requestor<D, P, M>
where
    D: DeclarePartGenerator<Mapper = M>,
    P: PropertyPartGenerator<M>,
    M: TypeMapper,
{
    fn from_path(
        path: impl AsRef<Path>,
        generator: TypeDescriptionGenerator<D, P, M>,
        extension: impl Into<Extension>,
    ) -> Self {
        let extension = extension.into();
        let config = RequestConfig::from_path(path).unwrap();
        Self::new(config, generator, extension)
    }
    fn new(
        config: RequestConfig,
        generator: TypeDescriptionGenerator<D, P, M>,
        extension: Extension,
    ) -> Self {
        Self {
            config,
            generator,
            extension,
        }
    }

    fn create_req(config: &RequestSourceConfig) -> RequestBuilder {
        let req = Client::new();
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
        req.header(CONTENT_TYPE, "application/json")
            .header(reqwest::header::USER_AGENT, "RustProgram")
    }
    async fn gen_all(&self) -> reqwest::Result<()> {
        for src in &self.config.sources {
            let req = Self::create_req(src);
            let res = req.send().await?.text().await?;
            println!("res : {:#?}", res);
            let json = serde_json::from_str::<serde_json::Value>(&res)
                .expect(&format!("Error res :{}", res));
            let json = Json::from(json);
            let type_structures = json.into_type_structures(&src.name);
            let content = self.generator.generate_concat_define(type_structures);
            let dist_path = format!(
                "{}/{}.{}",
                self.config.dist_root,
                src.name,
                self.extension.to_str()
            );
            let dist_path = PathStructure::new(dist_path, self.extension)
                .to_snake_path_consider_with_wellknown_words();
            FileStructer::new(content, dist_path).new_file()
        }
        Ok(())
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct RequestConfig {
    dist_root: String,
    sources: Vec<RequestSourceConfig>,
}

impl RequestConfig {
    fn from_path(path: impl AsRef<Path>) -> std::io::Result<Self> {
        let config = read_to_string(path)?;
        Ok(serde_json::from_str(config.as_str()).unwrap())
    }
}
#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct RequestSourceConfig {
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

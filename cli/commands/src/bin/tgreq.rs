use std::{fs::read_to_string, path::Path};

use description_generator::{
    type_description_generator::{
        DeclarePartGenerator, PropertyPartGenerator, TypeDescriptionGenerator,
    },
    type_mapper::TypeMapper,
};
use json::json::Json;
use rust::generator_builder::RustTypeDescriptionGeneratorBuilder;
use sf_df::{
    extension::Extension,
    fileconvertor::{FileStructer, PathStructure},
};

#[tokio::main]
async fn main() {
    let generator = RustTypeDescriptionGeneratorBuilder::new()
        .declare_part_pub_all()
        .build();
    let req = Requestor::from_path("config.json", generator, "rs");
    req.gen_all().await.unwrap();
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
    async fn gen_all(&self) -> reqwest::Result<()> {
        for src in &self.config.sources {
            let res = reqwest::Client::new()
                .get(&src.url)
                .header(reqwest::header::USER_AGENT, "Mac")
                .send()
                .await?
                .text()
                .await?;

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
            let dist_path = PathStructure::new(dist_path, self.extension).to_snake_path();
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
    url: String,
    basic_auth: Option<BasicAuthConfig>,
    bearer_auth: Option<BearerAuthConfig>,
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

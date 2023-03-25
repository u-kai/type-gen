use std::{collections::HashMap, fs::read_to_string, path::Path};

use description_generator::{
    type_description_generator::{
        DeclarePartGenerator, PropertyPartGenerator, TypeDescriptionGenerator,
    },
    type_mapper::TypeMapper,
};
use json::json::Json;
use npc::fns::to_pascal;
use serde::{de::value::Error, Deserialize};
use serde_json::Value;
use sf_df::{
    extension::{self, Extension},
    fileconvertor::{FileStructer, PathStructure},
    fileoperator::{all_file_structure, is_dir},
};

// dist に必要なもの
// 変換先言語のGenerator
// Vec<TypeStructure>
// Vec<PathStructure>

// TypeStructureにするのは誰の仕事？
// Convertor?
// ConvertorはVec<FileSource>をもらってDistを作る
// 具体的にはdistのルートと
// FileSourceからinto_type_structuresを作成できるのではないか？
// 一旦Jsonだけ気にしてみる
#[derive(Debug, PartialEq, Eq)]
pub enum TypeGenDist {
    File(FileDist),
    Dir(DirDist),
}
impl TypeGenDist {
    pub fn new(path: &str, extension: impl Into<Extension>) -> Self {
        if Self::is_dir(path) {
            return Self::Dir(DirDist::new_with_extension(path, extension));
        }
        Self::File(FileDist::new(path, extension))
    }
    pub fn distribution(&self, content: String) {}
    fn extension(&self) -> Extension {
        match self {
            Self::File(f) => f.extension,
            Self::Dir(d) => d.extension,
        }
    }
    fn is_dir(path: &str) -> bool {
        is_dir(path)
    }
}
#[derive(Debug, PartialEq, Eq)]
pub struct FileDist {
    path: String,
    extension: Extension,
}
impl FileDist {
    fn new(path: impl Into<String>, extension: impl Into<Extension>) -> Self {
        FileDist {
            path: path.into(),
            extension: extension.into(),
        }
    }
}
#[derive(Debug, PartialEq, Eq)]
pub struct DirDist {
    root: String,
    extension: Extension,
}
impl DirDist {
    fn new(src: &str) -> Self {
        DirDist {
            root: src.to_string(),
            extension: Extension::Json,
        }
    }
    fn new_with_extension(src: &str, extension: impl Into<Extension>) -> Self {
        DirDist {
            root: src.to_string(),
            extension: extension.into(),
        }
    }
    //fn to_files(&self) -> Vec<FileDist> {
    //all_file_structure(&self.root, self.extension)
    //.into_iter()
    //.map(|f| FileDist { src: f })
    //.collect()
    //}
}

#[derive(Debug, PartialEq, Eq)]
pub enum TypeGenSource {
    File(FileSource),
    Dir(DirSource),
    Remote(RemoteSource),
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
    fn to_files(&self) -> Vec<FileSource> {
        all_file_structure(&self.root, self.extension)
            .into_iter()
            .map(|f| FileSource { src: f })
            .collect()
    }
}
struct SourceConvertor {
    src: TypeGenSource,
}
impl SourceConvertor {
    fn new(src: TypeGenSource) -> Self {
        Self { src }
    }
    fn convert<D, P, M>(
        &self,
        dist_root: &str,
        generator: &TypeDescriptionGenerator<D, P, M>,
        extension: impl Into<Extension>,
    ) -> Vec<FileStructer>
    where
        D: DeclarePartGenerator<Mapper = M>,
        P: PropertyPartGenerator<M>,
        M: TypeMapper,
    {
        let dist = TypeGenDist::new(dist_root, extension);
        match (&self.src, dist) {
            (TypeGenSource::File(s), TypeGenDist::File(d)) => vec![s.src.to(
                &d.path,
                d.extension,
                Self::file_source_to_type_description(s, generator),
            )],
            (TypeGenSource::Dir(s), TypeGenDist::Dir(d)) => {
                let s_root = &s.root;
                s.to_files()
                    .iter()
                    .map(|s| {
                        s.src.to_dist(
                            s_root,
                            &d.root,
                            d.extension,
                            Self::file_source_to_type_description(s, generator),
                        )
                    })
                    .collect()
            }
            _ => todo!(),
        }
    }
    fn file_source_to_type_description<D, P, M>(
        f: &FileSource,
        generator: &TypeDescriptionGenerator<D, P, M>,
    ) -> String
    where
        D: DeclarePartGenerator<Mapper = M>,
        P: PropertyPartGenerator<M>,
        M: TypeMapper,
    {
        let json = Json::from(f.src.content());
        let type_structure = json.into_type_structures(to_pascal(f.src.name_without_extension()));
        generator.generate_concat_define(type_structure)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct FileSource {
    src: FileStructer,
}
impl FileSource {
    fn new(src: &str) -> Self {
        FileSource {
            src: FileStructer::from_path(src),
        }
    }
}
#[cfg(test)]
mod tests {
    use rust::generator_builder::RustTypeDescriptionGeneratorBuilder;
    use sf_df::{
        extension::Extension, fileconvertor::PathStructure, fileoperator::create_new_file,
    };

    #[test]
    fn url_to_dir() {}
    #[test]
    fn url_to_file() {}
    #[test]
    fn dir_to_dir() {}
    #[test]
    fn dir_to_file() {}
    #[test]
    fn file_to_dir() {}
    #[test]
    fn file_to_file() {}
    use super::*;
    #[test]
    #[ignore = "because create file"]
    fn convertorはdir_sourceからtype_structuerの配列を生成できる() {
        let src = "test-root";
        let mut ope = TestDirectoryOperator::new();
        let child1 = "test-root/test.json";
        let child2 = "test-root/child/child.json";
        ope.clean_up_before_test(src);
        ope.prepare_file(child1, r#"{"test":"Hello"}"#);
        ope.prepare_file(child2, r#"{"child":"World"}"#);
        let src = TypeGenSource::new(src, "json");
        let sut = SourceConvertor::new(src);
        assert_eq!(
            sut.convert(
                "dist",
                &RustTypeDescriptionGeneratorBuilder::new().build(),
                "rs"
            ),
            vec![
                FileStructer::new(
                    "struct Test {\n    test: String,\n}",
                    PathStructure::new("dist/test.rs", "rs")
                ),
                FileStructer::new(
                    "struct Child {\n    child: String,\n}",
                    PathStructure::new("dist/child/child.rs", "rs")
                ),
            ]
        );
        ope.clean_up();
    }
    #[test]
    #[ignore = "because create file"]
    fn convertorはfile_sourceからtype_structuerの配列を生成できる() {
        let src = "input.json";
        let mut ope = TestDirectoryOperator::new();
        ope.clean_up_before_test(src);
        ope.prepare_file(src, r#"{"test":"Hello"}"#);
        let src = TypeGenSource::new(src, "json");
        let sut = SourceConvertor::new(src);
        assert_eq!(
            sut.convert(
                "test.rs",
                &RustTypeDescriptionGeneratorBuilder::new().build(),
                "rs"
            ),
            vec![FileStructer::new(
                "struct Input {\n    test: String,\n}",
                PathStructure::new("test.rs", "rs")
            )]
        );
        ope.clean_up();
    }
    #[test]
    #[ignore = "because create file"]
    fn 入力されたsrcからsrcの種類を判定するjson_fine版() {
        let src = "input.json";
        let mut ope = TestDirectoryOperator::new();
        ope.clean_up_before_test(src);
        ope.prepare_file(src, r#"{"test":"Hello"}"#);
        let sut = TypeGenSource::new(src, "json");
        assert_eq!(sut, TypeGenSource::File(FileSource::new(src)));
        ope.clean_up();
    }
    #[ignore = "because create file"]
    #[test]
    fn 入力されたsrcからsrcの種類を判定するdir版() {
        let src = "test-root";
        let mut ope = TestDirectoryOperator::new();
        let child1 = "test-root/test.json";
        let child2 = "test-root/child/child.json";
        ope.clean_up_before_test(src);
        ope.prepare_file(child1, r#"{"test":"Hello"}"#);
        ope.prepare_file(child2, r#"{"child":"World"}"#);
        let sut = TypeGenSource::new(src, "json");
        assert_eq!(
            sut,
            TypeGenSource::Dir(DirSource::new_with_extension(src, "json"))
        );
        match sut {
            TypeGenSource::Dir(d) => {
                assert_eq!(
                    d.to_files(),
                    vec![FileSource::new(child1), FileSource::new(child2)]
                )
            }
            _ => panic!(),
        }
        ope.clean_up();
    }
    #[test]
    fn 設定ファイルにはsrcかr_srcの指定が必須() {
        let config_src = r#"{"src":{"root":"./","extension":"json"}}"#;

        let sut = TypeGenSource::from_config_str(config_src).unwrap();

        assert_eq!(
            sut,
            TypeGenSource::Dir(DirSource::new_with_extension("./", "json"))
        );

        //let config_src = r#"{
        //"r_src":[
        //{
        //"name": "JsonPlaceHolder",
        //"url": "https://jsonplaceholder.typicode.com/posts/1"
        //},
        //{
        //"name": "GitHubRateLimit",
        //"url": "https://api.github.com/rate_limit",
        //"basicAuth": {
        //"username": "GITHUB_OAUTH_CLIENT_ID",
        //"password": "GITHUB_OAUTH_CLIENT_SECRET"
        //}
        //}
        //]
        //}"#;

        //let sut = TypeGenSource::from_config_str(config_src).unwrap();

        //assert_eq!(
        //sut,
        //TypeGenSource::Dir(DirSource::new_with_extension("./", "json"))
        //);
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
    pub struct TestDirectoryOperator {
        paths: Vec<String>,
    }
    impl TestDirectoryOperator {
        pub fn new() -> Self {
            Self { paths: Vec::new() }
        }
        pub fn remove_dir_all(&self, root: &str) {
            std::fs::remove_dir_all(root).unwrap_or_default();
        }
        pub fn clean_up_before_test(&self, root: &str) {
            let path: &Path = root.as_ref();
            if path.is_file() {
                self.remove_file(root);
                return;
            }
            std::fs::remove_dir_all(root).unwrap_or_default();
        }
        pub fn prepare_file(&mut self, path: impl Into<String>, content: impl Into<String>) {
            let path = path.into();
            let content = content.into();
            create_new_file(path.clone(), content.clone());
            self.paths.push(path);
        }
        #[allow(unused)]
        pub fn assert_exist(&mut self, path: impl Into<String>) {
            let path = path.into();
            assert!(Path::new(&path).exists());
            self.paths.push(path);
        }
        pub fn assert_exist_with_content(
            &mut self,
            path: impl Into<String>,
            content: impl Into<String>,
        ) {
            let path = path.into();
            let content = content.into();
            assert!(Path::new(&path).exists());
            assert_eq!(read_to_string(&path).unwrap(), content);
            self.paths.push(path);
        }
        #[allow(unused)]
        pub fn remove_file(&self, file_name: &str) {
            std::fs::remove_file(file_name).unwrap_or_default();
        }
        pub fn clean_up(self) {
            self.paths
                .into_iter()
                .for_each(|p| std::fs::remove_file(p).unwrap_or_default())
        }
        #[allow(unused)]
        pub fn prepare_test_json_file(&mut self, json_path: &str) {
            self.clean_up_before_test(json_path);
            self.prepare_file(
                format!("{}/test.json", json_path),
                r#"
            {
              "id": 0,
              "name": "kai",
              "obj": {
                "from": "kanagawa",
                "now": "????",
                "age": 20
              }
            }"#,
            );
            self.prepare_file(
                format!("{}/nests/test-child.json", json_path),
                r#"
            {
              "id": 0,
              "child": [
                {
                  "hello": "world"
                }
              ]
            }
        "#,
            );
            self.prepare_file(format!("{}/nests/child/json-placeholder.json",json_path), r#"
            [
              {
                "userId": 1,
                "id": 1,
                "title": "sunt aut facere repellat provident occaecati excepturi optio reprehenderit",
                "body": "quia et suscipit\nsuscipit recusandae consequuntur expedita et cum\nreprehenderit molestiae ut ut quas totam\nnostrum rerum est autem sunt rem eveniet architecto"
              },
              {
                "userId": 1,
                "id": 2,
                "title": "qui est esse",
                "body": "est rerum tempore vitae\nsequi sint nihil reprehenderit dolor beatae ea dolores neque\nfugiat blanditiis voluptate porro vel nihil molestiae ut reiciendis\nqui aperiam non debitis possimus qui neque nisi nulla"
              }
            ] 
        "#);
            self.prepare_file(
                format!("{}/nests/child/array.json", json_path),
                r#"
            [
              {
                "id": 0,
                "greet": "Hello",
                "arr": [
                  {
                    "data": {
                      "id": 0
                    }
                  }
                ]
              }
            ]
        "#,
            );
        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq)]
pub struct RemoteSource {
    sources: Vec<RemoteSourceConfig>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq)]
struct RemoteSourceConfig {
    name: String,
    method: Option<HttpMethod>,
    url: String,
    #[serde(rename = "basicAuth")]
    basic_auth: Option<BasicAuthConfig>,
    #[serde(rename = "bearerAuth")]
    bearer_auth: Option<BearerAuthConfig>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq)]
enum HttpMethod {
    Get,
    Post,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq)]
struct BasicAuthConfig {
    username: String,
    password: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq)]
struct BearerAuthConfig {
    token: String,
}

//struct RemoteSource<D, P, M>
//where
//D: DeclarePartGenerator<Mapper = M>,
//P: PropertyPartGenerator<M>,
//M: TypeMapper,
//{
//generator: TypeDescriptionGenerator<D, P, M>,
//config: RemoteConfig,
//}

//impl<D, P, M> RemoteSource<D, P, M>
//where
//D: DeclarePartGenerator<Mapper = M>,
//P: PropertyPartGenerator<M>,
//M: TypeMapper,
//{
//fn create_req(config: &RemoteSourceConfig) -> reqwest::RequestBuilder {
//let req = reqwest::Client::new();
//let req = match &config.method {
//Some(HttpMethod::Get) => req.get(&config.url),
//Some(HttpMethod::Post) => req.post(&config.url),
//None => req.get(&config.url),
//};
//let req = match &config.basic_auth {
//Some(auth) => {
//let username = std::env::var(&auth.username).unwrap();
//let password = std::env::var(&auth.password).unwrap();
//req.basic_auth(username, Some(password))
//}
//None => req,
//};
//let req = match &config.bearer_auth {
//Some(auth) => {
//let token = std::env::var(&auth.token).unwrap();
//req.bearer_auth(token)
//}
//None => req,
//};
//req.header(reqwest::header::CONTENT_TYPE, "application/json")
//.header(reqwest::header::USER_AGENT, "RustProgram")
//}
//async fn gen_all(&self) -> reqwest::Result<()> {
////for src in &self.config.sources {
////let req = Self::create_req(src);
////let res = req.send().await?.text().await?;
////println!("res : {:#?}", res);
////let json = serde_json::from_str::<serde_json::Value>(&res)
////.expect(&format!("Error res :{}", res));
////let json = Json::from(json);
////let type_structures = json.into_type_structures(&src.name);
////let content = self.generator.generate_concat_define(type_structures);
////let dist_path = format!("{}/{}.{}", s, src.name, self.extension.to_str());
////let dist_path = PathStructure::new(dist_path, self.extension)
////.to_snake_path_consider_with_wellknown_words();
////FileStructer::new(content, dist_path).new_file()
////}
//Ok(())
//}
//}
//impl RemoteConfig {
//fn from_file_path(path: impl AsRef<Path>) -> std::io::Result<Self> {
//let config = read_to_string(path)?;
//Ok(serde_json::from_str(config.as_str()).unwrap())
//}
//fn from_direct(source: String) -> Result<Self, serde_json::Error> {
//serde_json::from_str(&source)
//}
//}

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

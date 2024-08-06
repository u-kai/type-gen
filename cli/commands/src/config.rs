use std::{fs::read_to_string, path::Path};

use description_generator::{
    type_description_generator::{
        DeclarePartGenerator, PropertyPartGenerator, TypeDescriptionGenerator,
    },
    type_mapper::TypeMapper,
};
use json::json::Json;
use npc::fns::to_pascal;
use reqwest::RequestBuilder;
use serde_json::Value;
use sf_df::{
    extension::Extension,
    fileconvertor::{FileStructure, PathStructure},
    fileoperator::{all_file_structure, is_dir},
};

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
    fn new_with_extension(src: &str, extension: impl Into<Extension>) -> Self {
        DirDist {
            root: src.to_string(),
            extension: extension.into(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct InlineSource {
    content: String,
    name: String,
}
impl InlineSource {
    pub fn new(content: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            name: name.into(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TypeGenSource {
    Inline(InlineSource),
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
    pub fn new_inline(content: &str, name: &str) -> TypeGenSource {
        TypeGenSource::Inline(InlineSource::new(content, name))
    }
    pub fn from_config_file(file: impl AsRef<Path>) -> Result<Self, String> {
        match read_to_string(&file) {
            Ok(s) => match RemoteSource::from_path(&file) {
                Ok(r) => Ok(Self::Remote(r)),
                Err(_) => match Self::from_config_str(&s) {
                    Ok(s) => Ok(s),
                    Err(e) => Err(e.to_string()),
                },
            },
            Err(e) => Err(e.to_string()),
        }
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
}
#[derive(Debug, PartialEq, Eq)]
pub struct DirSource {
    root: String,
    extension: Extension,
}
impl DirSource {
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
pub struct SourceConvertor {
    src: TypeGenSource,
}
impl SourceConvertor {
    pub fn new(src: TypeGenSource) -> Self {
        Self { src }
    }
    pub fn console<D, P, M>(&self, generator: &TypeDescriptionGenerator<D, P, M>)
    where
        D: DeclarePartGenerator<Mapper = M>,
        P: PropertyPartGenerator<M>,
        M: TypeMapper,
    {
        if let TypeGenSource::Inline(s) = &self.src {
            let json = Json::from(s.content.as_str());
            let type_description = Self::json_to_type_description(json, &s.name, generator);
            println!("{}", type_description);
        }
    }
    pub async fn convert<D, P, M>(
        &self,
        dist_root: &str,
        generator: &TypeDescriptionGenerator<D, P, M>,
        extension: impl Into<Extension>,
    ) -> Vec<FileStructure>
    where
        D: DeclarePartGenerator<Mapper = M>,
        P: PropertyPartGenerator<M>,
        M: TypeMapper,
    {
        let dist = TypeGenDist::new(dist_root, extension);
        match (&self.src, dist) {
            (TypeGenSource::File(s), TypeGenDist::File(d)) => Self::file_to_file(s, d, generator),
            (TypeGenSource::File(s), TypeGenDist::Dir(d)) => Self::file_to_dir(s, d, generator),
            (TypeGenSource::Dir(s), TypeGenDist::Dir(d)) => Self::dir_to_dir(s, d, generator),
            (TypeGenSource::Dir(s), TypeGenDist::File(d)) => Self::dir_to_file(s, d, generator),
            (TypeGenSource::Remote(s), TypeGenDist::Dir(d)) => {
                Self::remote_to_dir(s, d, generator).await
            }
            (TypeGenSource::Inline(s), TypeGenDist::File(d)) => {
                Self::inline_to_file(s, d, generator)
            }
            (TypeGenSource::Inline(_s), TypeGenDist::Dir(_d)) => todo!(),
            (TypeGenSource::Remote(_s), TypeGenDist::File(_d)) => todo!(),
        }
    }
    async fn remote_to_dir<D, P, M>(
        s: &RemoteSource,
        d: DirDist,
        generator: &TypeDescriptionGenerator<D, P, M>,
    ) -> Vec<FileStructure>
    where
        D: DeclarePartGenerator<Mapper = M>,
        P: PropertyPartGenerator<M>,
        M: TypeMapper,
    {
        let mut result = Vec::new();
        for s in &s.sources {
            let dist_path = format!("{}/{}.{}", &d.root, &s.name, d.extension.to_str());
            let dist_path = PathStructure::new(dist_path, d.extension)
                .to_snake_path_consider_with_wellknown_words();
            result.push(Self::remote_to_file_structure(s, dist_path, generator).await);
        }
        result
    }
    async fn remote_to_file_structure<D, P, M>(
        s: &RemoteSourceConfig,
        dist_path: PathStructure,
        generator: &TypeDescriptionGenerator<D, P, M>,
    ) -> FileStructure
    where
        D: DeclarePartGenerator<Mapper = M>,
        P: PropertyPartGenerator<M>,
        M: TypeMapper,
    {
        let client = RemoteClient::new();
        let res = client.fetch(s).await.unwrap();
        let content = Self::json_to_type_description(res, &s.name, generator);
        FileStructure::new(content, dist_path)
    }

    fn inline_to_file<D, P, M>(
        s: &InlineSource,
        d: FileDist,
        generator: &TypeDescriptionGenerator<D, P, M>,
    ) -> Vec<FileStructure>
    where
        D: DeclarePartGenerator<Mapper = M>,
        P: PropertyPartGenerator<M>,
        M: TypeMapper,
    {
        let json = Json::from(s.content.as_str());
        let type_description = Self::json_to_type_description(json, &s.name, generator);
        vec![FileStructure::new(
            type_description,
            PathStructure::from_path(&d.path),
        )]
    }
    fn dir_to_file<D, P, M>(
        s: &DirSource,
        d: FileDist,
        generator: &TypeDescriptionGenerator<D, P, M>,
    ) -> Vec<FileStructure>
    where
        D: DeclarePartGenerator<Mapper = M>,
        P: PropertyPartGenerator<M>,
        M: TypeMapper,
    {
        let contents = s
            .to_files()
            .iter()
            .map(|f| Self::file_source_to_type_description(f, generator))
            .reduce(|acc, cur| format!("{}\n{}", acc, cur))
            .unwrap_or_default();
        vec![FileStructure::new(
            contents,
            PathStructure::from_path(&d.path),
        )]
    }
    fn dir_to_dir<D, P, M>(
        s: &DirSource,
        d: DirDist,
        generator: &TypeDescriptionGenerator<D, P, M>,
    ) -> Vec<FileStructure>
    where
        D: DeclarePartGenerator<Mapper = M>,
        P: PropertyPartGenerator<M>,
        M: TypeMapper,
    {
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
    fn file_to_dir<D, P, M>(
        s: &FileSource,
        d: DirDist,
        generator: &TypeDescriptionGenerator<D, P, M>,
    ) -> Vec<FileStructure>
    where
        D: DeclarePartGenerator<Mapper = M>,
        P: PropertyPartGenerator<M>,
        M: TypeMapper,
    {
        vec![s.src.to(
            &d.root,
            d.extension,
            Self::file_source_to_type_description(s, generator),
        )]
    }
    fn file_to_file<D, P, M>(
        s: &FileSource,
        d: FileDist,
        generator: &TypeDescriptionGenerator<D, P, M>,
    ) -> Vec<FileStructure>
    where
        D: DeclarePartGenerator<Mapper = M>,
        P: PropertyPartGenerator<M>,
        M: TypeMapper,
    {
        vec![s.src.to(
            &d.path,
            d.extension,
            Self::file_source_to_type_description(s, generator),
        )]
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
        Self::json_to_type_description(f.src.content(), f.src.name_without_extension(), generator)
    }
    fn json_to_type_description<D, P, M>(
        json: impl Into<Json>,
        name: &str,
        generator: &TypeDescriptionGenerator<D, P, M>,
    ) -> String
    where
        D: DeclarePartGenerator<Mapper = M>,
        P: PropertyPartGenerator<M>,
        M: TypeMapper,
    {
        let json = json.into();
        let type_structure = json.into_type_structures(to_pascal(name));
        generator.generate_concat_define(type_structure)
    }
}
struct RemoteClient {}
impl RemoteClient {
    fn new() -> Self {
        Self {}
    }
    fn create_req(config: &RemoteSourceConfig) -> RequestBuilder {
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
    async fn fetch(&self, source: &RemoteSourceConfig) -> reqwest::Result<String> {
        let req = Self::create_req(source);
        let res = req.send().await?.text().await?;
        Ok(res)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct FileSource {
    src: FileStructure,
}
impl FileSource {
    fn new(src: &str) -> Self {
        FileSource {
            src: FileStructure::from_path(src),
        }
    }
}
#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq)]
pub struct RemoteSource {
    sources: Vec<RemoteSourceConfig>,
}

impl RemoteSource {
    fn from_path(file: impl AsRef<Path>) -> Result<Self, String> {
        match read_to_string(file) {
            Ok(file) => match serde_json::from_str(&file) {
                Ok(s) => Ok(s),
                Err(e) => Err(e.to_string()),
            },
            Err(e) => Err(e.to_string()),
        }
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone)]
struct RemoteSourceConfig {
    name: String,
    method: Option<HttpMethod>,
    url: String,
    #[serde(rename = "basicAuth")]
    basic_auth: Option<BasicAuthConfig>,
    #[serde(rename = "bearerAuth")]
    bearer_auth: Option<BearerAuthConfig>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Copy)]
enum HttpMethod {
    Get,
    Post,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone)]
struct BasicAuthConfig {
    username: String,
    password: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone)]
struct BearerAuthConfig {
    token: String,
}
#[cfg(test)]
mod tests {
    use std::{fs::read_to_string, path::Path};

    use rust::generator_builder::RustTypeDescriptionGeneratorBuilder;
    use sf_df::{fileconvertor::PathStructure, fileoperator::create_new_file};

    #[test]
    fn url_to_dir() {}
    #[test]
    fn url_to_file() {}
    use super::*;
    #[tokio::test]
    #[ignore = "because create file"]
    async fn convertorはdir_sourceからtype_structuerの要素が一つの配列を生成できる() {
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
                "dist/test.rs",
                &RustTypeDescriptionGeneratorBuilder::new().build(),
                "rs"
            )
            .await,
            vec![FileStructure::new(
                "struct Test {\n    test: String,\n}\nstruct Child {\n    child: String,\n}",
                PathStructure::new("dist/test.rs", "rs")
            ),]
        );
        ope.clean_up();
    }
    #[tokio::test]
    #[ignore = "because create file"]
    async fn convertorはdir_sourceからtype_structuerの配列を生成できる() {
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
            )
            .await,
            vec![
                FileStructure::new(
                    "struct Test {\n    test: String,\n}",
                    PathStructure::new("dist/test.rs", "rs")
                ),
                FileStructure::new(
                    "struct Child {\n    child: String,\n}",
                    PathStructure::new("dist/child/child.rs", "rs")
                ),
            ]
        );
        ope.clean_up();
    }
    #[tokio::test]
    #[ignore = "because create file"]
    async fn convertorはfile_sourceからtype_structuerの配列を生成できる() {
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
            )
            .await,
            vec![FileStructure::new(
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
    fn 入力されたsrcからsrcの種類を判定するinline版() {
        let sut = TypeGenSource::new(r#"{"key":"value"}"#, "json");
        assert_eq!(
            sut,
            TypeGenSource::Inline(InlineSource {
                content: String::from(r#"{"key":"value"}"#),
                name: "Test".to_string()
            })
        );
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
    }
    pub struct TestDirectoryOperator {
        paths: Vec<String>,
    }
    impl TestDirectoryOperator {
        pub fn new() -> Self {
            Self { paths: Vec::new() }
        }
        #[allow(unused)]
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
        #[allow(unused)]
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

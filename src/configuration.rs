use serde::Deserialize;

#[derive(Deserialize)]
pub struct Configuration {
    pub application_port: u16,
    pub database: DatabaseConfig,
}

#[derive(Deserialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub db_name: String,
}

impl DatabaseConfig {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{username}:{password}@{host}:{port}/{db_name}",
            username = self.username,
            password = self.password,
            host = self.host,
            port = self.port,
            db_name = self.db_name
        )
    }

    pub fn connection_string_without_db(&self) -> String {
        format!(
            "postgres://{username}:{password}@{host}:{port}",
            username = self.username,
            password = self.password,
            host = self.host,
            port = self.port,
        )
    }
}

pub fn get_configuration() -> Result<Configuration, config::ConfigError> {
    let mut configuration = config::Config::default();
    configuration.merge(config::File::with_name("configuration"))?;
    configuration.try_into()
}

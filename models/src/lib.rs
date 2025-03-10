pub mod auth;
pub mod users;

pub mod static_env {
    use dotenvy_macro::dotenv;
    pub const OPENAPI_TITLE: &'static str = dotenv!("OPENAPI_TITLE");
    pub const SERVER_BIND_ADDR: &'static str = dotenv!("SERVER_BIND_ADDR");
    pub const VITE_API_BASE_URL: &'static str = dotenv!("VITE_API_BASE_URL");
    pub const DATABASE_URL: &'static str = dotenv!("DATABASE_URL");
    pub const JWT_SECRET: &'static str = dotenv!("JWT_SECRET");
}

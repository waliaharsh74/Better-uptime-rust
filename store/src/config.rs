use std::env;

pub struct Config{
    pub database_url:String
}
impl Default for Config{
    fn default() -> Self {
        let database_url=env::var("DATABASE_URL").expect("DATABASE_URL IS REQUIRED!");
        Self {
             database_url
             }
        

    }
}
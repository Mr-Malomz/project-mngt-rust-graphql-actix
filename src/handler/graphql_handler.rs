use crate::config::mongo;

pub struct Context {
    pub db: mongo::MongoDB
}

impl juniper::Context for Context {
    
}
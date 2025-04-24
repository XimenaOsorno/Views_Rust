mod create; 
mod get;
mod edit;
use actix_web::web::{ServiceConfig, get, post, scope}; 

pub fn to_do_views_factory(app: &mut ServiceConfig) { 
    app.service( 
        scope("v1/item") 
        .route("create/pending/{title}", post().to(create::create)) 
        .route("create/done/{title}", post().to(create::create_done)) 
        .route("get", get().to(get::get)) 
        .route("edit", post().to(edit::edit)) // define view and URL
    ); 
}
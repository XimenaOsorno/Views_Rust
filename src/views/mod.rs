mod auth;
mod to_do; //Definir modulo
mod app;


use auth::auth_views_factory;
use actix_web::web::ServiceConfig;
use to_do::to_do_views_factory; // import the factory


pub fn views_factory(app: &mut ServiceConfig) {
    auth_views_factory(app);
    to_do_views_factory(app);
    app::app_views_factory(app);
}


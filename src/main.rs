use rocket::figment::Figment;
use rocket::Config;
use rocket::routes;
pub mod DataBase;
pub mod Controller;
pub mod Model;
pub mod routes;
use tokio::sync::Mutex;
use std::sync::Arc;
use crate::DataBase::student_mdb::StudentDB;
//use crate::Controller::student_controller::{create,display_all,update,delete};


#[rocket::main]
async fn main()->Result<(),rocket::Error>{
    let db=StudentDB::init_db().await;
    let db_m= Arc::new(Mutex::new(db));
    
    let config=Config::figment().merge(("port",8000)).merge(("address","0.0.0.0"));

    let _=rocket::custom(config).manage(db_m)
    //.mount("/", routes![create,display_all,update,delete])
    .mount("/",routes::student_routes::stu_routes())
    .launch()
    .await;
    Ok(())

}


use rocket::Route;
use rocket::routes;
use crate::Controller::student_controller::{create,display_all,update,delete,
    insert_students,display_by_id,update_name};              

pub fn stu_routes()->Vec<Route>{
    routes![create,display_all,update,delete,insert_students,display_by_id,update_name]
    
}
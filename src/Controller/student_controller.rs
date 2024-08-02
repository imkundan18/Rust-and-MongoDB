use crate::DataBase::student_mdb::StudentDB;
use crate::Model::student_model::Student;
use futures::StreamExt;
use rocket::serde::{Serialize,Deserialize};
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{doc, to_bson};
use mongodb::results::{DeleteResult, InsertOneResult, UpdateResult};
use rocket::http::Status;
use rocket::{delete, get, post, put};
use rocket::{serde::json::Json, State};
use std::sync::Arc;
use tokio::sync::Mutex;

#[post("/insert", data = "<student>")]
pub async fn create(
    student: Json<Student>,
    db: &State<Arc<Mutex<StudentDB>>>,
) -> Result<Json<InsertOneResult>, Status> {
    let students = student.into_inner();
    if students.name.is_empty() {
        return Err(Status::NotAcceptable);
    }
    let db_lock = db.inner().lock().await;
    let result = db_lock.student_collection.insert_one(students, None).await;
    match result {
        Ok(res) => Ok(Json(res)),
        Err(_) => Err(Status::NoContent),
    }
}

#[get("/display/all")]
pub async fn display_all(db: &State<Arc<Mutex<StudentDB>>>) -> Result<Json<Vec<Student>>, Status> {
    // let mut result=match db.student_collection.find(None, None).await{
    //     Ok(result)=>result,
    //     Err(_)=>return Err(Status::NoContent),
    // };
    let db_lock = db.inner().lock().await;
    let mut result = db_lock
        .student_collection
        .find(None, None)
        .await
        .map_err(|_| Status::InternalServerError)?;

    let mut students = Vec::new();
    while let Some(res) = result.next().await {
        match res {
            Ok(rest) => students.push(rest),
            Err(_) => return Err(Status::BadRequest),
        }
    }
    Ok(Json(students))
}

#[put("/update/<id>", data = "<value>")]
pub async fn update(
    id: String,
    value: Json<Student>,
    db: &State<Arc<Mutex<StudentDB>>>,
) -> Result<Json<UpdateResult>, Status> {
    let new_id = id;
    if new_id.is_empty() {
        return Err(Status::BadRequest);
    }
    let new_value = Student {
        id: Some(ObjectId::parse_str(&new_id).unwrap()),
        name: value.name.to_owned(),
        age: value.age.to_owned(),
        marks: value.marks.to_owned(),
    };
    let b_data = to_bson(&new_value).unwrap();
    let filter = doc! {"$set":b_data};
    let obj_id = ObjectId::parse_str(&new_id).unwrap();
    let filter_id = doc! {"_id":obj_id};

    let db_lock = db.inner().lock().await;
    let result = db_lock
        .student_collection
        .update_one(filter_id, filter, None)
        .await;
    match result {
        Ok(res) => Ok(Json(res)),
        Err(_) => Err(Status::BadRequest),
    }
}

#[delete("/delete/<id>")]
pub async fn delete(
    id: String,
    db: &State<Arc<Mutex<StudentDB>>>,
) -> Result<Json<DeleteResult>, Status> {
    let uid = id;
    if uid.is_empty() {
        return Err(Status::BadRequest);
    }

    let filter = doc! {"_id":ObjectId::parse_str(&uid).unwrap()};
    let db_lock = db.inner().lock().await;
    let result = db_lock.student_collection.delete_one(filter, None).await;
    match result {
        Ok(res) => Ok(Json(res)),
        Err(_) => Err(Status::BadRequest),
    }
}

//Insert multiple students Data at a time
#[post("/insert-students", data = "<data>")]
pub async fn insert_students(
    data: Json<Vec<Student>>,
    db: &State<Arc<Mutex<StudentDB>>>,
) -> Result<Json<Vec<InsertOneResult>>, Status> {
    let student_list = data.into_inner();
    let mut students = Vec::new();

    for student in student_list {
        let db_lock = db.inner().lock().await;
        let result = db_lock.student_collection.insert_one(student, None).await;
        match result {
            Ok(res) => {
                students.push(res);
            }

            Err(_) => return Err(Status::BadRequest),
        }
    }
    Ok(Json(students))
}

//Display students data by id
#[get("/display-id/<id>")]
pub async fn display_by_id(
    id: String,
    db: &State<Arc<Mutex<StudentDB>>>,
) -> Result<Json<Vec<Student>>, Status> 
{
    let db_lock = db.inner().lock().await;
    let filter=doc! {"_id":ObjectId::parse_str(&id).unwrap()};

    let mut result = db_lock.student_collection.find(filter, None).await.map_err(|_| Status::BadRequest)?;
    
    let mut students = Vec::new();

    while let Some(res) = result.next().await {
        match res {
            Ok(rest) => students.push(rest),
            Err(_) => return Err(Status::BadRequest),
            }
    }
    Ok(Json(students))
}



//Update name By id
#[derive(Deserialize,Serialize)]
pub struct UpdateName{
    pub name:String,
}

#[put("/update-name/<id>", data = "<value>")]
    pub async fn update_name(id:&str, value:Json<UpdateName>, db:&State<Arc<Mutex<StudentDB>>>)->Result<Json<UpdateResult>,Status>{
        let db_lock=db.inner().lock().await;
    
        let object_id=ObjectId::parse_str(id).unwrap();
        //eprint!("Object 1 {:?} {:?}",id,object_id);
        let filter=doc!{"_id":object_id};

        let find=db_lock.student_collection.find_one(filter.clone(),None).await;
        eprint!("find {:?}",find);

        match find{
            Ok(Some(_))=>{
                let updat=db_lock.student_collection.update_one(filter,doc!{"$set":{"name":&value.name}},None).await;
                //eprint!("update {:?}",updat);
                match updat{
                    Ok(res)=>
                        Ok(Json(res)),
                        Err(_)=>{
                //eprint!("Error 2  {:?}",e);
                Err(Status::BadRequest)
            }
                }
            }
            Ok(None)=>Err(Status::BadRequest),
            Err(_)=>Err(Status::BadRequest),
        }
    }

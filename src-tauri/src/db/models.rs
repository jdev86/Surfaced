use crate::schema::todos; // refers to the schema file generated by diesel
use serde::{Serialize}; // makes object of this class json serializable, we will convert these objects to json and send to user. 

#[derive(Queryable, Serialize, Debug)] // these annotation adds extra functionality to objects of this struct, Debug is for printing in console `dbg!(todo)`
pub struct Todo {
    pub id: i32,
    pub body: String,
    pub done: bool,
}


#[derive(Insertable, Serialize, Debug, Clone)]
#[table_name = "todos"]
pub struct NewTodo<'a> {  // this struct will be use when inserting into the db, a struct can be Queryable and Insertable at the same time too. 
    pub body: &'a str,
}
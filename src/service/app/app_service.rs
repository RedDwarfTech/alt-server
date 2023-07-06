use diesel::QueryDsl;
use crate::common::database::get_connection;
use crate::diesel::RunQueryDsl;
use crate::{model::diesel::alt::custom_alt_models::AltApp};

pub fn get_app_list(_tag: &String) -> Vec<AltApp>{
    use crate::model::diesel::alt::alt_schema::alt_app as cv_work_table;
    let query = cv_work_table::table.into_boxed::<diesel::pg::Pg>();
    let cvs = query
        .load::<AltApp>(&mut get_connection())
        .expect("error get tags list");
    return cvs;
}


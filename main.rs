mod TableSchema;
use std::time::Instant;

use crate::TableSchema::{Queries::SelectAll, *} ; 


fn main (){

    let mut table : Table<String> = Table::new() ; 
    // sample methods 
    table.add_item("How are you foo ".to_string()) ; 
    table.set("How are you foo ".to_string()) ; 

    // Query assist 
    // the table is moved into QueryTable
    let mut query_assist   = QueryTable::new_query_client(table) ; 

    // see Queries 
    query_assist.query(Queries::AddRecord("NYAMA".to_string())) ; 
    query_assist.query(Queries::Set("NYAMA".to_string())) ; 
    query_assist.query(Queries::Delete( |w| w == "NYAMA")) ; 
    query_assist.query(Queries::AddRecordIfEvery( String::from("Rada bro ") ,|w| w == "NYAMA" )) ; 
    query_assist.query(Queries::AddRecordIfSome( String::from("Rada bro ") ,|w| w == "NYAMA" )) ; 

    // to monito results 
    if let  QueryResults::State(bool) =  query_assist.query(Queries::Set("NYAMA1".to_string()))  {

        if bool { 
            println!("NYAMA1 does not exist ") ; 
            println!("NYAMA1 was set into the table {:?} " , query_assist.move_table()) ;  
        }
    } ; 



    // structural data ------- 

    #[derive(Debug , Clone , PartialEq)]
    struct Activity {
        resource : String  , 
        at_ : Instant 
    } 
    

    // table structure 
    #[derive(Debug , Clone , PartialEq)]
    struct Users {
        name : String , 
        ip_addr : String , 
        id : u8 , 
        username : String , 
        activity : Vec<Activity> 
    }


    let mut user_table : Table<Users> = Table::new() ; 
    let mut query_client : QueryTable<Users> = QueryTable::new_query_client(user_table) ; 


    // add record using query 
    let njorpge =  Users { name: "Njoroge".to_string(), ip_addr: "127.0.0.1:8080".to_string(), id: 43 , username: "Njoroge254".to_string(), activity: Vec::new() } ; 
    query_client.query( Queries::AddRecord(njorpge)) ; 


    // perform multiple query actions 
    query_client.queries(vec![
         Queries::AddRecord(Users { name: "Njoki".to_string(), ip_addr:  "127.0.0.1:8081".to_string(), id:  25 , username: "Njokimemes".to_string(), activity:  vec![] } ,     ),
         Queries::AddRecord(Users { name: "Njoki".to_string(), ip_addr:  "127.0.0.1:8081".to_string(), id:  25 , username: "Njokimemes".to_string(), activity:  vec![] } ,     ),
         Queries::AddRecord(Users { name: "Njoki".to_string(), ip_addr:  "127.0.0.1:8081".to_string(), id:  25 , username: "Njokimemes".to_string(), activity:  vec![] } ,     ),
         
         ]);

    if  let QueryResults::SelectAll(all) = query_client.query(SelectAll(|t| true )) {
    println!("Rcords {:?}" , all )
} 

    
} 



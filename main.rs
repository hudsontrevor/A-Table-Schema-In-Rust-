mod TableSchema;
use crate::TableSchema::{*} ; 


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
    query_assist.query(Queries::AddRecord("NYAMA".to_string())) ; 
    query_assist.query(Queries::Delete( |w| w == "NYAMA")) ; 

    // to monito results 
    if let  QueryResults::State(bool) =  query_assist.query(Queries::Set("NYAMA1".to_string()))  {

        if bool { 
            println!("NYAMA1 does not exist ") ; 
            println!("NYAMA1 was set into the table {:?} " , query_assist.move_table()) ;  
        }
    }
    
} 


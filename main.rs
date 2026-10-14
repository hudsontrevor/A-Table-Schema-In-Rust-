use std::{io::Error, net::*} ;

use crate::Queries::{AddRecord, Set}; 


trait Chesspiece { 
    fn get_p (&self) -> char ; 
}

struct Knight ; 
struct Bishop ;
impl Chesspiece for Knight {
    fn get_p (&self) -> char {
        'C'
    }
}
impl   Chesspiece for Bishop {
    fn get_p (&self) -> char {
        'B'
    }
}


fn main (){

    let mut table : Table<String> = Table::new() ; 
    // sample methods 
    table.add_item("How are you foo ".to_string()) ; 
    table.set("How are you foo ".to_string()) ; 

    // Query assist 
    // the table is moved into QueryTable
    let mut query_assist   = QueryTable::new_query_client(table) ; 

    // see Queries 
    query_assist.query(AddRecord("NYAMA".to_string())) ; 
    query_assist.query(AddRecord("NYAMA".to_string())) ; 
    query_assist.query(Queries::Delete( |w| w == "NYAMA")) ; 

    // to monito results 
    if let  QueryResults::State(bool) =  query_assist.query(Set("NYAMA1".to_string()))  {
        println!("NYAMA1 does not exist ") ; 
        println!("NYAMA1 was set into the table {:?} " , query_assist.table) ; 
    }
    
} 




///Holds the Records`<T>` of the table 
#[derive(Debug)]
struct Table<T> {
    records :  Vec<T>
}



impl<T : Clone> Table<T> where T : PartialEq {

    /// Creates a new Table 
    /// #Args
    /// *start_with - vector of records to push to the Tablel 
    /// 
    fn new_with ( start_with : Vec<T>) -> Table<T> {
        Table { records: start_with }
    }

    /// Creates a new Table 
    fn new () -> Table<T> {
        Table { records: Vec::new() }
    }
    ///Adds an item to the table returns true since modification occurs 
    fn add_item ( &mut self , item : T  ) -> bool   {
        self.records.push(item);
        true 
    }

    /// adds an item to the table if the predicate `wh` returns true for any item in the table 
    /// returns a true if table was modified else false 
    fn add_item_if_some ( & mut self  , item : T , wh : impl Fn(&T) -> bool   ) -> bool {
        let mut state : bool  =  false  ; 
        for item_ in self.records.iter() {
            state = wh(item_) ;
            if state {
                self.records.push(item);
                break  ;
            }} ;state
        }
        
    /// adds an item to the table if the predicate `wh` returns true for every item in the table 
    /// returns a true if table was modified else false 
    fn add_item_if_every ( &mut self , item : &T , wh : impl  Fn(&T) -> bool   ) -> bool {
    for item_ in self.records.iter() {
        if !wh(item_) {
            return  false ;
        }
    } ;
    self.records.push(item.clone()); 
    return  true 

    }

    ///
    /// adds an item if no other item in the table matches its signature 
    /// returns a true if table was modified else false 
    fn set ( & mut self  , item : T ) -> bool {
        self.add_item_if_every(&item, |t| t != &item.clone() )
    }

    /// removes all items whose predicate, `wh`, returns true 
    /// returns a true if table was modified else false 
    fn delete ( & mut  self ,   wh : impl  Fn(&T) -> bool) -> bool  {
        let mut state : bool = false  ; 
        self.records.retain(|element| { let action = wh(element)  ; if action {state = true } ; !action  });
        state 
    }


    /// updates the first instance of an element in the table 
    /// # Arg
    /// * call_back - pottentially can execute on all elements in the vector.In predicate , sorround the update  with an if statement and return true if desired  element is found and modified 
    /// ```
    /// let table : Table<String> = Table::new_with(vec![ "foo" , "bar"])
    /// table.update_once(|item| {if item == "foo" {item = "bar" ; return true} ; return false })
    /// 
    /// ```
    /// returns a true if table was modified else false 
    /// 
    fn update_once ( &mut self , call_back : impl Fn(& mut T) -> bool  ) -> bool {
        let mut state : bool = false ; 
        for record  in self.records.iter_mut() {
            if call_back(record) {
                state = true  ; 
                break;
            }
        } ; 
        state 
    }

    ///executes a call_back to all elements whise predicate, `wh`, returns true 
    /// returns ()
    fn update_all (& mut self , wh : impl  Fn(&T) -> bool,  call_back : impl Fn(&mut T) ) {
        for record in self.records.iter_mut(){if wh(&record) {call_back(record)}} ; 
    }


    /// returns an Option containing  first element in the table whose predicate `wh` returned true 
    fn select_once(&self , wh : impl Fn(&T) -> bool ) -> Option<&T>{
        for item in self.records.iter() {
            if wh(item) {
                return  Some(item);
            }
        } ; 
        None
    }


    /// returns a Vector containing all items in the table whose predicate `wh` returned true 
    fn select_all (&self , wh : impl  Fn(&T) -> bool) -> Vec<&T> {self.records.iter().filter( |t| wh(t)).collect() }


    // takes elements from Vector`<T>` supplied and adds the to the table `<T>`
    fn absorb (&mut self , records : &mut Vec<T> ) {
        self.records.append(records)
    }


    // takes elements from Table`<T>` supplied and adds them to this table`<T>` 
    fn take_from (&mut self , table : &mut Table<T>){
        self.absorb( &mut table.records)
    }
    }



/// A query assist for struct Table 
struct QueryTable<T> {
 table : Table<T>
}

enum QueryResults< 'a  , T> { 
    /// returns the state of the succession  of add_item , add_item_if_some , add_item_if_every , set, update_once and delete methods of the  Table struct that can modify the table 
    State(bool) , 

    /// holds for methods  that return () , UpdateAll , Absord , TakeFrom 
    Void, 


    /// returns the result  of the SelectOnce Query
    SelectOnce(Option<T>) , 
    /// returns the result  of the SelectAll Query 
    SelectAll(Vec<& 'a T>) , 
}


enum Queries<'a, T> { 
    /// ``` 
    /// query  return s` true 
    /// ```
    AddRecord(T) , 
    ///
    /// 
    /// QueryResult returns a true if  Table was modified  else false 
    AddRecordIfSome(T , fn(&T)-> bool  )  , 

    ///
    ///  QueryResult returns a true if  Table was modified  else false 
    AddRecordIfEvery(T , fn(&T)-> bool  )  , 
    ///
    ///  QueryResult returns A Vector af all records whose predicate returns true 
    SelectAll(fn(&T)-> bool ) ,

    /// adds an item if no other item with its signature exists
    /// QueryResult returns a true if  Table was modified  else false 
    Set(T) , 

    ///
    /// removes all  elements whose predicate returns true 
    /// QueryResult returns a true if  Table was modified  else false 
    Delete(fn(&T)-> bool )   , 

    /// updates the first occurence of an item as specified 
    /// QueryResult returns a true if  Table was modified  else false 
    UpdateOnce(fn(&mut T )-> bool ) , 

    ///
    /// updates all elements whose `wh` predicate returns true 
    /// QueryResult returns Void 
    UpdateAll(fn(&T)-> bool , fn(&mut T )) , 
    /// takes all elements in the Vector`<T>` Provided and adds them to the table`<T>`
    /// QueryResult returns Void 
    Absorb( &'a mut Vec<T>) ,

    ///  takes all elements in the Table`<T>` Provided and adds them to the table `<T>`
    /// QueryResult returns Void 
    TakeFrom( &'a mut Table<T>)
}



/// Query Methods 
trait QueryMethods<T> { 
    fn new_query_client (  records :  Table<T> ) -> QueryTable<T> ; 
    fn query ( &mut self , query : Queries<T>) -> QueryResults< '_ , T>; 
    fn move_table (self  ) -> Table<T> ; 
}



impl<T : PartialEq> QueryMethods<T> for QueryTable<T> where T : Clone  { 

    /// Outputs the table supplied to initialize this obj. taking it out of this context 
    fn move_table ( self ) -> Table<T> {
        self.table
    }

    /// Takes in An enum Queries and outputs  an Enum QueryResult 
    fn query ( &mut self ,  query : Queries<T>) -> QueryResults< '_  , T> {
        match query  {
            Queries::AddRecord(record ) => QueryResults::State( self.table.add_item(record)) ,
            Queries::AddRecordIfSome( record ,wh ) => QueryResults::State(self.table.add_item_if_some(record, wh)),
            Queries::AddRecordIfEvery( record , wh ) =>QueryResults::State(self.table.add_item_if_every(&record , wh)),
            Queries::SelectAll(wh) => { QueryResults::SelectAll(self.table.select_all(|t| wh(t)) )},
            Queries::Set( record) => QueryResults::State(self.table.set(record)),
            Queries::Delete(wh) =>QueryResults::State(self.table.delete(wh)),
            Queries::UpdateOnce(wh) => QueryResults::State(self.table.update_once(wh)),
            Queries::UpdateAll( wh , call_back ) => { self.table.update_all(wh, call_back) ;QueryResults::Void },
            Queries::Absorb(items) => { self.table.absorb(items) ; QueryResults::Void },
            Queries::TakeFrom(table) =>{ self.table.take_from(table) ; QueryResults::Void },
        } 
        
        
    }
    /// consumes a Table to initialize A QueryTable obj 
    fn new_query_client (  records : Table<T> ) -> QueryTable<T> {
        QueryTable { table : records}
    }
}
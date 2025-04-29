use std::collections::HashMap;


pub fn getting_started_with_hashmaps() -> HashMap<&'static str, &'static str> {
    let mut users:HashMap<&str, &str>  = HashMap::new();

    users.insert("hi", "bye");
    users.insert("check", "row");
    users.insert("rook", "1");
    println!("{:?}", users.get("rook"));

    return users;
}


pub fn hashmap_with_vectors_from_vector_tupes<'a>(vals: Vec<(&'a str, &'a str)>) -> HashMap<&'a str, Vec<&'a str>> {
    let mut records: HashMap<&str, Vec<&str>> = HashMap::new();
    
    for (key, value) in vals {
        // if !records.contains_key(key) {
        //     records.insert(key, Vec::new());
        // }
        
        // if let Some(current_record) = records.get_mut(key) { 
        //     //get => val isnt mutable and get_mut gives a mutable Option response
        //     current_record.push(value);
        // }

        records.entry(key)
        .or_insert(Vec::new())
        .push(value);
    }
    
    println!("{:?}",records);
    return records;
}

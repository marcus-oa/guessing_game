use std::collections::HashMap;

pub(crate) fn hashmap_example() {

    // example HashMap usage
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // collect and zip example
    // turns two vectors into a tuple
    // turns a set of tuples into a HashMap
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10,50];

    let mut scores: HashMap<_,_> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();

    // For types which implement copy we get a copy when
    // we create a HashMap
    // For types like String (which don't) the new HashMap
    // takes ownership
    let field_name = String::from("Favorite Color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name,field_value); //Note: Can no longer call field_name or field_value after this point

    // Hashmap access example
    let mut scores2 = HashMap::new();

    scores2.insert(String::from("Blue"),10);
    scores2.insert(String::from("Yellow"),50);

    let team_name = String::from("blue");
    // Value here is some as get returns Option<&V>
    let score = scores2.get((&team_name));

    // HashMaps can be iterated over
    for (key,value) in &scores2 {
        println!("{}: {}", key, value);
    }

    // Overwriting hashmap values example
    let mut scores3 = HashMap::new();

    // Results in an overwrite of 10 with 25 for key "Blue"
    scores3.insert(String::from("blue"), 10);
    scores3.insert(String::from("blue"), 25);

    println!("{:?}", scores3);

    // Insert only if key is not bound
    // entry is the API for this action
    let mut scores4 = HashMap::new();
    scores4.insert(String::from("Blue"), 10);

    scores4.entry(String::from("Yellow")).or_insert(50);
    scores4.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores4);

    // Update keys value based on existing (old) value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    // Remember: .or_insert returns a mutable reference to a keys value (&mut V)
    // such that we can update it as we do with the count line below
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}

pub(crate) fn exercises() {
    // come back to these
    // https://doc.rust-lang.org/book/ch08-03-hash-maps.html (Summary)
}
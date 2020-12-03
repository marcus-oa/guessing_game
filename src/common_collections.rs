fn temp() {
    let v: Vec<i32> = Vec::new();

    let v2 = vec![1,2,3];

    let mut mut_vec = Vec::new;

    mut_vec.push(5);
    mut_vec.push(6);
    mut_vec.push(6);
    mut_vec.push(7);
    mut_vec.push(8);

    {
        let v3 = vec![1,2,3,4];
        // do stuff with v
    } // <- v goes out of scope and is freed

    // Vector Access

    let v4 = vec![1,2,3,4,5];

    let third: &i32 = &v[4];
    println!("The third element is {}", third);

    match v4.get(2) {
        Some(third) => println!("the third element is {}",  third);
        None => println!("There is no element")
    }

    // Vector iteration

    let v6 = vec![100,32,57];
    for i in &v {
        println("{}",i);
    }

    let mut v7 = vec![100,32,57];
    for i in &mut v {
        // Dereference operator of * required
       *i += 50;
    }

    // Enums to store multiple values

    enum SpreadsheetCall {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCall::Int(3),
        SpreadsheetCall::Float(10.12),
        SpreadsheetCall::Text(String::from("blue")),
    ];


}
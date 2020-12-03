fn temp() {
    let mut s = String::new();
    let data = "initial contents";

    let s = data.to_string();

    // Direct string literal can be referenced
    let s = "initial contents".to_string();

    // Or more commonly
    let s = String::from("initial contents");

}
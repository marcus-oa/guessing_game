use std::char::from_u32;

pub(crate) fn print_carol() {

    let lyric_array = ["A partridge in a pear tree",
    "Two turtle doves, and",
    "Three french hens",
    "Four calling birds",
    "Five golden rings",
    "Six geese a-laying",
    "Seven swans a-swimming",
    "Eight maids a-milking",
    "Nine ladies dancing",
    "10 lords a-leaping",
    "11 pipers piping",
    "12 drummers drumming"];

    fn to_formatted(i: i32) -> String {
        i.to_string() + "th"
    }

    (1..12).for_each(|line_num| {
        println!("VERSE {}", line_num);
        let formatted_line_num = match line_num {
            1 => "1st".to_string(),
            2 => "2rd".to_string(),
            3 => "3rd".to_string(),
            _ => line_num.to_string() + "th"
        };
        println!("On the {} day of Christmas my true love sent to me", formatted_line_num);
        (1..line_num).rev().for_each(|line_to_print| {
            println!("{}", lyric_array[line_to_print - 1]);
        });
        println!()
    });

}
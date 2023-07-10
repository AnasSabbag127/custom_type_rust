mod structure;
mod enums;
use structure::struct_type;
use crate::enums::enum_example;
use std::env;


fn main() {
    println!("Hello, world!");
    let args:Vec<String> = env::args().collect();
    let topic_list = r#"1.structure
2.Enum
3.constant"#;
    match args[1].as_str(){
        "struct_fun" => {
            struct_type::struct_fun();
        },
        "enum_fun" => {
            enum_example::enum_fun();
        }
        "list" => {
            println!("topic_list: {:?}",topic_list);
        },
        _ => {
            println!("Invalid arguement .");
        }

    }

}

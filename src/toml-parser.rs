fn main() {
    #[derive(Debug)]
    struct TomlValue {
        key:String,
        value:String,
    }

    #[derive(Debug)]
    struct TomlTable {
        name: String,
        values: Vec<TomlValue>
    }
    
    let text = "[ho]\na=how\nb=are\nc=you\n   \n[ho]\na=how\nb=are\nc=you\n";

    let mut currentTable= TomlTable { name:"".to_string(), values: vec![]};
    
    for x in text.lines() {
        let x=x.trim();
        match x {
            x if x.contains("[")=>{
                println!("class name: {}",x);
                currentTable.name=x.to_string().replace("[","").replace("]","");
            },
            x if x.contains("=")=>{
                let key_value:Vec<&str>= x.split("=").collect();
                println!("key: {}, value: {}",key_value[0],key_value[1]);
                currentTable.values.push(TomlValue {key: key_value[0].to_string(),value: key_value[1].to_string()});
            },
            x if (x=="")=> {
                println!("end of table {}",x);
            },
            _=>panic!("non valid file {}",x)
            
        }
    }
    println!("{:?}",currentTable)
    }
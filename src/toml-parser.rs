fn main() {
    #[derive(Debug)]
    struct TomlFile{
        globals: Vec<TomlValue>,
        tables: Vec<TomlTable>,
    }
    #[derive(Debug)]
    struct TomlValue{
        key:String,
        value:String,
    }
    #[derive(Debug)]
    struct TomlTable {
        name: String,
        values: Vec<TomlValue>
    }
    
    let text = "[ho]\na=how\nb=are\nc=you\n   \n[he]\na=how\nb=are\nc=you\n";
    let mut current_file=TomlFile {globals:vec![], tables:vec![]};
    let mut current_table_index= 0;
    
    for x in text.lines() {
        let x=x.trim();
        match x {
            x if x.contains("=")=>{
                if current_table_index==0 {
                    let key_value:Vec<&str>= x.split("=").collect();
                    current_file.globals.push(TomlValue {key: key_value[0].to_string(),value: key_value[1].to_string()})
                } else {
                    let key_value:Vec<&str>= x.split("=").collect();
                    current_file.tables[current_table_index-1].values.push(TomlValue {key: key_value[0].to_string(),value: key_value[1].to_string()});
                }    
            },
            x if x.contains("[")=>{
                let name =x.to_string().replace("[","").replace("]","");
                current_file.tables.push(TomlTable {name:name, values: vec![]});
                current_table_index=current_table_index+1;
    
            },
            x if (x=="")=> {
                println!("empty line {}",x);
            },
            _=>panic!("non valid value {}",x)
            
        }
    }
    println!("{:?}",current_file)
}
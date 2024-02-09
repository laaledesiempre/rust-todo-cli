enum Action {
    add,
    remove,
    mark,
    get,
}
#[derive(Debug)]
struct Todo_action {
    action: Action,
    list: Option<String>,
    todo: Option<String>,
}

fn main() {
    let args: Vec<String>= std::env::args().collect();

    let todo_action = parse_args(&args);
}

// Args manipulation

fn parse_args(args: &Vec<String>)-> Result<Todo_action,String> {
    match args.len() {
        2=>{
            match args[1] { // $ todo get
                String::from("get")=> Ok(Todo_action {
                    action: Action::get,
                    list: None,
                    todo: None
                })
                _=>Err(format!("Can't make a one long with {}!",args[1]));//TODO
            }
        }, 
        3=> {
            match args[1] {
                String::from("add")=> Ok(Todo_action {
                    action: Action::add,
                    list: args[2],
                    todo: None,
                }),
                String::from("remove")=> Ok(Todo_action {
                    action: Action::remove,
                    list: args[2],
                    todo: None,
                }),
                String::from("mark")=> Ok(Todo_action {
                    action: Action::mark,
                    list: args[2],
                    todo: None,       
                }),
                String::from("get")=> Ok(Todo_action {
                    action: Action::get,
                    list: args[2],
                    todo: None
                }),
                _=>Err(format!("Can't make a one long with {}!",args[1]));//TODO
            }
        },
        4=>{
            match args[1] {
                 String::from("add")=> Ok(Todo_action {
                    action: Action::add,
                    list: args[2],
                    todo: args[3],
                }),
                String::from("remove")=> Ok(Todo_action {
                    action: Action::remove,
                    list: args[2],
                    todo: args[3],
                }),
                String::from("mark")=> Ok(Todo_action {
                    action: Action::mark,
                    list: args[2],
                    todo: args[3],       
                }),
                String::from("get")=> Ok(Todo_action {
                    action: Action::get,
                    list: args[2],
                    todo: args[3],
                }),
                _=>Err(format!("Can't make a one long with {}!",args[1]));//TODO               
            }
        }
        _=>Err(String::from("Not one of the program options"))
    }
}

// Todo manipulation
fn todo_get(list: String) {
    todo!("List all todos on a list")
}

fn todo_add(todo: Todo) {
    todo!("Adds a todo to a list")
}

fn todo_mark_as_done(todo: Todo){
    todo!("Mark as done a todo")
}

fn todo_delete(todo: Todo){
    todo!("Delete a todo from the list and renumber all the other todos")
}

// List manipulation
fn list_get() {
    todo!("List all lists")
}

fn list_add(list_name:String) {
    todo!("Creates a new list")
}

fn list_remove(list_name:String) {
    todo!("Remove a whole list")
}
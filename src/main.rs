enum Action {
    add,
    remove,
    mark,
    get,
}

struct Todo_action {
    action: Action,
    list: Option<String>,
    todo: Option<String>,
}

fn main() {
    let args: Vec<String>= std::env::args().collect();

    let todo_action = parse_args(&args);
    //println!("{:?}",args);
    if args[1] == "add" && args.len() >2 {
        println!("Added {}",args[2])
    } else {
        println!("not enough args");
    }
}

// Args manipulation

fn parse_args(args: &Vec<String>)-> Todo_action {
    match args.len() {
        2=>{
            match args[1] { // $ todo get
                String::from("get")=> Todo_action {
                    action: Action::get,
                    list: None,
                    todo: None
                }
                _=>println!("Can't make a one long with {}!",args[1]);
            }
        }, 
        3=> {
            todo!("Parse to remove/add list");
        },
        4=>{
            todo!("Parse the rest of situations");
        }
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
struct Todo {
    list: String;
    todo: String;
}

fn main() {
    let args: Vec<String>= std::env::args().collect();

    //println!("{:?}",args);
    if args[1] == "add" && args.len() >2 {
        println!("Added {}",args[2])
    } else {
        println!("not enough args");
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
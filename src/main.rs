
#[derive(Clone, Debug)]
struct Todo {
    id: u32,
    title: String,
    todo: String,
    start: String,
    complete: bool,
}

struct TodoGroup {
    todo_arr: Vec<Todo>,
    idx:u32,
}


impl TodoGroup {
    fn new() -> Self {
        Self {
            idx: 0,
            todo_arr: Vec::new()
        }
    }

    fn add( &mut self, title: &str, body: &str, start: &str  ) -> u32 {
        self.todo_arr.push( Todo{ 
            id: self.idx,
            title: title.into(),
            todo: body.into(),
            start: start.into(),
            complete: false,
        });
        self.idx += 1;
        self.idx-1
    }

    fn remove( &mut self, idx: u32 ) -> Result<Todo, &str> {
        let i = idx as usize;
        if idx > 0 || i < self.todo_arr.len() {
            self.idx -= 1 as u32;
            Ok(self.todo_arr.swap_remove(i))
        } else {
            Err("Index out of bounds")
        }
    }

    fn retrieve( &self, idx: u32 ) -> Result<Todo, &str> {
        let i = idx as usize;
        if idx > 0 || i < self.todo_arr.len() {
            Ok(self.todo_arr[i].clone())
        } else {
            Err("Index out of bounds")
        }
    }



}

fn main() {

    let mut todo = TodoGroup::new();

    let idx = todo.add( "Test Todo", "This is a wonderful todo", "2022-03-11 08:00" );

    let t = todo.retrieve(idx).unwrap();

    println!("get todo: {:?}", t);
    
    let t = todo.remove(idx).unwrap();

    println!("deleted todo: {:?}", t);

    println!("Hello, world!");
}

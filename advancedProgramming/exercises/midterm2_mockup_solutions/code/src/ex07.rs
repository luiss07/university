// Write a trait `Task` that define a method `execute(&self)->usize`.

// implement the `Task` trait for the following structs:
//  - `SumTask` is a struct with a method `new(n1: usize, n2: usize)` were executing task returns
//  the sum of n1 and n2
//  - LenTask is a struct with a method `new(s: String)` were executing task returns
//  the len of s


// Write two structs: `Tasker` and `Executer`, that interact following this protocol:
//  - At any given time any number of tasker and executer can be linked together.
//  - `Tasker` can ask for a task to be scheduled using the method `schedule_task(&mut self, task: ...)` that take as input a
// box with inside an object that implements Task
//  - `Executer` can execute a task using the method execute_task(&mut self)->Option<usize>. this method can fail if no task is scheduled
//  - Tasks are executed inf a FIFO queue
//  - `Tasker` has a method `new` that return am instance with an empty queue, linked to no one.
//  - `Tasker` has a method `get_tasker(&self)->Tasker` that return a new `Tasker` linked with self.
//  - `Tasker` has a method `get_executer(&self)->Executer` that return a new `Executer` linked with self.



use std::cell::RefCell;
use std::collections::LinkedList;
use std::rc::Rc;

pub trait Task{
    fn execute(&self) -> usize;
}
pub struct SumTask{
    n1: usize,
    n2: usize
}
impl SumTask{
    fn new(n1: usize, n2: usize) -> Self{
        Self{n1,n2}
    }
}
impl Task for SumTask{
    fn execute(&self) -> usize {
        return self.n2+self.n1
    }
}
pub struct LenTask{
    s: String
}

impl LenTask {
    fn new(s: String)->Self{
        Self{s}
    }
}

impl Task for LenTask {
    fn execute(&self) -> usize {
        self.s.len()
    }
}

struct TaskQueue{
    v: LinkedList<Box<dyn Task>>
}


impl TaskQueue {
    fn push(&mut self, e: Box<dyn Task>){
        self.v.push_back(e)
    }

    fn pop(&mut self) -> Option<Box<dyn Task>>{
        self.v.pop_front()
    }

    fn new() -> Self{
        Self{
            v: LinkedList::new()
        }
    }
}

pub struct Tasker {
    queue: Rc<RefCell<TaskQueue>>
}
impl Tasker{
    pub fn new()->Self{
        Self{
            queue: Rc::new(RefCell::new(TaskQueue::new()))
        }
    }
    pub fn get_tasker(&self)->Tasker{
        Self{
            queue: self.queue.clone()
        }
    }
    pub fn get_executer(&self)->Executer{
        Executer{
            queue: self.queue.clone()
        }
    }
    pub fn schedule_task(&mut self, task: Box<dyn Task>){
        self.queue.borrow_mut().push(task);
    }
}

pub struct Executer {
    queue: Rc<RefCell<TaskQueue>>
}

impl Executer {
    fn execute_task(&mut self)->Option<usize>{
        let t = self.queue.borrow_mut().pop()?;
        Some(t.execute())
    }
}
// ---


#[test]
fn test(){

    macro_rules! sum_task {
        (let $task: ident =$n1: literal + $n2: literal) => {
            let $task: Box<dyn Task> = Box::new(SumTask::new($n1, $n2));
        };
    }

    sum_task!(let t1 = 1+1);
    sum_task!(let t2 = 2+2);
    sum_task!(let t3 = 3+3);
    sum_task!(let t4 = 4+4);
    sum_task!(let t5 = 5+5);
    sum_task!(let t6 = 6+6);
    sum_task!(let t7 = 7+7);

    let mut tasker = Tasker::new();
    let mut executer = tasker.get_executer();

    println!("{:?}",executer.execute_task());

    tasker.schedule_task(t1);
    tasker.schedule_task(t2);

    println!("{:?}",executer.execute_task());

    tasker.schedule_task(t3);
    tasker.schedule_task(t4);
    tasker.schedule_task(t5);
    tasker.schedule_task(t6);
    tasker.schedule_task(t7);

    println!("{:?}",executer.execute_task());
    println!("{:?}",executer.execute_task());
    println!("{:?}",executer.execute_task());
    println!("{:?}",executer.execute_task());
    println!("{:?}",executer.execute_task());
    println!("{:?}",executer.execute_task());
    println!("{:?}",executer.execute_task());
}
/*
MARK 1
None
Some(2)
Some(4)
Some(6)
Some(8)
Some(10)
Some(12)
Some(14)
None
*/
#[test]
fn test2(){

    macro_rules! sum_task {
        (let $task: ident =$n1: literal + $n2: literal) => {
            let $task: Box<dyn Task> = Box::new(SumTask::new($n1, $n2));
        };
    }
    macro_rules! len_task {
        (let $task: ident =$s: literal) => {
            let $task: Box<dyn Task> = Box::new(LenTask::new($s.to_owned()));
        };
    }


    sum_task!(let t1 = 10+1);
    len_task!(let t2 = "four");
    let mut tasker1 = Tasker::new();
    let mut tasker2 = tasker1.get_tasker();
    let mut executer1 = tasker2.get_executer();
    let mut executer2 = tasker1.get_executer();
    tasker1.schedule_task(t1);
    tasker2.schedule_task(t2);
    println!("{:?}",executer1.execute_task());
    println!("{:?}",executer2.execute_task());
}
/*
MARK 1
Some(11)
Some(4)
*/
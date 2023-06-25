mod queue;
use queue::Queue;

mod stack;
use stack::Stack;

mod doubly_linked;
use doubly_linked::DLList;

#[allow(clippy::many_single_char_names)]
fn main() {
    let mut a = Queue::new();
    a.enqueue(8);
    a.enqueue(16);
    a.enqueue(32);
    a.enqueue(64);
    a.enqueue(128);
    a.enqueue(256);
    let b = a.peek();
    println!("{a:?}\n{b:?}");
    a.dequeue();
    println!("{a:?}\n{b:?}");
    a.dequeue();
    println!("{a:?}");
    a.dequeue();
    println!("{a:?}");

    let mut c = Stack::new();
    c.push(8);
    println!("{c:?}");
    c.push(16);
    println!("{c:?}");
    c.push(32);
    println!("{c:?}");
    let d = c.peek();
    println!("{c:?}\n{d:?}");
    c.pop();
    let d = c.peek();
    println!("{c:?}\n{d:?}");
    c.pop();
    let d = c.peek();
    println!("{c:?}\n{d:?}");
    c.pop();
    let d = c.peek();
    println!("{c:?}\n{d:?}");

    let mut e = DLList::new();
    println!("{e:?}");
    e.append(16); //16
    println!("{e:?}");
    e.prepend(8); //8 16
    println!("{e:?}");
    e.append(64); //8 16 64
    println!("{e:?}");
    e.prepend(4); // 4 8 16 64
    println!("{e:?}");
    e.insert_at(32, 3).expect("unreachable"); //4 8 16 32 64
    println!("{e:?}");
    println!("{:?}", e.get(3)); //32
    println!("{e:?}");
    println!("{:?}", e.remove_at(3)); //32
    println!("{e:?}");
    println!("{:?}", e.remove(&16)); //16
    println!("{e:?}");
    println!("{:?}", e.pop()); //64
    println!("{e:?}");
    println!("{:?}", e.unprepend()); //4
    println!("{e:?}");
}

#[test]
fn test_remove() {
    let mut list = DLList::new();
    list.append(32);
    list.append(16);
    list.append(8);
    println!("{list:?}");
    list.insert_at(64, 1).unwrap();
    println!("{list:?}");
    println!("{:?}", list.unprepend());
    println!("{list:?}");
    println!("{:?}", list.unprepend());
    println!("{list:?}");
    println!("{:?}", list.unprepend());
    println!("{list:?}");
    println!("{:?}", list.unprepend());
    println!("{list:?}");
}

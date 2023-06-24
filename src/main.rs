mod queue;
use queue::Queue;

mod stack;
use stack::Stack;

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
}

#[test]
fn test_enqueue() {
    let mut queue: Queue<i32> = Queue::new();
    queue.enqueue(10);
    queue.enqueue(20);
    queue.enqueue(30);
    assert_eq!(queue.length, 3);
    assert_eq!(
        {
            {
                queue.peek()
            }
        },
        Some(10)
    );
}

#[test]
fn test_dequeueue() {
    let mut queue: Queue<i32> = Queue::new();
    queue.enqueue(10);
    queue.enqueue(20);
    queue.enqueue(30);

    assert_eq!({ queue.dequeue() }, Some(10));
    assert_eq!({ queue.dequeue() }, Some(20));
    assert_eq!({ queue.dequeue() }, Some(30));
    assert_eq!({ queue.dequeue() }, None);
    assert_eq!(queue.length, 0);
}

#[test]
fn test_peek() {
    let mut queue: Queue<i32> = Queue::new();
    assert_eq!({ queue.peek() }, None);
    queue.enqueue(10);
    assert_eq!({ queue.peek() }, Some(10));
    queue.enqueue(20);
    assert_eq!({ queue.peek() }, Some(10));
}

#[derive(Clone, Debug)]
struct QNode<T>
where
    T: Clone,
{
    pub data: T,
    pub next: Option<*mut QNode<T>>,
}

#[derive(Clone, Debug)]
pub struct Queue<T: std::clone::Clone> {
    pub length: usize,
    head: Option<*mut QNode<T>>,
    tail: Option<*mut QNode<T>>,
}

impl<T: std::clone::Clone> Drop for Queue<T> {
    fn drop(&mut self) {
        while self.dequeue().is_some() {}
    }
}

impl<T: std::clone::Clone> Queue<T> {
    pub const fn new() -> Self {
        Self {
            length: 0,
            head: None,
            tail: None,
        }
    }

    pub fn enqueue(&mut self, data: T) {
        let node = Box::new(QNode { data, next: None });
        let raw = Box::into_raw(node);

        self.length += 1;

        if let Some(tail) = &mut self.tail {
            unsafe {
                //1 self.tail.next = node
                (**tail).next = Some(raw);
                //2 self.tail = node
                self.tail = Some(raw);
            }
        } else {
            self.head = Some(raw);
            self.tail = self.head;
        }

        // self.tail =  Some( Box::from_raw( raw ));
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if let Some(head) = self.head.as_mut() {
            unsafe {
                use std::alloc::{dealloc, Layout};
                let next = (**head).next;
                let data = (**head).data.clone();

                let layout = Layout::for_value(&*head);

                dealloc((*head).cast::<u8>(), layout);
                self.head = next;
                if self.length > 0 {
                    self.length -= 1;
                }
                if self.length == 0 {
                    self.tail = None;
                }
                Some(data)
            }
        } else {
            None
        }
    }

    pub fn peek(&self) -> Option<T> {
        let head = self.head?;
        Some(unsafe { (*head).data.clone() })
    }
}

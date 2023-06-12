#[derive(Debug, Clone)]
struct SNode<T>
where T: Clone,
{
    data: T,
    prev: Option<*mut SNode<T>>,
}

#[derive(Debug, Clone)]
pub struct Stack<T: Clone>{
    pub len: usize,
    head: Option<*mut SNode<T>> ,
}

impl<T: Clone> Stack<T> {
    pub fn new() -> Self {
        Self {
            len: 0,
            head: None,
        }
    }

    pub fn push(&mut self, data: T) {
        

        self.len += 1;
        if self.head.is_some() {
            unsafe {
                let node = Box::new(SNode {data, prev: self.head});
                let raw = Box::into_raw(node);
                self.head = Some(raw);
            }
        }else {
            let node = Box::new(SNode {data, prev: None});
            let raw = Box::into_raw(node); 
            self.head = Some(raw);
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Some(head) = self.head.as_mut() {
            unsafe {
                use std::alloc::{dealloc, Layout};
                let prev = (**head).prev;
                let data = (**head).data.clone();
                let layout = Layout::for_value(&*head);
                dealloc(*head as *mut u8, layout);
                self.head = prev;
                if self.len > 0{
                    self.len -= 1;
                } 
                Some(data)
            }
        }else {
            None
        }
    }

    pub fn peek(&self) -> Option<T> {
        let head = self.head?;
        Some(unsafe { (*head).data.clone() })
    }
}
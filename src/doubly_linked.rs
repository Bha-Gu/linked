#[derive(Debug)]
pub enum DLLError {
    InsertAt(InsertAtError),
}

#[derive(Debug)]
pub enum InsertAtError {
    IndexOutOfBounds,
}

#[derive(Debug, Clone)]
struct LNode<T>
where
    T: Clone + PartialEq,
{
    value: T,
    prev: Option<*mut LNode<T>>,
    next: Option<*mut LNode<T>>,
}

#[derive(Debug, Clone)]
pub struct DLList<T: Clone + PartialEq> {
    pub length: usize,
    head: Option<*mut LNode<T>>,
    tail: Option<*mut LNode<T>>,
}

impl<T: Clone + PartialEq> Drop for DLList<T> {
    fn drop(&mut self) {
        while self.pop().is_some() && self.unprepend().is_some() {}
    }
}

impl<T: Clone + PartialEq> DLList<T> {
    pub const fn new() -> Self {
        Self {
            length: 0,
            head: None,
            tail: None,
        }
    }

    pub fn prepend(&mut self, item: T) {
        let node = Box::new(LNode {
            value: item,
            prev: None,
            next: self.head,
        });
        let raw = Box::into_raw(node);
        self.length += 1;

        if let Some(head) = &mut self.head {
            unsafe {
                (**head).prev = Some(raw);
            }
        } else {
            self.tail = Some(raw);
        }

        self.head = Some(raw);
    }

    pub fn insert_at(&mut self, item: T, idx: usize) -> Result<(), DLLError> {
        if idx > self.length {
            Err(DLLError::InsertAt(InsertAtError::IndexOutOfBounds))
        } else if idx == 0 {
            self.prepend(item);
            Ok(())
        } else if idx == self.length {
            self.append(item);
            Ok(())
        } else {
            self.length += 1;
            //b: a,c c: b,d
            let mut curr = self.head;
            for _i in 0..idx {
                unsafe {
                    curr = (*curr.expect("Unreachable state")).next;
                }
            }
            //f: a, c
            unsafe {
                let mut node = (*(curr.expect("Unreachable state"))).clone();

                (node).value = item;
                (node).next = curr;

                let node = Box::new(node);
                let raw = Box::into_raw(node);
                //b: f, c

                (*curr.expect("Unreachable state")).prev = Some(raw);
                (*(*raw).prev.expect("Unreachable state")).next = Some(raw);
            }
            //f: a, b

            Ok(())
        }
    }

    pub fn append(&mut self, item: T) {
        let node = Box::new(LNode {
            value: item,
            prev: self.tail,
            next: None,
        });
        let raw = Box::into_raw(node);
        self.length += 1;

        if let Some(tail) = &mut self.tail {
            unsafe {
                (**tail).next = Some(raw);
            }
        } else {
            self.head = Some(raw);
        }

        self.tail = Some(raw);
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Some(tail) = &mut self.tail {
            unsafe {
                use std::alloc::{dealloc, Layout};
                let prev = (**tail).prev;
                let data = (**tail).value.clone();

                let layout = Layout::for_value(&*tail);
                dealloc((*tail).cast::<u8>(), layout);

                self.tail = prev;
                if let Some(prev) = prev {
                    (*prev).next = None;
                }
                if self.length > 0 {
                    self.length -= 1;
                }
                if self.length == 0 {
                    self.head = None;
                }
                Some(data)
            }
        } else {
            None
        }
    }

    pub fn unprepend(&mut self) -> Option<T> {
        if let Some(head) = &mut self.head {
            unsafe {
                use std::alloc::{dealloc, Layout};
                let next = (**head).next;
                let data = (**head).value.clone();

                let layout = Layout::for_value(&*head);
                dealloc((*head).cast::<u8>(), layout);

                self.head = next;
                if let Some(next) = next {
                    (*next).prev = None;
                }
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

    pub fn remove(&mut self, item: &T) -> Option<T> {
        if let Some(head) = self.head {
            unsafe {
                use std::alloc::{dealloc, Layout};
                let mut curr = head;
                for _i in 0..self.length {
                    if (*curr).value == item.clone() {
                        break;
                    } else if let Some(next) = (*curr).next {
                        curr = next;
                    } else {
                        return None;
                    }
                }

                if curr == head {
                    return self.unprepend();
                }
                if Some(curr) == self.tail {
                    return self.pop();
                }

                let prev = (*curr).prev;
                let next = (*curr).next;

                if let Some(next) = &mut (*curr).next {
                    (*(*next)).prev = prev;
                }
                if let Some(prev) = &mut (*curr).prev {
                    (*(*prev)).next = next;
                }
                let data = (*curr).value.clone();

                let layout = Layout::for_value(&curr);
                dealloc((curr).cast::<u8>(), layout);
                self.length -= 1;
                Some(data)
            }
        } else {
            None
        }
    }

    pub fn get(&self, idx: usize) -> Option<T> {
        if idx >= self.length {
            return None;
        }
        if let Some(head) = self.head {
            let mut curr = head;
            unsafe {
                for _i in 0..idx {
                    if let Some(next) = (*curr).next {
                        curr = next;
                    } else {
                        return None;
                    }
                }
                let data = (*curr).value.clone();
                Some(data)
            }
        } else {
            None
        }
    }

    pub fn remove_at(&mut self, idx: usize) -> Option<T> {
        if idx >= self.length {
            return None;
        }
        if let Some(head) = self.head {
            let mut curr = head;
            unsafe {
                use std::alloc::{dealloc, Layout};

                for _i in 0..idx {
                    if let Some(next) = (*curr).next {
                        curr = next;
                    } else {
                        return None;
                    }
                }
                if curr == head {
                    return self.unprepend();
                }
                if Some(curr) == self.tail {
                    return self.pop();
                }

                let prev = (*curr).prev;
                let next = (*curr).next;

                if let Some(next) = &mut (*curr).next {
                    (*(*next)).prev = prev;
                }
                if let Some(prev) = &mut (*curr).prev {
                    (*(*prev)).next = next;
                }
                let data = (*curr).value.clone();

                let layout = Layout::for_value(&curr);
                dealloc((curr).cast::<u8>(), layout);
                self.length -= 1;
                Some(data)
            }
        } else {
            None
        }
    }
}


fn main() {
    let mut l: LinkedList<u8> = LinkedList::new();

    println!("{:?}", l.to_vec());
    l.add_back(2);
    println!("{:?}", l.to_vec());
    l.add_back(3);
    println!("{:?}", l.to_vec());
    l.add_back(4);
    println!("{:?}", l.to_vec());
    l.add_front(1);
    println!("{:?}", l.to_vec());
    l.add_back(5);
    println!("{:?}", l.to_vec());
}

/// [DO NOT CHANGE THE STRUCT SIGNATURE]
#[derive(Debug, Clone, PartialEq)]
pub struct LinkedList<T> {
    pub front: Option<Box<Link<T>>>,
    length: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Link<T> {
    thing: T,
    next: Option<Box<Link<T>>>,
}

impl<T: Clone> LinkedList<T> {

    /// New instance of LinkedList
    pub fn new() -> Self {
        todo!()
    }

    /// Returns the length of the list.
    pub fn len(&self) -> usize {
        todo!()
    }

    /// Returns true if the list is empty.
    pub fn is_empty(&self) -> bool {
        todo!()
    }

    /// Adds an element to the front of the list.
    /// 1. create the new link.
    /// 2. update the head pointer of the list --> Option.take()
    /// 3. update the list metadata
    pub fn add_front(&mut self, thing: T) {
        todo!();
    }

    /// Adds an element to the back of the list.
    pub fn add_back(&mut self, thing: T) {
        // 1. iterate through list, and find final element
        let mut cur: &mut Option<Box<Link<T>>> = &mut self.front;
        while cur.is_some() && cur.as_ref().unwrap().next.is_some() {
            cur = &mut cur.as_mut().unwrap().next;
        }

        // 2. IF list is empty, create the first link
        //    ELSE add a link to the last link
        if self.length == 0 {
            self.front = Some(Box::new(Link::new(thing)));
        } else {
            cur.as_mut().unwrap().next = Some(Box::new(Link::new(thing)));
        }

        // 3. update the length metadata
        self.length += 1;
    }

    // Returns a vector from the given linked list.
    pub fn to_vec(&self) -> Vec<T> {
        let mut vec = Vec::new();
        let mut curr = &self.front;        
        while curr.is_some() {
            vec.push(curr.as_ref().unwrap().thing.clone());
            curr = &curr.as_ref().clone().unwrap().next;
        }
        vec
    }
}

impl<T> Link<T> {
    /// New instance of Link
    fn new(thing: T) -> Self {
        todo!()
    }
}
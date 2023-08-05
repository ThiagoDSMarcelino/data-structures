#[cfg(test)]
mod tests {
    use data_structures::linked_list::LinkedList;

    #[test]
    fn test_get() {
        let mut list: LinkedList<i32> = LinkedList::new(2);
        assert_eq!(list.get(0), Some(2));
        assert_eq!(list.get(1), None);
    }

    #[test]
    fn test_push() {
        let mut list: LinkedList<i32> = LinkedList::new(2);
        list.push(3);

        assert_eq!(list.get(0), Some(2));
        assert_eq!(list.get(1), Some(3));

        let sla = list.collect();
        assert_eq!(sla, [Some(2), Some(3)]);
    }

    #[test]
    fn test_collect() {
        let mut list: LinkedList<i32> = LinkedList::new(2);
        list.push(3);
        list.push(5);
        list.push(1);

        let vec = list.collect();
        assert_eq!(vec, [Some(2), Some(3), Some(5), Some(1)]);
    }

    #[test]
    fn test_pop() {
        let mut list = LinkedList::new(2);
        list.push(1);
        list.push(3);
        list.push(2);

        let popped = list.pop();
        let vec = list.collect();

        assert_eq!(popped, Some(2));
        assert_eq!(vec, [Some(2), Some(1), Some(3)]);
    }

    #[test]
    fn test_general() {
        let mut list: LinkedList<i32> = LinkedList::new(2);
        list.push(3);

        assert_eq!(list.get(0), Some(2));
        assert_eq!(list.get(1), Some(3));
        assert_eq!(list.get(2), None);

        list.push(5);
        list.push(1);
        
        let mut vec = list.collect();
        assert_eq!(vec, [Some(2), Some(3), Some(5), Some(1)]);

        list.pop();
        vec = list.collect();
        assert_eq!(vec, [Some(2), Some(3), Some(5)]);
    }
}

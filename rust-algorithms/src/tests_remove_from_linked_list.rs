#[cfg(test)]
mod tests {

    use lib::{
        LinkedListNode,
        at_linked_list,
    };

    #[test]
    fn test_remove_from_linked_list() {

        let list = LinkedListNode::Next(
            1,
            Box::new(
                LinkedListNode::Next(
                    2,
                    Box::new(
                        LinkedListNode::Next(
                            3,
                            Box::new(LinkedListNode::End),
                        )
                    )
                )
            )
        );

        assert_eq!(at_linked_list(&list, 0), 1);
        assert_eq!(at_linked_list(&list, 1), 2);
        assert_eq!(at_linked_list(&list, 2), 3);
    }
}

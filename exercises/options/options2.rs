// options2.rs
//
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: Make this an if let statement whose value is "Some" type
        let word = optional_target.unwrap();
        if let Some(i)=optional_target {
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..(range + 1) {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // TODO: make this a while let statement - remember that vector.pop also
        // adds another layer of Option<T>. You can stack `Option<T>`s into
        // while let and if let.
       
    //     while let Some(i)=integer{
    //    //pop delete last element and return last
    //         assert_eq!(i, cursor);
    while let Some(Some(integer)) = optional_integers.pop() { //vec element isn't none   and the element is an option too
        assert_eq!(integer, cursor);
            cursor -= 1;
        }

        assert_eq!(cursor, 0);
    }
}

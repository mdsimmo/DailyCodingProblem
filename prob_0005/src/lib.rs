pub fn cons<'c, 'a: 'c, 'b: 'a+'c, T>(a: &'a T, b: &'b T) -> impl Fn(&dyn Fn(&'a T, &'b T) -> &'c T) -> &'c T {
    move |f| f(&a, &b)
}

pub fn car<'c, 'a: 'c, 'b: 'c, T>(pair: &dyn Fn(&dyn Fn(&'a T, &'b T) -> &'c T) -> &'c T) -> &'c T {
    pair(&|a, _b| a)
}

pub fn cdr<'c, 'a: 'c, 'b: 'c, T>(pair: &dyn Fn(&dyn Fn(&'a T, &'b T) -> &'c T) -> &'c T) -> &'c T {
    pair(&|_a, b| b)
}

#[cfg(test)]
mod test {
    use ::*;

    #[test]
    fn test_car() {
        let pair = cons(&1, &2);
        let first = car(&pair);

        assert_eq!(first, &1);
    }

    #[test]
    fn test_cdr() {
        let pair = cons(&1, &2);
        let sec = cdr(&pair);

        assert_eq!(sec, &2);
    }

    #[test]
    fn test_refrences_not_stollen() {
        let pair = cons(&3, &4);
        let first = car(&pair);
        let sec = cdr(&pair);

        assert_eq!(first, &3);
        assert_eq!(sec, &4);
    }

    #[test]
    fn test_works_with_any_type() {
        // annotations are not needed for the assertions
        #[derive(PartialEq, Debug)]
        struct A<'a>(&'a str);

        let pair = cons(&A("a"), &A("b"));
        let first = car(&pair);
        let sec = cdr(&pair);

        assert_eq!(first, &A("a"));
        assert_eq!(sec, &A("b"));
    }
}
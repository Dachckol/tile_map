#[macro_export]
macro_rules! same_enum {
    ( $e1: expr, $e2: pat ) => {
        {
            match $e1 {
                $e2 => true,
                _ => false,
            }
        }
    };
}


#[cfg(test)]
mod tests {
    #[macro_use]
    use super::*;

    #[test]
    fn true_for_same_enum() {
        assert!(same_enum!(Option::Some(1), Option::Some(1)));
    }
    
    #[test]
    fn false_for_different_enum() {
        assert!(!same_enum!(Option::Some(2), Option::None));
        assert!(!same_enum!(Option::Some(2), Option::Some(1)));
    }
}

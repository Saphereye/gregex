use std::hash::{Hash, Hasher};

/// The `SetTerminal` enum represents the different types of terminals that can be used in a regular expression.
#[derive(Debug)]
pub enum SetTerminal {
    SingleElement(char, u32),            // a₁
    DoubleElement(char, u32, char, u32), // a₁b₂
    Epsilon,                             // ε
    Empty,                               // ∅
}

impl SetTerminal {
    /// Performs the cartesian product of two `SetTerminal` instances.
    pub fn product(&self, other: &SetTerminal) -> SetTerminal {
        match (self, other) {
            (SetTerminal::SingleElement(a, a_code), SetTerminal::SingleElement(b, b_code)) => {
                SetTerminal::DoubleElement(*a, *a_code, *b, *b_code)
            }
            (SetTerminal::SingleElement(a, a_code), SetTerminal::Epsilon) => {
                SetTerminal::SingleElement(*a, *a_code)
            }
            (SetTerminal::Epsilon, SetTerminal::SingleElement(b, b_code)) => {
                SetTerminal::SingleElement(*b, *b_code)
            }
            (SetTerminal::Epsilon, SetTerminal::Epsilon) => SetTerminal::Epsilon,
            (SetTerminal::Empty, _) => SetTerminal::Empty,
            (_, SetTerminal::Empty) => SetTerminal::Empty,
            _ => unreachable!("Invalid product"),
        }
    }
}

impl PartialEq for SetTerminal {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (SetTerminal::SingleElement(a, a_code), SetTerminal::SingleElement(b, b_code)) => {
                a == b && a_code == b_code
            }
            (
                SetTerminal::DoubleElement(a, a_code, b, b_code),
                SetTerminal::DoubleElement(c, c_code, d, d_code),
            ) => a == c && a_code == c_code && b == d && b_code == d_code,
            (SetTerminal::Epsilon, SetTerminal::Epsilon) => true,
            (SetTerminal::Empty, SetTerminal::Empty) => true,
            _ => false,
        }
    }
}

impl Eq for SetTerminal {}

impl Hash for SetTerminal {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            SetTerminal::SingleElement(a, a_code) => {
                a.hash(state);
                a_code.hash(state);
            }
            SetTerminal::DoubleElement(a, a_code, b, b_code) => {
                a.hash(state);
                a_code.hash(state);
                b.hash(state);
                b_code.hash(state);
            }
            SetTerminal::Epsilon => {
                "Epsilon".hash(state);
            }
            SetTerminal::Empty => {
                "Empty".hash(state);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_product() {
        let a = SetTerminal::SingleElement('a', 1);
        let b = SetTerminal::SingleElement('b', 2);
        let c = SetTerminal::Epsilon;
        let d = SetTerminal::Empty;

        assert_eq!(a.product(&b), SetTerminal::DoubleElement('a', 1, 'b', 2));
        assert_eq!(a.product(&c), SetTerminal::SingleElement('a', 1));
        assert_eq!(c.product(&b), SetTerminal::SingleElement('b', 2));
        assert_eq!(c.product(&c), SetTerminal::Epsilon);
        assert_eq!(d.product(&a), SetTerminal::Empty);
        assert_eq!(b.product(&d), SetTerminal::Empty);
    }
}
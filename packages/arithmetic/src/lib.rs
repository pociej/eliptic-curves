use wasm_bindgen::prelude::*;

#[derive(Debug, Clone)]
pub struct FieldElement {
    pub num: u32,
    pub prime: u32,
}

impl FieldElement {
    pub fn new(num: u32, prime: u32) -> Self {
        // check that num is between 0 and prime-1 inclusive.
        // if not, will be paniced.
        if num >= prime {
            panic!("Num {} not in field range 0 to {}", num, prime - 1);
        }

        Self { num, prime }
    }

    pub fn pow(&self, exponent: u32) -> Self {
        let pow = self.num.pow(exponent);
        let rem = pow % self.prime;
        let div = (pow - rem) / self.prime;
        Self {
            num: div,
            prime: self.prime,
        }
    }
}

pub fn AddFieldElements(first_element: FieldElement, second_element: FieldElement) -> FieldElement {
    if first_element.prime != second_element.prime {
        panic!("Cannot add two numbers in different Fields");
    }

    let num = (first_element.num + second_element.num) % first_element.prime;
    FieldElement::new(num, first_element.prime)
}

pub fn SubFieldElements(first_element: FieldElement, second_element: FieldElement) -> FieldElement {
    if first_element.prime != second_element.prime {
        panic!("Cannot subtract two numbers in different Fields");
    }

    let num = (first_element.num + first_element.prime - second_element.num) % first_element.prime;
    FieldElement::new(num, first_element.prime)
}

pub fn MulFieldElements(first_element: FieldElement, second_element: FieldElement) -> FieldElement {
    if first_element.prime != second_element.prime {
        panic!("Cannot multiply two numbers in different Fields");
    }

    let num = (first_element.num * second_element.num) % first_element.prime;
    FieldElement::new(num, first_element.prime)
}

pub fn DivFieldElements(first_element: FieldElement, second_element: FieldElement) -> FieldElement {
    if first_element.prime != second_element.prime {
        panic!("Cannot divide two numbers in different Fields");
    }

    let exp = second_element.prime - 2;
    let num_pow = second_element.pow(exp);
    let result = first_element.num * num_pow.num;
    FieldElement::new(result, first_element.prime)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let a = FieldElement::new(2, 31);
        let b = FieldElement::new(15, 31);
        let c = FieldElement::new(17, 31);
        assert_eq!(AddFieldElements(a, b).num, c.num);
    }
}

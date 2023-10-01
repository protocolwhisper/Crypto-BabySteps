#[derive(Debug)]
struct Polynomial {
    coefficients: Vec<i64>,
}

impl Polynomial {
    fn new(coefficients: Vec<i64>) -> Self {
        Polynomial { coefficients }
    }
    //Hornet's Method for Efficient Evaluating
    // X is the
    fn evaluate(
        &self,
        x: i64, //p: i64
    ) -> i64 {
        let mut result = 0;
        //2x3+3x2-4x+1

        for coefficients in self.coefficients.iter().rev()
        // For preserving sagemath order
        {
            result = result * x + coefficients; // Iterate through all coeficcients
                                                //Handling the mo
        }
        result
        // let moduli = result % p;
        // println("Moduli taken", &moduli)
    }
    //
    fn evaluate_gf(&self, x: i64, p: i64) -> i64 {
        self.evaluate(x) % p
    }

    fn add(&self, other: &Polynomial) -> Polynomial {
        let sum: Vec<i64> = self
            .coefficients
            .iter()
            //Group iterators
            .zip(other.coefficients.iter())
            //Capture value of the group iterators and add
            .map(|(a, b)| a + b)
            //Collecting into vec
            .collect();
        Polynomial::new(sum)
    }

    fn multiply(&self, other: &Polynomial) -> Polynomial {
        let mut result_coefficients =
            vec![0; self.coefficients.len() + other.coefficients.len() - 1]; // Sizing
                                                                             //[a, b, c] -> (0, a), (1, b), and (2, c).
        for (i, &coeff_a) in self.coefficients.iter().enumerate() {
            for (j, &coeff_b) in other.coefficients.iter().enumerate() {
                result_coefficients[i + j] += coeff_a * coeff_b;
            }
        }

        Polynomial::new(result_coefficients)
    }
}

fn main() {
    let poly = Polynomial::new(vec![1, -4, 3, 2]);
    let value = Polynomial::new(vec![2, 3, 1, 5]);
    let add = poly.add(&value);
    println!("Result{:?}", add);
    let result = value.evaluate(8); // Example usage
    let result2 = value.evaluate_gf(8, 13);
    let poly3 = poly.multiply(&value);
    println!("{:?}", poly3.coefficients); // Output: [4, 13, 28, 27, 18]
    println!("Result {} ", result);
    println!("Result {} ", result2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polynomial_multiply() {
        let poly1 = Polynomial::new(vec![1, 2]); // 1 + 2x
        let poly2 = Polynomial::new(vec![3, 4]); // 3 + 4x

        let poly3 = poly1.multiply(&poly2);

        assert_eq!(poly3.coefficients, vec![3, 10, 8]); // 3 + 10x + 8x^2
    }
}

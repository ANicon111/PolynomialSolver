use std::{
    convert::From,
    env,
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Div, Mul, Sub},
    vec,
};
#[derive(Copy, Clone, Debug, Default, PartialEq)]
struct Complex {
    re: f64,
    im: f64,
}

impl Add for Complex {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }
}

impl AddAssign for Complex {
    fn add_assign(&mut self, other: Self) {
        self.re += other.re;
        self.im += other.im;
    }
}

impl Sub for Complex {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            re: self.re - other.re,
            im: self.im - other.im,
        }
    }
}

impl Mul for Complex {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            re: self.re * other.re - self.im * other.im,
            im: self.re * other.im + self.im * other.re,
        }
    }
}

impl Div for Complex {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        let div = other.re * other.re - other.im * other.im;
        Self {
            re: (self.re * other.re + self.im * other.im) / div,
            im: (self.im * other.re - self.re * other.im) / div,
        }
    }
}

impl PartialOrd for Complex {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (self.re * self.re + self.im * self.im)
            .partial_cmp(&(other.re * other.re + other.im * other.im))
    }
}

impl Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.re == 0.0 && self.im == 0.0 {
            f.write_fmt(format_args!("0"))
        } else if self.im == 0.0 {
            f.write_fmt(format_args!("{}", self.re))
        } else if self.re == 0.0 {
            if self.im == 1.0 {
                f.write_fmt(format_args!("i"))
            } else if self.im == -1.0 {
                f.write_fmt(format_args!("-i"))
            } else {
                f.write_fmt(format_args!("{}i", self.im))
            }
        } else if self.im > 0.0 {
            f.write_fmt(format_args!("{}+{}i", self.re, self.im))
        } else {
            f.write_fmt(format_args!("{}{}i", self.re, self.im))
        }
    }
}

impl From<f64> for Complex {
    fn from(val: f64) -> Self {
        Complex { re: val, im: 0.0 }
    }
}

impl Complex {
    fn two_dec(&self) -> Complex {
        let mut x = *self;
        x.re = (x.re * 100.0).round() / 100.0;
        x.im = (x.im * 100.0).round() / 100.0;
        x
    }
}

fn get_complex_number(mut string: &str, mut ok: &bool) -> Complex {
    if string.is_empty() {
        return Complex { re: 1.0, im: 0.0 };
    }
    let mut sign: f64 = 1.0;
    if string.starts_with("-") {
        sign = -1.0;
        string = string.strip_prefix("-").unwrap();
    } else if string.starts_with("+") {
        string = string.strip_prefix("+").unwrap();
    }
    let val: f64;
    if !string.starts_with("i") {
        let unparsed_val = string.split(&['+', '-', 'i']).nth(0).unwrap();
        let wrapped_val = unparsed_val.parse();
        match wrapped_val {
            Ok(value) => {
                val = value;
                string = string.strip_prefix(unparsed_val).unwrap();
            }
            _ => {
                ok = &false;
                val = 0.0
            }
        }
    } else {
        val = 1.0;
    }
    if string.starts_with("i") {
        string = string.strip_prefix("i").unwrap();
        if string.is_empty() || !ok {
            Complex {
                re: 0.0,
                im: sign * val,
            }
        } else {
            Complex {
                re: 0.0,
                im: sign * val,
            } + get_complex_number(string, &ok)
        }
    } else {
        if string.is_empty() || !ok {
            Complex {
                re: sign * val,
                im: 0.0,
            }
        } else {
            Complex {
                re: sign * val,
                im: 0.0,
            } + get_complex_number(string, &ok)
        }
    }
}

#[derive(Debug)]
struct Polynomial {
    coeficients: Vec<Complex>,
}

impl Default for Polynomial {
    fn default() -> Self {
        Polynomial {
            coeficients: Vec::new(),
        }
    }
}

impl Display for Polynomial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut polynomial = "".to_string();

        for i in (0..self.coeficients.len()).rev() {
            let val = self.coeficients[i].two_dec();
            if val != Complex::default() {
                if polynomial.is_empty() {
                    if val.re == 1.0 && val.im == 0.0 {
                        match i {
                            0 => polynomial += format!("1").as_str(),
                            1 => polynomial += format!("X").as_str(),
                            _ => polynomial += format!("X^{}", i).as_str(),
                        }
                    } else if val.re == -1.0 && val.im == 0.0 {
                        match i {
                            0 => polynomial += format!("-1").as_str(),
                            1 => polynomial += format!("-X").as_str(),
                            _ => polynomial += format!("-X^{}", i).as_str(),
                        }
                    } else {
                        match i {
                            0 => polynomial += format!("{}", val).as_str(),
                            1 => polynomial += format!("{}X", val).as_str(),
                            _ => polynomial += format!("{}X^{}", val, i).as_str(),
                        }
                    }
                } else {
                    if val.re == 1.0 && val.im == 0.0 {
                        match i {
                            0 => polynomial += format!(" + 1").as_str(),
                            1 => polynomial += format!(" + X").as_str(),
                            _ => polynomial += format!(" + X^{}", i).as_str(),
                        }
                    } else if val.re == -1.0 && val.im == 0.0 {
                        match i {
                            0 => polynomial += format!(" - 1").as_str(),
                            1 => polynomial += format!(" - X").as_str(),
                            _ => polynomial += format!(" - X^{}", i).as_str(),
                        }
                    } else if val.re < 0.0 || val.re == 0.0 && val.im <= 0.0 {
                        match i {
                            0 => polynomial += format!(" - {}", Complex::default() - val).as_str(),
                            1 => polynomial += format!(" - {}X", Complex::default() - val).as_str(),
                            _ => {
                                polynomial +=
                                    format!(" - {}X^{}", Complex::default() - val, i).as_str()
                            }
                        }
                    } else {
                        match i {
                            0 => polynomial += format!(" + {}", val).as_str(),
                            1 => polynomial += format!(" + {}X", val).as_str(),
                            _ => polynomial += format!(" + {}X^{}", val, i).as_str(),
                        }
                    }
                }
            }
        }

        if polynomial.is_empty() {
            polynomial = "0".to_string();
        }
        f.write_fmt(format_args!("{}", polynomial))
    }
}

fn show_solutions(solution_list: Vec<Complex>) -> String {
    let mut result: String = "{ ".to_string();
    let length: usize;
    if solution_list.len() == 0 {
        return "{}".to_string();
    }
    length = solution_list.len() - 1;
    for i in 0..length {
        result += format!("{}, ", solution_list[i]).as_str();
    }
    result += format!("{} }}", solution_list.last().unwrap()).as_str();
    result
}

impl Polynomial {
    fn value_at(&self, x: Complex) -> Complex {
        let mut factor = Complex { re: 1.0, im: 0.0 };
        let mut solution = Complex::default();

        for i in 0..self.coeficients.len() {
            solution += factor * self.coeficients[i];
            factor = factor * x;
        }
        solution
    }
    fn search_min(&self) -> Complex {
        let mut solution = Complex { re: 0.0, im: 0.0 };
        let mut re = 1e20;
        for _ in 0..150 {
            let mut im = re;
            for _ in 0..150 {
                if self.value_at(solution) > self.value_at(solution + Complex { re, im }) {
                    solution += Complex { re, im };
                } else if self.value_at(solution)
                    > self.value_at(solution + Complex { re: -re, im })
                {
                    solution += Complex { re: -re, im };
                } else if self.value_at(solution)
                    > self.value_at(solution + Complex { re, im: -im })
                {
                    solution += Complex { re, im: -im };
                } else if self.value_at(solution)
                    > self.value_at(solution + Complex { re: -re, im: -im })
                {
                    solution += Complex { re: -re, im: -im };
                }
                if self.value_at(solution) == Complex::default() {
                    break;
                }
                im /= 2.0;
            }
            re /= 2.0;
        }
        solution
    }
    fn horner(&mut self, div: Complex) -> Complex {
        let mut solution: Vec<(usize, Complex)> = vec![];
        for i in (1..self.coeficients.len()).rev() {
            let coeficient: Complex = solution
                .last()
                .unwrap_or(&(0, Complex { re: 0.0, im: 0.0 }))
                .1
                * div
                + self.coeficients[i];
            solution.push((i - 1, coeficient));
        }
        self.coeficients.pop();
        for val in solution {
            self.coeficients[val.0] = val.1;
        }
        Complex { re: 0.0, im: 0.0 }
    }
    fn degree(&self) -> usize {
        let max_power: usize = std::cmp::max(self.coeficients.len(), 1) - 1;
        max_power
    }
    fn trim0s(&mut self) {
        while self
            .coeficients
            .last()
            .unwrap_or(&Complex { re: 1.0, im: 0.0 })
            == &Complex::default()
        {
            self.coeficients.pop();
        }
    }
    fn add0s(&mut self, new_length: usize) {
        let length = self.coeficients.len();
        for _ in length..new_length {
            self.coeficients.push(Complex::default());
        }
    }
}

fn main() {
    let mut polynomial: Polynomial = Polynomial::default();
    let args: Vec<String> = env::args().collect();
    if args.contains(&"-h".to_string()) {
        println!(
            "\
Standard notation: X^5 + 4X^4 + 3iX^3 - 2+3ix^2 + (i+3)x^2 - x + 1
Clumped variables act as if they are in a parenthesis, and as such, they are optional"
        );
        return;
    }
    let mut ok: bool = true;
    let mut err: String = "".to_string();
    let mut sign: f64 = 1.0;
    for i in 1..args.len() {
        if args[i] == "-" || args[i] == "+" {
            if args[i] == "-" {
                sign = -1.0;
            } else {
                sign = 1.0;
            }
        } else if args[i].contains("x") || args[i].contains("X") {
            let mut coeficient = args[i].split(&['x', 'X']).nth(0).unwrap();
            let power = args[i].split(&['x', 'X']).last().unwrap();
            let mut powerval: usize = 0;
            if !power.contains("^") {
                if !power.is_empty() {
                    ok = false;
                    err = ("Invalid power at ").to_owned()
                        + args[i].as_str()
                        + "; use ax^k to signify a power";
                }
                powerval = 1;
            } else {
                let unwrappedval = power.split("^").last().unwrap().parse::<usize>();
                match unwrappedval {
                    Ok(val) => powerval = val,
                    _ => ok = false,
                };
            }
            if coeficient.starts_with("(") {
                coeficient = coeficient
                    .strip_prefix("(")
                    .unwrap()
                    .strip_suffix(")")
                    .unwrap();
            }
            polynomial.add0s(powerval + 1);
            polynomial.coeficients[powerval] +=
                Complex { re: sign, im: 0.0 } * get_complex_number(coeficient, &ok);
        } else {
            polynomial.coeficients[0] +=
                Complex { re: sign, im: 0.0 } * get_complex_number(args[i].as_str(), &ok);
        }
    }
    polynomial.trim0s();
    if !ok {
        if err.is_empty() {
            println!("Failed to get the term coefficients",)
        } else {
            println!("{}", err);
        }
    } else {
        //creating the polynomial output string
        //getting solutions
        let polynomial_string = format!("{}", polynomial);
        let mut solutions: Vec<Complex> = vec![];
        let max_power: usize = polynomial.degree();

        println!("Horner results:");
        for i in 0..max_power {
            let solution = polynomial.search_min();
            solutions.push(solution);
            polynomial.horner(solution);
            polynomial.trim0s();
            println!("{}", polynomial);
            solutions[i] = solutions[i].two_dec();
        }
        println!(
            "The roots of {} are: S = {}",
            polynomial_string,
            show_solutions(solutions)
        );
    }
}

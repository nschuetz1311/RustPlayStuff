mod interest;
mod tax_calc;
use num::integer::gcd;
use num::integer::lcm;

fn main() {
    let income = 45000.0;
    let tax = tax_calc::calculate_income_tax (income);
 
    // ':.' shows how many digits should be displayed after the '.'
    // so in this case 2 digits after .
    println!("Annual Income Tax: ${:.2}", tax);

    let principal = 1000.0;
    let rate = 0.05;
    let time = 10.0;
    let n = 4.0;
    let compound_amount = interest::calculate_compound_interest(principal, rate, time, n);
    println!("Compound Interest Amount: ${:.2}", compound_amount);

    let simple_amount = interest::calculate_simple_interest (principal, rate, time);
    println!("Simple Interest Amount: ${:.2}", simple_amount);

    let tax_rate = tax_calc::get_tax_rate (income);
    println!("Tax Rate: {:.2}%", tax_rate * 100.0);

    // Scenario 2 : Find GCD and LCM of two numbers
    let a = 56;
    let b = 98;

    let result_lcm = lcm(a, b);
    let result_gcd = gcd(a, b);

    println!("LCM and GCD of {a} and {b} are {result_lcm} and {result_gcd}");
}

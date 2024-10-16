use rand::seq::SliceRandom;
use rand;

fn main() {
    let p: i128 = find_prime();
    let q: i128 = find_prime();

    let n: i128 = q*p;

    let phi: i128 = (p-1) * (q-1);
    let e: i128 = find_prime_bigger_than(p, q, n);
    let d: i128 = resolve_diophantic(e, phi, 1);

    println!("For the numbers p y q: {},{}",p,q);
    println!("n: {}",n);
    println!("phi: {}", phi);
    println!("e: {}",e);
    println!("d: {}",e);



















































































































    
}


    

fn gcd_extended(a: i128, b: i128) -> (i128, i128, i128) {
    if a == 0 {
        return (b, 0, 1);
    }
    
    let (g, x1, y1) = gcd_extended(b % a, a);
    
    let x = y1 - (b / a) * x1;
    let y = x1;

    (g, x, y)
}

fn resolve_diophantic(coef_x: i128, coef_y: i128, result: i128) -> i128 {
    let (g, x0, y0) = gcd_extended(coef_x.abs(), coef_y.abs());

    // Scale the solution by c / g
    let x = x0 * (result / g);

    // Adjust signs based on the original a and b
    let final_x = if coef_x < 0 { -x } else { x };

    return final_x;
}

fn find_prime_bigger_than(p: i128, q: i128, n: i128) -> i128 {
    loop {
        let e = find_prime();
        if e > p && e > q && 2^e > n {return e}
    }
}

fn find_prime() -> i128 {
    loop {
        let x = rand::random::<i128>();
        if test_miller_robin(x){return x}
    }
}

fn test_miller_robin(n: i128) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 || n == 3 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }

    let k = 100;  // Number of random bases to test

    // Create a vector of numbers from 2 to n-1
    let mut numbers: Vec<i128> = (2..n).collect();

    // Create a random number generator
    let mut rng = rand::thread_rng();

    // Shuffle the numbers and take the first k elements
    let selected = numbers.as_mut_slice().choose_multiple(&mut rng, k);

    // Perform Miller-Rabin test for each selected base
    for &base in selected {
        if !test_miller(n, base) {
            return false; // Composite if test fails for any base
        }
    }
    return true;  // Probably prime if all tests pass
}

fn test_miller(n: i128, base: i128) -> bool {

    // Write n-1 as 2^s * d, with d odd
    let mut d = n - 1;
    let mut s = 0;
    while d % 2 == 0 {
        d /= 2;
        s += 1;
    }

    // a^d % n
    let mut x = mod_exp(base, d, n);
    if x == 1 || x == n - 1 {
        return true;
    }

    // Square x and check if it becomes n-1
    for _ in 0..s-1 {
        x = mod_exp(x, 2, n);
        if x == n - 1 {
            return true;
        }
    }

    false
}

// Helper function for modular exponentiation
fn mod_exp(mut base: i128, mut exp: i128, modulus: i128) -> i128 {
    let mut result = 1;
    base = base % modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        exp = exp >> 1;
        base = (base * base) % modulus;
    }
    result
}




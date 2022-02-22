fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }

    let v2 = vec![1, 2, 3]; // we consume v1_iter so we can't use it. we need to define a new iter
    let v2_iter = v2.iter();
    let total: i32 = v2_iter.sum(); // sum method is also a consuming adaptors so we can't use v2_iter after using that
    println!("Total: {}", total);

    let v3: Vec<i32> = vec![1, 2, 3];
    let outer_val = 4;
    let v4: Vec<_> = v3.iter().map(|x| x * outer_val).collect(); // because we use a clouser in map; we have access to outer scope variables too
    println!("v3 plus 1 is: {:?}", v4)
}

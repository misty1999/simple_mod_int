### example

```rust
use simple_mod_int::Mod;

let mut a: Mod<25> = 7.into();
let b: Mod<25> = 13.into();

let sum = a + b;  // 20
let product = a * b;  // 16
let power = a.pow(3);  // 18
let inv = a.inv();  // 18
let power_neg = a.pow(-3);  // 7
let div = a / b;  // 14
a /= b; // a = 14
a += 10; // a = 7
println!("{:?}",a); // {value : 7, mod : 25 }
println!("{}",a.value); // 7
```
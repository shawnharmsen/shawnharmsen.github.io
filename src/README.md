# INTRODUCTION

Inspiration from [RiceMaximalist](https://mobile.twitter.com/ricemaximalist) journal [rice-daily.super.site](https://rice-daily.super.site/)

# Rust snippets

```rust 
// format strings with [] and ()
fn main() {
    let inputs = ["string1", "string2", "string3"];

    for input in &inputs {
        let output = format!(" - [{input}] ({input})");
        println!("{output}");
    }
}
```
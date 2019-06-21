# default-env

`default_env!` is a macro like [`env!`](https://doc.rust-lang.org/std/macro.env.html) that returns a default value if the environment variable is not found.
Unlike [`option_env!`](https://doc.rust-lang.org/std/macro.option_env.html), the output of `default_env!` can be used in macros (because who doesn't love macros in their macros?).

## Example

```rust
macro_rules! long_str {
  () => {
    concat!(
        "Hello, ", default_env!("USER", "anonymous user"), ".",
        "Today is ", default_env!("WEEKDAY", compile_error!("You exist in a land beyond time."))
      )
  }
}
```

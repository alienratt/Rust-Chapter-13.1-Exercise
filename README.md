# Rust-Chapter-13.1-Exercise
Chapter challenge for Closures found in the last two paragraphs of this section: https://doc.rust-lang.org/stable/book/ch13-01-closures.html#limitations-of-the-cacher-implementation

>Try modifying Cacher to hold a hash map rather than a single value. The keys of the hash map will be the arg values that are passed in, and the values of the hash map will be the result of calling the closure on that key. Instead of looking at whether self.value directly has a Some or a None value, the value function will look up the arg in the hash map and return the value if it’s present. If it’s not present, the Cacher will call the closure and save the resulting value in the hash map associated with its arg value.
>
>The second problem with the current Cacher implementation is that it only accepts closures that take one parameter of type u32 and return a u32. We might want to cache the results of closures that take a string slice and return usize values, for example. To fix this issue, try introducing more generic parameters to increase the flexibility of the Cacher functionality.

---
### Hints
See chapter 10.2 for trait bounds needed for extra generics.

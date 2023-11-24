## Variables and Mutability

1. How to Declare Variables in Rust

    - Use the `const` or `let` keywords to initialize variables<br/>

    - `let` variable declarations have **optional** type annotations and are **immutable by default**

    - Examples: <br/>

        ~~~
        // Initializing an i32 type variable that is immutable without the type

        let x = 4;

        // Initializing an i32 type variable that is immutable with a type

        let y: i32 = 4;
        ~~~

    - Defaults for primitive types

        Integers -> `i32`<br/>
        Floats -> `f64`<br/>
        Characters -> `char`<br/>
        Booleans -> `bool`<br/>

    - `const` variable declarations have **mandatory** type annotations and are **always immutable**

    - Example: <br/>

        ~~~
        // Initializing a float type constant variable that is always immutable

        const IMMUTABLE: f64 = 5.78;
        ~~~

    Code Example:

    `cd declaring_variables` && `cargo run`

2. The concept of Mutability/Immutability in Rust

   - All variables are immutable by default

3. `let`, `mut`, and `const` Keywords

4. Benefits of the Immutable Design Pattern

5. General Best Practices

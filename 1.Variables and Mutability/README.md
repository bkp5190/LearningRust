## Variables and Mutability

1. How to Declare Variables in Rust

    - Use the `const` or `let` keywords to initialize variables

    - Type annotations are optional

    - Examples: <br/>

        ~~~
        // Initializing an i32 type variable that is immutable without the type

        let x = 4;

        // Initializing an i32 type variable that is immutable with a type

        let x: i32 = 4;
        ~~~

    - Defaults for primitive types

        Integers -> `i32`<br/>
        Floats -> `f64`<br/>
        Characters -> `char`<br/>
        Booleans -> `bool`<br/>

    - `const` variable declarations require type annotations and are always immutable

    - Example: <br/>

        ~~~
        // Initializing a float type constant variable that is always immutable

        const IMMUTABLE: f64 = 5.78;
        ~~~

2. The concept of Mutability/Immutability in Rust

3. `let`, `mut`, and `const` Keywords

4. Benefits of the Immutable Design Pattern

5. General Best Practices

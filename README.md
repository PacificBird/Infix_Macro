# Infix Macro 
#### A declaritive macro for quickly creating infix operators

Using the technique first displayed by @wishawa on 
[RFC #1579](https://github.com/rust-lang/rfcs/issues/1579#issuecomment-1398724803), 
by overloading the multiplication operator, and by creating an intermediate struct that "passes" 
the left hand operand over to the right over the multiplications, you can create custom binary 
infix operations that work like this: 
>`let dot_product = vector_a *dot* vector_b;`

Unfortunately, to force this unintended functionality out of rust it requires quite a lot of boilerplate (see the linked RFC ). 
Un-unfortunately, this boilerplate is almost exactly identical between implementations, 
which means we can macro it all away! That's exactly what this package does.

##### Examples and Usage

Simply `cargo add infix_macro` and then place `use infix_macro::infix;` at the top of your module, then follow this syntax:
> `infix!(name, partial_name, T, U, F);`

where `name` is the operator you'll actually be using (I recommend something short and lower case), 
`partial_name` is the name of the intermediary `struct` used to pass the value over the overloaded multiplications 
(I recommend something like `PartialName`, you may need to use it if the closure can't infer the types by itself), 
`T` is the input type, `U` is the output type, and `F` is a binary function implementing `Fn(T, T) -> U` 
  
After you've done this, you can use your `*infix*` operator. I have not yet profiled performance against raw function or method calls, 
though I plan to do that in the near future. One can reasonably assume it is ever so slightly less performant than those options.

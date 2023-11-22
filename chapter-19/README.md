#advanced features
#trait
#macro



Unsafe Rust: how to opt out of some of Rust’s guarantees and take responsibility for manually upholding those guarantees  
Advanced traits: associated types, default type parameters, fully qualified syntax, supertraits, and the newtype pattern in relation to traits  
Advanced types: more about the newtype pattern, type aliases, the never type, and dynamically sized types  
Advanced functions and closures: function pointers and returning closures  
Macros: ways to define code that defines more code at compile time  



##unsafe superpower
Dereference a raw pointer  
Call an unsafe function or method  
Access or modify a mutable static variable  
Implement an unsafe trait  
Access fields of unions




## Macros

Custom #[derive] macros that specify code added with the derive attribute used on structs and enums 
Attribute-like macros that define custom attributes usable on any item 
Function-like macros that look like function calls but operate on the tokens specified as their argument 

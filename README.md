# risein-rust
This is a Risein tutorials for Rust Smart Contracts.


## Comparison of Collection Types

In this part, we will summarize the differences between vectors, strings, and hash maps, and discuss when to use each collection type in Rust.

### Vectors

    Vectors store elements of the same type in a contiguous memory space, which makes them ideal for handling ordered lists of elements.
    Vectors automatically resize themselves when needed, making them a dynamic alternative to arrays.
    When to use: You should choose vectors when you need a dynamic, ordered collection of elements where all elements have the same type.

### Strings

    Strings are used for storing and manipulating text data. They handle UTF-8 encoded text, which allows the representation of a wide range of characters.
    The String type is mutable and can grow or shrink as needed.
    When to use: You should use strings when you need to store or manipulate text data, while ensuring proper handling of Unicode characters.

### Hash Maps

    Hash maps store key-value pairs, allowing you to associate a value with a unique key.
    They provide quick lookup, insertion, and deletion of elements based on the key.
    When to use: Choose hash maps when you need to store data in a collection with unique keys and fast retrieval times, or when you want to quickly find a value based on a specific key.


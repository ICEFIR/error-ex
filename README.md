# Error _Ex_
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)  

---
`error-ex` is a Rust crate designed for those who desire clear, explicit, and easy-to-use error creation and handling.

## Features

---
The Goal of this create si to provide the following functionalies

- **Dead Simple**: Error definition at will!
- **Explicit Error Creation**:  Conveniently construct your own error types with associated messages.
- **Error Mapping**: Effortlessly map one error type to another, enhancing clarity and context.

## Installation

---
Add this to your `Cargo.toml`:

```toml
[dependencies]
error-ex = "0.1.2"
```

## Quickstart

---

### Creating and instantiating Explicit Errors

```rust
use error_ex::{create_error, map_to_error};

create_error!(InputError => IllegalArgument, InvalidInput, MissingArgument);
```

And you'd be able to instantiate this error!
```rust
InputError::illegal_argument(format!("Your message here"));
```

---

### Error Mapping

The explicit way
```rust
 use std::fs;
 use error_ex::{create_error};

 create_error!(FileError => IoError);
 create_error!(SchemaError => ParseError);

 let error: Result<(), FileError> = Err(FileError::io_error("".to_string()));

 let mapped = error.map_err(|error| {
     SchemaError::parse_error(format!("SchemaError::ParseError error {error}"))
 });
```


### Function wrapper


 The above code can be simplified using `map_to_error!`
 macro using the build in lower order function
 ```rust
 use std::fs;
 use std::io::Error;
 use error_ex::{create_error};
 
 create_error!(FileError => IoError);
 create_error!(SchemaError => ParseError);
 
 let error: Result<(), FileError> = Err(FileError::io_error("".to_string()));

 let mapped = error.map_err(SchemaError::map_parse_error);
 ```

### Usage Examples

---
Please refer to rust doc and test directory for sample use-cases.

## Contribution

---
Any PR or suggestions for this library is welcome and appreciated! :)

## License

---

```
MIT License

Copyright (c) 2023 ICEFIR

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

```
# Rust Calculator

A command-line calculator application written in Rust that supports mathematical expressions with proper operator precedence and parentheses.

## Features

- **Basic Arithmetic Operations**: Addition (+), Subtraction (-), Multiplication (*), Division (/)
- **Operator Precedence**: Correctly follows mathematical order of operations
- **Parentheses Support**: Nested parentheses for complex expressions
- **Decimal Numbers**: Floating-point arithmetic support
- **Negative Numbers**: Unary minus operator support
- **Error Handling**: Comprehensive error messages for invalid expressions
- **Interactive Mode**: Real-time calculation with command-line interface

## Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd calculator
```

2. Build the project:
```bash
cargo build --release
```

3. Run the calculator:
```bash
cargo run
```

Or run the release binary:
```bash
./target/release/calculator
```

## Usage

### Interactive Mode
Start the calculator and enter mathematical expressions:

```
Rust Calculator Demo
==================
2+3 = 5
10-4 = 6
3*4 = 12
8/2 = 4
2+3*4 = 14
(2+3)*4 = 20
2.5+1.5 = 4
-5+3 = -2
(2+3)*(4-2)/3 = 3.3333333333333335

Interactive mode (enter expressions, 'quit' to exit):
> 2+3*4
= 14
> (10-5)/2
= 2.5
> quit
```

### Supported Expressions

- **Simple arithmetic**: `2+3`, `10-4`, `3*4`, `8/2`
- **Operator precedence**: `2+3*4` (equals 14, not 20)
- **Parentheses**: `(2+3)*4` (equals 20)
- **Nested parentheses**: `(2+(3*4))*2` (equals 28)
- **Decimals**: `2.5+1.5`, `3.14*2`
- **Negative numbers**: `-5+3`, `2*-3`
- **Complex expressions**: `(2+3)*(4-2)/3`

### Error Handling

The calculator provides clear error messages for:
- Empty expressions
- Invalid characters
- Division by zero
- Missing parentheses
- Unexpected tokens

## Testing

Run the test suite to verify functionality:

```bash
cargo test
```

The test suite includes 12 comprehensive tests covering:
- Basic arithmetic operations
- Operator precedence
- Parentheses handling
- Decimal and negative numbers
- Complex expressions
- Error conditions

## Architecture

The calculator uses a **recursive descent parser** with the following components:

1. **Tokenizer**: Converts input strings into tokens (numbers, operators, parentheses)
2. **Parser**: Implements grammar rules for expression evaluation
3. **Evaluator**: Computes results using proper operator precedence

### Grammar Rules

```
expression → term ((+|-) term)*
term       → factor ((*|/) factor)*
factor     → primary
primary    → NUMBER | '(' expression ')' | '-' primary
```

## Technical Details

- **Language**: Rust
- **Parsing**: Recursive descent parser
- **Number Type**: `f64` (64-bit floating-point)
- **Error Type**: `Result<f64, String>`
- **Dependencies**: None (uses only Rust standard library)

## Examples

### Basic Operations
```bash
> 5+3
= 8
> 10-4
= 6
> 7*6
= 42
> 15/3
= 5
```

### Operator Precedence
```bash
> 2+3*4
= 14
> (2+3)*4
= 20
> 8/2+3
= 7
> 8/(2+3)
= 1.6
```

### Advanced Features
```bash
> -5+10
= 5
> 3.14*2
= 6.28
> (2+(3*4))*2
= 28
> (10-5)*(3+2)/4
= 6.25
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Ensure all tests pass
5. Submit a pull request

## License

This project is open source and available under the [MIT License](LICENSE).

## Future Enhancements

- [ ] Scientific functions (sin, cos, tan, log, etc.)
- [ ] Variables and assignment
- [ ] Command history
- [ ] Configuration file support
- [ ] Expression history and replay
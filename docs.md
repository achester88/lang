## Types
---
The following data types are currently supported in lang
- Strings
- Integers (i32)
- booleans

## Variables Definitions
---
`type name = value`

Supported types:
- bool 
- int
- string

## Supported Operations
---
### Math
`intager symbol intager`

    / - + * %

## Strings
---
String uses the `"value"` format

Suported operations:
- string + string
- string * int

## Logic
---
### comparison

    == != < >

### logic
    && || !

## Output
---
`output(value: any)` - Output value to console

`outputln(value: any)` - Outputs value to a newline

## Controls Statments
---

**if**: Executes a statment if a codition is true 

**else**: Executes a statment if a if statment is false 

**else if**: Executes a statment if a if statment is false and if a new condition is true

Example:

    if(condition) {
        statement
    } else if (condition) {
        statement
    } else {
        statement
    }
**while**: Contiunes to execute a statment until a codition is false

Example:
```
    while(condition) {
        statement
    }
```
## Functions
---
functions are defined with the fn keyword like so:
```
fn foo() {
    outputln("hello functions");
}
```
they can be call by there name followed by opening and closing brackets:
```
foo();
```
functions additional can take in parameter of they three built in types by using *type name* format. they can then be accessed as predefined variables:
```
fn foo(bool b, int i, string s) {
    outputln(s);
}

foo(true, 0, "hello functions");
```
## Built-In Methods
---
`sleep(ms: number)` Sleep for defined milliseconds
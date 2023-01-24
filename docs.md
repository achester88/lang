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

Unsupported types:
- String

## Supported Operations
---
### Math
`intager symbol intager`

    / - + * %
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

## Built-In Methods
---
`sleep(ms: number)` Sleep for defined milliseconds
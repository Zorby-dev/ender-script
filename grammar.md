# atom

- **INT**
- **STRING**
- **IDENT**
- **(** _statement_ **)**

# math-expr-1

- _atom_ ( [ **\*/** ] _atom_ )\*

# math-expr-2

- _math-expr-1_ ( [ **+-** ] _math-expr-1_ )\*

# statement

- **LET IDENT : IDENT =** _math-expr-2_
- **FUNC IDENT ( )**
- **RAW STRING**
- _math-expr-2_

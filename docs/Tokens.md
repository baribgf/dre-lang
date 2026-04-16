# `Dre` Language Tokens

> N.B All tokens are case-sensitive.

> N.B No token can have length > `4KiB`.

| Token             | Description
|-------------------|-----------------------------------------------------|
| Identifier        | Any string beginning with a letter or `@` followed by any number of letters and digits, with at most one underscore separating each.
| Newline | `\n` character.
| Multi-line comment  | A string starting and ending with `---`.
| Single-line comment | A string starting with `--` and ending with newline.
| `use` keyword       | Used for bringing modules into scope.
| `let` keyword       | Used to declare variables, structs, enums, ...etc
| `struct` keyword | Used to define structures.
| `enum` keyword | Used to define enums.
| `prot` keyword | Used to define protocols (interfaces).
| `func` keyword | Used to define functions.
| `begin` keyword | Used to mark the beginning of a block.
| `end` keyword | Used to mark the ending of a block.
| `is` keyword | Used to impose protocols.
| `pub` keyword | Used to mark public struct regions.
| `self` keyword | Used to refer to struct instances.
| `return` keyword | Used to return values from functions.
| `until` keyword | Used to write `until-loop` statements.
| `while` keyword | Used to write `while-loop` statements.
| `for` keyword | Used to write `for-loop` statements.
| `do` keyword | Used to mark the beginning of a `do-end` block
| `in` keyword | Used to delimit between `for-loop` iteration variable binding and iterator expression.
| `break` keyword | Used to quit a loop.
| `skip` keyword | Used to interrupt an iteration of a loop.
| `if` keyword | Used to write conditional branching statements.
| `then` keyword | Used to mark the beginning of a conditional branching block.
| `else` keyword | Used to write default conditional branching statements.
| `true` keyword | Used to denote a `true` value literal.
| `false` keyword | Used to denote a `false` value literal.
| `not` keyword | Logic negation unary operator.
| `and` keyword | Logic conjunction operator.
| `or` keyword | Logic disjunction operator.
| `inc` keyword | Number increment operator.
| `dec` keyword | Number decrement operator.
| `const` keyword | Used to declare constants.
| Integer literal | Any sequence of digits with optional `-` leading sign.
| Floating-point literal | Any sequence of digits containing `.`, with optional `-` leading sign.
| String literal | Any sequence of UTF-8 characters enclosed between `“` and `”`.
| Char literal | Any UTF-8 character enclosed between two `'`s.
| `: `  | Spaced colon, for type identification.
| `::` | Double colon, for namespaced access.
| `=` | Used to assign constants, and to test equality.
| `<>` | Diamond operator, used to test non-equality.
| `<` | Less than operator.
| `<=` | Less than or equal operator.
| `>` | Greater than operator.
| `>=` | Greater than or equal operator.
| `(` | Opening parenthesis.
| `)` | Closing parenthesis.
| `[` | Opening bracket.
| `]` | Closing bracket.
| `, ` | Spaced comma, for separating elements.
| `->` | Right arrow, for function return types.
| `<-` | Left arrow, for assigning values to variables.
| `.` | Period, for struct member access.
| `..` | Double period, for specifying ranges.
| `+` | Addition operator.
| `-` | Substraction operator.
| `*` | Multiplication operator.
| `/` | Division operator.
| `%` | Modulo operator.
| `<<` | Shift-left operator.
| `>>` | Shift-right operator.
| `&` | Ampersand operator.
| `|` | Pipe operator.
| `^` | Hat operator.
| `~` | Tilde operator.

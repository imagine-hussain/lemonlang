
# LemonLang Grammar


### Notation

The following document specifies the LemonLang grammar in
[EBNF](https://en.wikipedia.org/wiki/Extended_Backus%E2%80%93Naur_form).

Non-terminals are denoted with `<angle-brackets>`.
Literals will be denoted with quotes `"("` to distinguish from
EBNF symbols and separators.

## Program

A program exists of individual modules.
These modules are linked together at compile time.

Modules may be nested.

```
<module> := ( <use-decl> | <func-decl> | <global-decl> | <mod-decl> )*

<use-decl> = "use" <use-paths> ";";

// TODO: Refactor
<use-path> = <ident> | "{" <use-path> ("," <use-path>)* "}";

```

## Identifiers

Identifiers can contain alphanumeric characters and underscores.
However, identifiers can only start with alphabetical characters
or underscores, not numbers.

Identifiers may also qualify what module they come from by using `::`.

Unqualified identifiers may not conflict with `<keyword>`s reserved
by the language.

```

<ident>             := <unqualified-ident> ("::" <unqualified-ident> )*

```

## Reserved Keywords

```
<keyword>   = "use"
            | "let"
TODO:
```

## Variable Declarations

Variables can be declared with the `let` keyword.
Not that any `<ident>` on the LHS must be an unqualified identifier.

```
<decl-expr>  = "let" <ident> "=" <expr> 
TODO
```


## Functions and Closures

The syntax for a closure is:

```
<closure>           = (<type> "from")?  <para-list> "->" <expr>
<paralist>          = "(" <proper-para-list>? ")"
<proper-para-list>  = <para-decl> ( "," <para-decl> )*
<para-decl>         = <ident> ":" <type>
```

Functions are treated as bindings to closures.
However, while closures may normally omit their type, when being passed
as arguments, when functions must have an explicit type. If there is
no type given for functions then, a return type of `()` is assumed.

<!-- TODO: add link to tilde -->
For a `<simple-expr>` closure that expects a unit return, the
`~` operator can be used to discard the return value of the expression.


```
<function-decl>     = "let" <ident> "=" <closure> ";"
```
Note that `<function-decl>` is not an intrinsic part of the
grammar and exists only for demonstrative purposes.
In practice, a `<function-decl>` is not treaty any differently
to a normal `<decl>.`

### Example

```rust
// Closure binding: Explicit Type Required
let double = i32 from (x: i32) -> x * 2
// Closure with compound statement
let double = i32 from (x: i32) -> {
    foo(x);
    x * 2
};
// Closure with discarded return using tilde
let remove_top = (v: Vec<i32>) -> ~v.pop();
// Closure with discarded return using compound statement
let remove_top = (v: Vec<i32>) -> { v.pop(); };

// As argument - Type ommitted
foo_iter()
    .map((x: i32) -> x * 2) // Type omitted
    .map(i32 from (x: i32) -> x * 2) // Explicit Type
```

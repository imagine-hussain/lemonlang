
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
<keyword> = "use"
            | "for"
            | "break"
    ... currently incomplete: TODO
```



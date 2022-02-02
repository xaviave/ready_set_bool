# Convert to NNF

## Steps

1. Eliminate double negation
2. Eliminate `>` and `=`
3. Apply `De Margan'Law`, push negation to every vars
4. Apply distributivity

## Parsing

- Start parsing with this: https://rustc-dev-guide.rust-lang.org/the-parser.html
- Use stack to store data
- Add Method to BinaryTree

        - get_nnf() -> `String` : transform proposionnal formula to `Negation Normal Form` formula
        - get_cnf() -> `String` : transform proposionnal formula to `NNF` then `Conjonctive Normal Form`
        - is_satisfiable() -> `bool`: determine if a formula can be `TRUE`

- Create custom Abstract class for every token:

        - var = neg | left | right
        - fn collapse() -> `bool` : recursively vall each leaves and get the final `VAR` by computing every operations (described as token)
                

    - ID => NEGATION: `bool` | VAR: `char`
    - OR -> NEGATION: `bool` | left: `Abstract class `| right: `Abstract class`
    - XOR -> NEGATION: `bool` | left: `Abstract class `| right: `Abstract class`
    - AND -> NEGATION: `bool` | left: `Abstract class `| right: `Abstract class`
    - IMPLY -> NEGATION: `bool` | left: `Abstract class `| right: `Abstract class`
    - EQUIVALENCE -> NEGATION: `bool` | left: `Abstract class `| right: `Abstract class`

- Apply the above laws
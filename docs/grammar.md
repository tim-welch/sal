```
program            -> definition* expression ;

definition         -> "def" identifier "=" expression ";" ;

expression         -> term ;
term               -> factor ( ("-" | "+") factor )* ;
factor             -> primary ( ("*" | "/") primary )* ;
primary            -> literal | grouping | identifier ;
grouping           -> "(" expression ")" ;

identifier         -> (not digit & not punctuation & not whitespace)(not punctuation and not whitespace)* ;
literal            -> NUMBER ;
digit              -> "0".."9" ;
punctuation        -> "(" | ")" ;
whitespace         -> any unicode character classified as whitespace
```
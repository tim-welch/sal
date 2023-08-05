```
expression         -> term ;
term               -> factor ( ("-" | "+") factor )* ;
factor             -> primary ( ("*" | "/") primary )* ;
primary            -> literal | grouping
literal            -> NUMBER ;
grouping           -> "(" expression ")"
```
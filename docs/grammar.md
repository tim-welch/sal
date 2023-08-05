```
expression         -> literal
                    | binary
                    | grouping ;
                    
literal            -> NUMBER ;
grouping           -> "(" expression ")"
primary            -> literal | grouping
binary             -> expression operator expression ;
operator           -> "+" | "-" | "*" | "/" ;
```
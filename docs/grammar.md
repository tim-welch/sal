```
expression         -> literal
                    | binary
                    | grouping ;
                    
literal            -> NUMBER ;
grouping           -> "(" expression ")"
binary             -> expression operator expression ;
operator           -> "+" | "-" | "*" | "/" ;
```
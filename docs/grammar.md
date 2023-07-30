```
expression         -> literal
                    | binary ;
                    
literal            -> NUMBER ;
binary             -> expression operator expression ;
operator           -> "+" | "-" | "*" | "/" ;
```
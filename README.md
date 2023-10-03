Render asciimath in terminal

Display math formulas as plain text using unicode characters.

This is work in progress, several important parts of spec are not supported yet.

Examples:

> f(x)=sqrt x + 1/2x^2
```
        ▁  1  2
f(x)=2╲╱x+───x 
           2   
```

> sqrt x
```
  ▁
╲╱x
```

Limitations:
 - cancel cannot be implemented in general. It is possible to replace it with strike-through line for text
literals ( cancel x -> x̶  or x̷ or x̸  ), there is no way to do it for expressions.
 - we don't control fonts, therefore fancy text effects are also impossible or very limited (they can be imitated for text literals)

Render asciimath in terminal

Display math formulas as plain text using unicode characters.

This is work in progress, several important parts of spec are not supported yet.

Examples:

> The Discrete Fourier Transform is defined as 
> X^k=1/N sum_(n=0)^(N-1)x_n * e^(-ik (2pi)/N n) = 1/N sum_(n=0)^(N-1)x_n[cos(k (2pi)/N n) -i sin(k (2pi)/N n)]
```
                  2π                                      
              -ik────n                                    
 k  1  N-1         N    1  N-1  ⎡   ⎛  2π  ⎞     ⎛  2π  ⎞⎤
X =───∑   x ⋅e        =───∑   x ⎥cos⎜k────n⎜-isin⎜k────n⎜⎥
    N  n=0 n            N  n=0 n⎣   ⎝   N  ⎠     ⎝   N  ⎠⎦
```

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

Render asciimath in terminal

This is Rust crate for displaying math formulas as plain text using unicode characters, useful for applications that work in terminal.

This is work in progress, several important parts of asciimath spec are not supported yet, but it's enough to be useful.

See http://asciimath.org/ for asciimath spec


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

> The Cardano's formula is:

> root 3 (- q/2 + sqrt (q^2/4 + p^3/27)) + root 3 (-q/2 + sqrt(q^3/4+p^3/27))

```
      ▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁       ▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁▁
     ╱           ▁▁▁▁▁▁▁▁▁      ╱           ▁▁▁▁▁▁▁▁▁
    ╱   q       ╱  2    3      ╱   q       ╱  3    3 
   ╱  -───+    ╱  q    p      ╱  -───+    ╱  q    p  
 3╱     2  ╲  ╱  ────+────  3╱     2  ╲  ╱  ────+────
╲╱          ╲╱     4   27 +╲╱          ╲╱     4   27 
```

> Euler's Identity:

```
 iπ    
e  +1=0
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


usage:
```
cargo add asciimath-text-renderer
```
if you downloaded source, you can run example:
```
cargo run --example render 'sqrt(2)'
  ▁
╲╱2
```

Similar projects:

[simonschmidt/Asciimat](https://github.com/simonschmidt/Asciimath) Convert plain-text math to unicode (Python)
[Nota](https://kary.us/nota/)[pouyakary/Nota](https://github.com/pouyakary/Nota) (Haskell)
[Diagon](https://github.com/ArthurSonzogni/Diagon) Beside formulas, Diagon renders many types of diagrams. (C++)




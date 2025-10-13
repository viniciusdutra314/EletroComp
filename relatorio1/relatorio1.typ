#import "@preview/codly:1.3.0": *
#import "@preview/tabut:1.0.2"
#import "@preview/codly-languages:0.1.1": *
#show: codly-init.with()
#codly(languages:codly-languages)
#set page(numbering: "1", number-align: center)
#set text(lang: "pt")

= Equação De Laplace

$nabla^2 V=(partial^2 V)/(partial x^2)+(partial^2 V)/(partial y^2)+(partial^2 V)/(partial z^2)=0$

Nós precisamos fazer uma discretização,
assumindo $Delta x=Delta y=Delta z=1$

$(partial^2 V)/(partial x^2)=(V(x+Delta x)-V(x)-V(x)+V(x- Delta x))/(Delta x^2)=(V(x+Delta x)-2V(x)+V(x-Delta x) )/(Delta x^2)$

Jogando essa aproximação na equação de Laplace

$V(x,y,z)=1/6 (V(x+Delta x,y,z)+V(x-Delta x,y,z)+V(x,y+Delta y,z)+V(x,y -Delta y,z)
  + V(x,y,z+Delta z)+V(x,y,z-Delta z))$

#pagebreak()
 #outline()
#pagebreak()

= Implementação em Rust

== Por que Rust?
A linguagem de programação Rust foi escolhida para esse trabalho por duas razões: Alto desempenho e programação genérica.  Como os exercícios são basicamente somente variações de se resolver a equação de Poisson ($nabla^2 V=-rho/epsilon$) para diferentes casos especiais, a programação genérica oferece formas de resolvermos o problema de uma só vez sem a necessidade de duplicação de código.

Diferente do uso de funções virtuais em orientação a objetos, o uso de generics no código não implica em impacto em runtime, pois, todas as funções são resolvidas *estaticamente*, consistindo em basicamente uma substituição simples de preprocessamento na compilação.

Somente foi utilizado closures e não ponteiros para funções, o que significa que as funções podem 
serem `inline` mesmo no caso de funções de alta ordem

=== Dimensionalidade e tipo númerico genérico
Com a biblioteca #link("https://docs.rs/ndarray/latest/ndarray","ndarray")  é possível utilizar arrays multidimensionais de rank $N$ genérico, garantido que o rank é conhecido em 
tempo de compilação, não há impactos no desempenho.

O tipo numérico utilizado não é especificado, só é necessário que ele se comporte 
como um Float de acordo com o #link("https://docs.rs/num-traits/latest/num_traits/","num-traits") do Rust. Com isso é possível testar o código para f16,f32,f64... etc, assim verificando
se a precisão do float utilizado interfere ou não no resultado final.

Com isso é possível criar uma função que cria um array que representa um hipercubo
de dimensão D com tipo numérico T
```rust
fn create_hypercube<T,D>(n: usize) -> (Array<T, D>)
where
    T:Float,
    D:Dimension

```

== As 3 abstrações principais
Cada problema pode serem pensando em 3 coisas extremamente correlacionadas
- Condições inicias
- Média dos vizinhos
- Método (Jacobi ou SOR)
=== Condição Inicial
Uma condição inicial é essencialmente o potencial elétrico em cada ponto e se aquele ponto tem potencial fixo ou 
não. Portanto, dois arrays de dimensão D, um do tipo T e outro do tipo bool. 

Podemos criar qualquer condição inicial que for necessária, somente para demonstração eu criei uma função que recebe uma imagem qualquer e retorna uma condição inicial. 
```rust
fn generic_image<T: Float>(path: &str) -> (Array2<T>, Array2<bool>)
```

A função considera os pixels escuros da imagem como sendo $V=1$ fixo e o resto $V=0$, as bordas são fixas em 0, para exemplo segue uma ilustração da Torre Eiffel




#figure(
    image("./results/eifel.jpg", width: 50%),
    caption: [Imagem de entrada]
)

#figure(
  image("./results/generic_image.png", width:66%),
  caption: [Potencial elétrico final resultante da simulação.],
)


Infelizmente o ndarray não é genérico
ao ponto de permitir customizar o armazenamento interno para aproveitar memória em matrizes esparsas, simétricas,antissimétricas etc. Logo, todas as matrizes serão densas e cada elemento terá um endereço único (arrays de booleanos não são densamente empacotados com máscaras binárias). Isso não é um problema relevante para os exemplos com $N$ entre $10^2$ e $10^3$  

As condições inicias se encontram em *`simulation/src/initial_conditions.rs`*
=== Média dos Vizinhos
Dado um ponto $P$, queremos a média do potencial elétrico nos pontos vizinhos a $P$. Para cada condição de contorno e simetria diferente, teremos uma noção diferente de vizinhança. Fica claro a necessidade de uma função que dado um potencial de tipo numérico T / dimensão D e um ponto de mesma dimensão D, retorne a média no tipo T 
```rust
NeighborAverage: Fn(&Array<T, D>, Index<D>) -> T
```

Essas médias se encontram em *`simulation/src/neighbor_averages.rs`*

=== Métodos
Para aplicar um método é necessário as condições iniciais (potencial inicial, quais pontos são fixos e a densidade de carga), a função que calcula a média da vizinhança e uma variação máxima de uma iteração para a outra.

A função retorna o potencial elétrico final e o número de iterações

#figure(raw(
"fn jacobi_method<T, D, NeighborAvg>(
    initial_potential: ArrayView<T, D>,
    fixed_points: ArrayView<bool, D>,
    charge_density: Option<ArrayView<T, D>>,
    neighbor_average: NeighborAvg,
    error_tolerance: T,
) -> (Array<T, D>, usize)
where
    T: Float,
    D: Dimension 
    NeighborAvg: Fn(&Array<T, D>, D) -> T",lang: "rust",block:true),caption:"Assinatura geral dos métodos")


Como só é introduzido cargas elétricos nos últimos exercícios, a densidade de carga é opcional, para evitar verificações desnecessárias, os métodos são funções que despacham para duas variações delas mesmas, uma com densidade de carga e outra sem. Assim, essa verificação da presença ou não desse array é feita fora do loop principal, o que mostrou ser uma diferença importante em performance

Os métodos se encontram em *`simulation/src/methods.rs`*

= Exercícios
== 5.1
#box(
  fill: luma(240),
  inset: 10pt,     
  outset: 5pt,    
  radius: 3pt,   
  [
Solve for the potential in the prism geometry in Figure 5.4.
  ]
)


#figure(
  grid(
    columns: (auto,auto),
    gutter: 10em,
    image("results/ex01_potential_small.jpg", width: 150%),
    image("results/ex01_potential_small_wire.jpg", width: 150%),
  ),
  caption: "Potencial elétrico figura 5.4"
)

Esse exercício consiste em criar um grid $N times N$ com bordas de potencial fixo
$V=0$ e um quadrado interno com $V=1$, como  o esperado, a tensão lentamente decai
lentamente do quadrado até a borda.

Com um quadrado com um tamanho pequeno (10% de N), o problema parece ter uma simetria
esférica, mas isso não é verdade, com um grande grande o suficiente para ficar próxima das bordas
é possível ver que o problema só tem a simetria dos quadrantes

#figure(
  grid(
    columns: (auto,auto),
    gutter: 10em,
    image("results/ex01_potential_big.jpg", width: 150%),
    image("results/ex01_potential_big_wire.jpg", width: 150%),
  ),
  caption: "Potencial elétrico figura 5.4"
)

#codly-range(7)
#figure(
raw(read("simulation/src/bin/cap05_ex01.rs"),lang: "rust",block:true),caption: "Código exercício 1")

#pagebreak()
== 5.2

#box(
  fill: luma(240),
  inset: 10pt,     
  outset: 5pt,    
  radius: 3pt,   
  [
    #text(weight: "bold")[
      Exercício 5.2:
    ]
Use the symmetry of the problem described in Figure 5.4 to write a program that solves for V by calculating the potential in only one quadrant of the x-y plane.
  ]
)

O problema é basicamente idêntico ao anterior, mas como o problema é o mesmo em todos os quadrantes, podemos usar que $V(x,y)=V(|x|,|y|)$ e só simular um quadrante. Isso pode ser feito facilmente aplicando uma *view* sobre as condições inicias, fazendo um slicing para só pegar o canto superior direito. A própria função `jacobi_method` espera receber views porque as vezes é mais fácil em pensar no problema como um todo e então "cortar" pedaços fora dele

#codly-range(7)
#figure(
raw(read("simulation/src/bin/cap05_ex02.rs"),lang: "rust",block:true),caption: "Código exercício 2")


#figure(
  image("results/ex02_eletric_potential_partial.jpg"),
  caption: "Potencial elétrico figura 5.4 quadrante direito superior"
)


Simulando somente $1/4$ do problema se mede um speedup importante no tempo de execução da simulação, seria esperado um speedup de 4 mas outro fator importante é que com o método converge mais rápido porque pelas condições de contorno, as simetrias já são impostas.
#align(center)[
  #table(
    columns: (auto, auto, auto),
    stroke: 0.4pt,
    align: center + horizon,
    [Simulação], [Tempo de Execução], [Speedup],
    [Grid Completo], [57.5 s], [1.0x],
    [Um Quadrante], [10.8 s], [5.3x],
  )
]




A única coisa mais sútil são as condições de contorno, que já não são triviais como o execício anterior.
Usando `pattern matching` do Rust é possível expressar elas de maneira bem direta, estamos essencialmente impondo que  $V(x,y)=V(|x|,|y|)$

#codly-range(25,end:53)
#figure(
raw(read("simulation/src/neighbor_averages.rs"),lang: "rust",block:true),caption: "Condições de contorno exercício 2")


#codly-range(3,end:11)
#figure(
raw(read("plot_scripts/cap05_ex02.py"),lang: "rust",block:true),
caption: "Condições de contorno exercício 2")

Com somente o quadrante superior direito e as simetrias do problema, podemos 
criar a imagem toda realizando uma "colagem" de quatro cópias do array menor
realizando rotações
#figure(
  image("results/ex02_eletric_potential_entire.jpg"),
  caption: "Potencial elétrico figura 5.4 quadrante direito superior (colagens)"
)

#pagebreak()
== 5.3
#box(
  fill: luma(240),
  inset: 10pt,     
  outset: 5pt,    
  radius: 3pt,   
  [
    #text(weight: "bold")[
      Exercício 5.3:
    ]
 Use the symmetry of the capacitor problem (Figure 5.6) to write a program that obtains the result by calculating the potential in only one quadrant of the x-y plane.
  ]
)

A simetria usada nesse exercício é que $V(x,y)=-V(x,y)$ e que $V(x,y)=V(x,-y)$, a mesma técnica de realizar
um slicing das condições inicias e então realizar a colagem na visualização.


#figure(
  image("results/ex03_eletric_potential_colormap.jpg"),
  caption: "Placas paralelas figura 5.6"
)


#figure(
  image("results/ex03_eletric_potential_wire.jpg"),
  caption: "Placas paralelas figura 5.6"
)

#pagebreak()
== 5.4
#box(
  fill: luma(240),
  inset: 10pt,     
  outset: 5pt,    
  radius: 3pt,   
  [
    #text(weight: "bold")[
      Exercício 5.4:
    ]
Investigate how the magnitude of the fringing field of a parallel plate capacitor, that is, the electric field outside the central region of the capacitor in Figure 5.6, varies as a function of the plate separation.
  ]
)

#figure(
  image("results/teste.jpg"),
  caption: "Placas paralelas com variados espaçamentos"
)
Chamemos o espaçamento entre as placas de $L$ e o tamanho das placas de $h$. O regime do capacitor 
"infinito" ocorre quando $h>>L$. Nesse regime, o campo elétrico entre as placas deve ser aproximadamente
uniforme com direção do capacitor positivo ao negativo, além disso, o campo fora deve ser zero.

Como o colormap é compartilhado entre as figuras, fica evidente que conforme as placas ficam mais próximas
o potencial fica cada vez mais branco fora das placas, ou seja, $V arrow 0$
#pagebreak()
== 5.5
#box(
  fill: luma(240),
  inset: 10pt,     
  outset: 5pt,    
  radius: 3pt,   
  [
    #text(weight: "bold")[
      Exercício 5.5:
    ]
Study the accuracy of the relaxation method by solving any of the problems considered in this section with several different values of the convergence (error) limit. Compare the results for $V$ and $arrow(E)$, and estimate how the actual error in either of these quantities compares to the convergence limit. Theoretically, the number of iterations required to achieve p significant digits should be proportional to p for the Jacobi, Gauss-Seidel, and SOR methods. Compare your results above to this theoretical expectation.
  ]
)

É impossível falar da precisão numérica de um método sem se referir ao tipo numérico usado,
como o tipo numérico é genérico no código, vamos comparar os dois floats mais comuns, precisão
simples (f32) e precisão dupla (f64), ambos tipos do Rust seguem o padrão IEEE 754
#figure(
  image("results/ex05_comparison.png"),
  caption: "Precisão dos métodos em f32 e f64 (capacitor de placas paralelas N=500)"
)<fig:f32_f64>

A @fig:f32_f64 mostra que os tipos f32/f64 não fazem muita diferença até $p=5$, isso ocorre porque 
o $epsilon_(f 3 2 ) approx 10^(-7)$ enquanto $epsilon_(f 6 4) approx 2 times 10^(-16)$, então para precisões altas o f32
começa a já perder precisão pois ele não consegue representar a diferença entre dois números muito próximos.

O $V_(i d e a l)$ foi calculado usando precisão quadrupla (f128) com erro $10^(-30)$ para ser a referência. A 
diferença entre essa referência e o valor calculado foi feita acumulando o absoluto da diferença entre os arrays.
É visível que em f64 o $V(x,y)$ resultante linearmente converge para $V_(i d e a l)$, enquanto em f32 existe um ponto
de estagnação

#pagebreak()
== 5.6
#box(
  fill: luma(240),
  inset: 10pt,     
  outset: 5pt,    
  radius: 3pt,   
  [
    #text(weight: "bold")[
      Exercício 5.6:
    ]
 Calculate the electric potential and field near a lightning rod. Model this as a very long and narrow metal rod held at a high voltage, with one end near a conducting plane. Of special interest is the field near the tip of the rod.
  ]
)
#pagebreak()
== 5.7

#box(
  fill: luma(240),
  inset: 10pt,     
  outset: 5pt,    
  radius: 3pt,   
  [
    #text(weight: "bold")[
      Exercício 5.7:
    ]
 Write two programs to solve the capacitor problem of Figures 5.6 and 5.7, one using the Jacobi method and one using the SOR algorithm. For a fixed accuracy (as set by the convergence test) compare the number of iterations, $N_(i t e r)​$, that each algorithm requires as a function of the number of grid elements, $L$. Show that for the Jacobi method $N_(i t e r) prop L^2$, while with SOR $N_(i t e r) prop L$.

  ]
)
Rodando o mesmo código para diferentes valores de N, foi salvo em um .csv a quantidade 
de iterações e o tempo de execução em segundos, como consta na @ex07-data.

#let data = csv("results/ex07_comparison.csv")

#figure(align(center, table(
  columns: data.first().len(),
  align: center + horizon,
  stroke: 0.4pt,
  ..data.flatten()
)),caption: "Dados Jacobi vs SOR",
)<ex07-data>

Aplicando um fitting de $O(n^2)$ no método de Jacobi e um $O(n)$ no SOR fica evidente que 
os métodos tem o comportamento assintótico esperado. Uma vez que $n^2/n=n$ o speedup é esperado
que cresça linearmente, ou seja, quanto maior o problema mais vantajoso é o método de SOR sobre o Jacobi.

Em termos de memória 


#figure(
  image("results/ex07_comparison.png"),
  caption: "Performance Jacobi vs SOR"
)



#pagebreak()
== 5.8
#box(
  fill: luma(240),
  inset: 10pt,     
  outset: 5pt,    
  radius: 3pt,   
  [
    #text(weight: "bold")[
      Exercício 5.8:
    ]
    Extend our treatment of a point charge in a metal box to deal with the case in which the charge is located near one face of the box. Study how the equipotential contours are affected by the proximity of a grounded surface (the face of the box).
  ]
)
#pagebreak()

== 5.9
#box(
  fill: luma(240),
  inset: 10pt,     
  outset: 5pt,    
  radius: 3pt,   
  [

In spherical coordinates Poisson’s equation has the form
$1/r (partial^2)/(partial r^2)(r V)=-(rho)/epsilon $.


where we have assumed a spherically symmetric problem so that $V$ is a function only of the distance from the origin. Solve this equation numerically using the relaxation method for a point charge at $r=0$, imposing $V=0$ some large distance away. Compare your result with Coulomb’s law, (5.21). Hint: This problem is made difficult by the factor of $1/r$ on the left side of (5.22) and its effect on constructing a numerical solution, especially when the charge distribution is a singular function at $r=0$, as is the case for a point charge. One way to deal with this problem is to instead give the “point” charge a small but nonzero spatial size; that is, assume that there is a uniform charge density inside a small sphere of radius $r_(m i n)$. If you take this approach, be sure to pick a grid size smaller than $r_(m i n​)$. Convenient parameter choices are $r_(m i n​)=0.2$ with a grid size of 0.025, and $V=0$ imposed at $r=5$, but you should also try other values. Compare your result for $V(r)$ with Figure 5.10.
  ]
)

Primeiramente, é necessário discretizar a equação de Poisson em coordenadas esféricas, uma forma simples de se fazer é isso usando uma biblioteca de cálculo simbólico como o #link("https://docs.sympy.org/latest/explanation/special_topics/finite_diff_derivatives.html","Sympy")

#codly(zebra-fill: none)
#codly-range(0,end:13)
#codly(
  annotations: (
    (
      start: 5,
      end: 8,
      content: block(
        width: 2em,
        rotate(
          -90deg,
          align(center, box(width: 100pt)[Discretização escolhida])
        )
      )
    ), 
  )
)
#codly(highlights: (
  (line: 6, start: 0, end: none, fill: green),
  (line: 7, start: 0, end: none, fill: green),
))
#figure(raw(
  read("plot_scripts/cap05_ex09_equation.py"),
  block:true,
  lang:"python"
),caption: "Discretização da equação de Poisson")
#pagebreak()

== 5.10

#box(
  fill: luma(240),
  inset: 10pt,     
  outset: 5pt,    
  radius: 3pt,   
  [
 Investigate the performance of the simultaneous over-relaxation algorithm for a point charge in two and three dimensions. Hint: In two dimensions we know the optimum choice of the over-relaxation parameter, α in (5.18). In three dimensions you should determine the optimum choice of this parameter by observing the speed of convergence for different values of α. How sensitive is the convergence to the value of α?
  ]
)



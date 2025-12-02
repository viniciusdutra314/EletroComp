#import "@preview/codly:1.3.0": *
#import "@preview/tabut:1.0.2" as tabut
#import "@preview/codly-languages:0.1.1": *
#show: codly-init.with()
#codly(languages: codly-languages)
#set page(numbering: "1", number-align: center)
#set text(lang: "pt")
#set page(
  paper: "a4",
  margin: (top: 3cm, bottom: 2.5cm, left: 2.5cm, right: 2.5cm),
)
#set par(justify: true)
#set math.equation(block:true,  numbering: "(1)")
#set document(
  title: "Projeto 2 - Campo Magnético por Integração Numérica",
  author: "Vinícius Sousa Dutra",
)

#align(center)[

  #image("plots/ifsc_logo.png", width: 15cm)
  // Centered University and Institute Name (Repeated)

  // Flexible vertical space to push the title down.
  #v(2.5fr)

  // Main Title Section
  #text(30pt)[
    Projeto 2 - Campos magnéticos por integração numérica
  ]
  #v(1.5em)
  #text(20pt)[
    Eletromagnetismo Computacional (7600036)
  ]

  #v(4fr)
]

#block[
  #strong[Professor:]\
  Guilherme Matos Sipahi
  #v(2em)

  #strong[Aluno:]\
  Vinícius Sousa Dutra (13686257)
]

#v(1fr)

// Date Section
#align(center)[
  02 de dezembro de 2025
]
#pagebreak()

== Exercício 5.11 e 5.12

#box(
  fill: luma(240),
  inset: 10pt,     
  outset: 5pt,    
  radius: 3pt,   
  [
  
Calculate the field from a straight wire using Simpson’s rule (see Appendix E), and compare it with the result obtained from (5.25) for the same grid size, $Delta z$.
  ]
)

#box(
  fill: luma(240),
  inset: 10pt,     
  outset: 5pt,    
  radius: 3pt,   
  [
Evaluate (5.25) for different grid sizes and compare the results with Ampere’s law. Derive a rule of thumb concerning how small the grid size must be in comparison with the distance from the wire, in order for the calculated field to be within 5 percent of the exact result.
  ]
)

Como os exercícios 5.11 e 5.12 são muito similares, ambos envolvendo a comparação entre os métodos de Riemann e Simpson para o cálculo do campo magnético gerado por um fio retilíneo finito, optei por resolvê-los juntos nesta seção.
 

O método da soma de Riemann aproxima a integral de uma função dividindo a área sob a curva em uma série de retângulos e somando suas áreas, o que vem diretamente da definição do que é integração.

$ integral_a^b f(x) d x= lim_(n -> infinity) sum_(i=0)^n  f(a+i(b-a)/n)(b-a)/ n $

#codly-range(18,end:22)
#figure(
  raw(read("src/magnetic_field.jl"), lang: "julia", block: true),
  caption: "Método de Riemann",
)


Já o método de Simpson oferece uma aproximação mais sofisticada, em vez de retângulos, ele utiliza uma aproximação parabólica  para se ajustar a pequenos segmentos da curva. A fórmula abaixo considera N par,
por simplicidade vamos desconsiderar o caso impar (retornando um erro na implementação).

$ integral_a^b f(x)d x approx (Delta x)/3(f(a) +f(b)+ 4 sum_(i=í m p a r e s)f(x_i)+2sum_(i=p a r e s)f(x_i)) $

#codly-range(8,end:16)
#figure(
  raw(read("src/magnetic_field.jl"), lang: "julia", block: true),
  caption: "Método de Simpsons",
)


O método de Simpson geralmente converge para o valor real da integral de forma muito mais rápida e com maior precisão do que a soma de Riemann para o mesmo número de subdivisões. Isso ocorre essencialmente porque um método é uma aproximação linear enquanto outro é uma aproximação quadrática

Nesse caso específico estamos calculando o campo magnético gerado por um fio retilíneo finito de tamanho total $2L$ a uma distância $d$, estamos colocando o sistema de coordenadas para que a integração ocorra variando $z$, portanto, o tamanho da discretização é $d z$,
para esse caso específico temos uma expressão analítica para o campo magnético, que é dada por:

$ ||B||= (mu_0 I)/(4 pi d)L/sqrt((L/2)^2 + d^2) $  <eq:b_analitico_fio>


Como ilustrado na @img:riemman_vs_simpsons, variamos $d z$ para cada método e comparamos o erro relativo em relação ao valor analítico do campo magnético dado pela @eq:b_analitico_fio

#figure(
  image("plots/ex11_12.png"),
  caption: "Comparação entre os métodos de Riemann e Simpson",
) <img:riemman_vs_simpsons>

Como o esperado, o método de Simpson apresenta uma taxa de convergência muito maior do que o método da soma de Riemann, o que é evidenciado pela inclinação mais acentuada da curva correspondente ao método de Simpson no gráfico de erro relativo em função $d z$.

Apartir de um certo tamanho de malha ambos os métodos começam a diminuir a precisão, isso ocorre devido à precisão limitada dos números de ponto flutuante, no caso desse código f64.

#pagebreak()
== Exercício 5.13
#box(
  fill: luma(240),
  inset: 10pt,     
  outset: 5pt,    
  radius: 3pt,   
  [
    Calculate the value of π by using numerical integration to estimate the area of a circle of unit radius. Observe how your estimate approaches the exact value (3.1415926…) as the grid size in the integration is reduce
  ]
)

Considerando o $1/4$ de circunferência $f(x)=sqrt(1-x^2)$ em que $0<=x<=1$, calculando a sua área através do método de Simpsons obtemos $A approx pi/4$



#let data = csv("results/ex13_simpson.csv")

#figure(
  table(
    columns: 3,
    [log10(N)], [Aproximação de $pi$], [Log10(|Erro|)],
    ..data.flatten(),
    align: center,
  ),
  caption: [Aproximação de $pi$ por integração]
)


Observamos uma relação linear no crescimento de log(N) e o decréscimo de log10(|Error|), a constante linear é aproximadamente 1.5, existe portanto uma lei de potência relacionando as duas grandezas. O método de Simpsons é superlinear, ou seja, o erro diminui mais rapidamente do que uma relação linear conforme aumentamos o número de pontos de integração N. Nesse caso específico a potência parece ser 1.5

#pagebreak()
== Exercício 5.14
#box(
  fill: luma(240),
  inset: 10pt,     
  outset: 5pt,    
  radius: 3pt,   
  [
    Write a program to calculate the magnetic field for your favorite current distribution. One possibility is a pair of loops of radius r, with one loop lying in the x-y plane and the other in the y-z plane. Another possibility is the solenoid considered in Figure 5.17.
  ]
)

Escolhi calcular o campo magnético gerado por um solenoide, pois o resultado esperado é mais simples de visualizar,o solenoide possui raio $R$ e comprimento $L$, percorrido por uma corrente $I$.

Como o solenoide possui uma geometria helicoidal, precisamos criar um código que calcule o campo magnético gerado por uma curva arbitrária no espaço, essencialmente o código implementa a lei de Biot-Savart calculando a integral de linha numericamente por Simpsons.

$ B(arrow(r))=(mu_0)/(4 pi) integral_gamma I ((d l)/(d gamma) times r')/(|r'^3|) d gamma $

#codly-range(25,end:41)
#figure(
  raw(read("src/magnetic_field.jl"), lang: "julia", block: true),
  caption: "Campo magnético gerado por uma curva arbitrária",
) <code:magnetic_field_solenoid>

Utilizando a @code:magnetic_field_solenoid, calculei o campo magnético gerado por um solenoide parametrizando o solenoide como:

$ arrow(r)(gamma)=(r cos gamma,r sin gamma, L / (2π  N) gamma) $

O resultado da simulação se encontra na @img:solenoid_3d, onde podemos observar que o campo magnético é mais intenso no interior do solenoide, com direção a favor de z  e diminui conforme nos afastamos do eixo do solenoide.

#figure(
  image("plots/ex14_solenoid_3d.png"),
  caption: "Campo magnético gerado por um solenoide",
) <img:solenoid_3d>
#pagebreak()

== Exercício 5.15
#box(
  fill: luma(240),
  inset: 10pt,     
  outset: 5pt,    
  radius: 3pt,   
  [
Consider the magnetic field produced by a set of two coils that are both centered on the z axis, are both parallel to the x-y plane, and are both of radius r (see Figure 5.18). Let the separation between the coils also be r. These are called Helmholtz coils and are noteworthy because they produce a particularly uniform field near the point centered between them. Calculate numerically the field produced by these coils both on the z axis (where you can compare with the exact result from Biot-Savart law), and along the x axis.
  ]
)

Como o nosso código calcula somente o campo magnético gerado por *uma* curva arbitrária, podemos usar o princípio da superposição ($arrow(B)_(t o t a l)=arrow(B_1) + arrow(B_2)$) e somar o campo 
de cada bobina individualmente. Vamos supor que ambas as bobinas possuem raio $R$ e corrente $I$, e estão posicionadas em $z=-R/2$ e $z=R/2$.

$ arrow(r)_(b o b i n a 1)=(R cos(gamma), R sin(gamma), -R/2) $ 

$ arrow(r)_(b o b i n a 2)=(R cos(gamma), R sin(gamma), +R/2) $

O campo magnético no eixo z pode ser calculado analiticamente como:
$ ||arrow(B)(z)||= (mu_0 I R^2)/(2(R^2 + (z - R/2)^2)^(3/2)) + (mu_0 I R^2)/(2(R^2 + (z + R/2)^2)^(3/2)) $ <eq:b_analitico_helmholtz>

Usamos a @eq:b_analitico_helmholtz para comparar o resultado analítico com o resultado numérico obtido através da simulação, a comparação só será feita no eixo z, pois no outro eixo não temos o resultado analítico.

#figure(
  image("plots/ex15_helmholtz.png"),
  caption: "Campo magnético gerado por bobinas de Helmholtz",
) <img:helmholtz_coils>

Os resultados da @img:helmholtz_coils são o que esperávamos, o campo magnético é
muito uniforme próximo ao centro das bobinas, devido as 
derivadas de $B$ em relação as coordenadas serem zero nesse ponto.
#pagebreak()

== Exercício 5.16
#box(
  fill: luma(240),
  inset: 10pt,     
  outset: 5pt,    
  radius: 3pt,   
  [
Calculate the magnetic field both inside and outside a coil wrapped on a torus. Be sure to compare your result for B on the axis of the torus with the exact answer.
  ]
)

Para encontramos a curva $gamma$ associada ao toroide, baste notarmos que existe uma relação entre os ângulos da parametrização, precisamos que se um ângulo complete uma volta o outro percorra $N$ voltas, por tanto podemos reduzir a dimensão da superfície toroidal para uma reta, usando o vinculo: $theta/(phi)=N$

$ arrow(r)=((R+r sin theta)cos(theta N),(R+ r sin theta)sin (N theta),r cos (theta)) $

Onde $R$ é o raio maior do toroide, $r$ é o raio menor e $N$ é o número de voltas da espira em torno do toroide, basta rodarmos o código para essa curva para obter o campo magnético gerado pelo toroide.

#figure(
  image("plots/ex16_torus.png"),
  caption: "Campo magnético gerado por um toroide",
) <img:torus_coil>

O resultado analítico que esperamos é:

$
||B|| = cases(
  (mu_0 N I)/ (2 pi r), & "dentro",
  0, & "fora "
)
$ <eq:b_analitico_torus>

Como é visível na @img:torus_coil,a simulação modela muito bem o resultado esperado, se observa uma pequena curva de transição dentro/fora do toroide que não existe na aproximação do modelo analítico.
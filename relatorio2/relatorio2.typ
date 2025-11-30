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
    Projeto 1 - Potenciais e campos
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


== Exercício 13

Considerando o $1/4$ de circunferência $f(x)=sqrt(1-x^2)$ em que $0<=x<=1$, calculando a sua área através do método de Simpsons obtemos $A approx pi/4$
#codly-range(0,end:30)
#figure(
  raw(read("src/cap05_ex13.jl"), lang: "julia", block: true),
  caption: "Aproximação de pi",
)



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


Observamos uma relação linear no crescimento de log(N) e o decréscimo de log10(|Error|), a constante linear é aproximadamente 1.5, existe portanto uma lei de potência relacionando as duas grandezas. 

O problema é que essa relação desaparece para N muito grande pois a precisão limitada dos floats começa a ser relevante

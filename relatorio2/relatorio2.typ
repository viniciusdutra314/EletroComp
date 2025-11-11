#import "@preview/codly:1.3.0": *
#import "@preview/tabut:1.0.2"
#import "@preview/codly-languages:0.1.1": *
#show: codly-init.with()
#codly(languages:codly-languages)
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

  #image("plots/ifsc_logo.jpg", width: 15cm)
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
  14 de outubro de 2025
]
#pagebreak()

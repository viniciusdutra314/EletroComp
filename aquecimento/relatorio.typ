#import "@preview/codly:1.3.0": *
#import "@preview/codly-languages:0.1.1": *
#show: codly-init.with()
#codly(
  languages: (
    rust: (name: "Rust", color: rgb("#CE412B")),
  ),
  display-icon: true
)
#raw(block: true, lang: "rust", read("./src/decay_functions.rs"))

#codly-disable()
afsafsa
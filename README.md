# Feca <img src="https://user-images.githubusercontent.com/1056013/182203532-f220cabf-51ae-4f4a-b902-e7a5a83cf37f.png" style="width: 128px;margin: 0 auto" align=right />

A web browser prototype.

## About Feca

Feca is my part-time project aiming to practice ideas about web page rendering. It is just a prototype and can load and render extremely trival web pages for now.

![](https://user-images.githubusercontent.com/1056013/182268670-d674dcb5-8f31-4991-b21d-4a445339caa4.png)

Feca consists of 2 sub components:

- Felis: a layout and rendering engine
- Catus: a Javascript interpreter

Feca is a combination of the 2 components, with a web runtime for Javascript.

There are 2 extra utility libs:

- xcdt: eXtensible Chained Data Type, [bringing Chaos into Rust](xcdt/README.md) by introducing inheritance and virtual methods.
- crosscom: A lib developed when I wrote [OpenPAL3](https://github.com/dontpanic92/OpenPAL3), providing platform independent COM-like interfaces.

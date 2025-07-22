<div align="center"> <h1> {{ package.name }} {{ package.version }} </h1> </div>

<p align="center">
    <em> 

{{ package.description }}
    </em>
</p>

<div align="right">
    <a href="https://github.com/Magicolo/{{ package.name }}/actions/workflows/test.yml"> <img src="https://github.com/Magicolo/{{ package.name }}/actions/workflows/test.yml/badge.svg"> </a>
    <a href="https://crates.io/crates/{{ package.name }}"> <img src="https://img.shields.io/crates/v/{{ package.name }}.svg"> </a>
</div>

---
### Features
- has `#![no_std]` and `#![forbid(unsafe_code)]`
- supports the applicable core traits
- `features = ["iter"]` *(default)*: supports the `Iterator` family of traits 
- `features = ["serde"]`: supports the `Serialize` and `Deserialize` traits
- `features = ["rayon"]`: supports the `ParallelIterator` family of traits

---
### Cheat Sheet

```rust
{% include "cheat.rs" %}
```
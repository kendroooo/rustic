# 🌲 Rustic

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)](#)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](#)
[![Language](https://img.shields.io/badge/language-Rust-orange.svg)](#)
[![Version](https://img.shields.io/badge/version-0.1.0-yellow.svg)](#)

**Rustic** is a beginner-friendly programming language that compiles directly to **Rust**.  
It combines the *simplicity of Python* with the *safety and performance of Rust*.  
The goal: make systems programming accessible without scaring off newcomers.

---

## ✨ Features

- ✅ **Simple syntax** — intuitive and clean, inspired by Python & Rust.  
- ⚡ **Compiles to Rust** — produces safe, high-performance Rust binaries.  
- 🛡️ **Dynamic ownership model** — memory safety without overwhelming beginners.  
- 🧰 **Standard library essentials** — math, collections, strings, and I/O.  
- 🚀 **Designed for learners** — your first step into the Rust ecosystem.  

---

## 📖 Philosophy

Rust is powerful, but its strict rules can intimidate newcomers.  
Rustic takes away the initial fear while keeping the best parts of Rust —  
ownership, performance, and safety — but with a softer learning curve.

Think of it as a **gentle gateway** into the Rust world.  

---

## 📜 License

Rustic is licensed under the [MIT License](LICENSE).  
You are free to use, modify, and distribute it as long as attribution is provided.

---

## 🧑‍💻 Example

```typescript
// Import built-in modules
import math
import io

// Define a structure
struct Point {
    x: float,
    y: float
}

// Function to calculate distance
fn distance(p1: Point, p2: Point) -> float {
    let dx: float = p1.x - p2.x
    let dy: float = p1.y - p2.y
    return math.sqrt(dx*dx + dy*dy)
}

// Program entry
fn main() -> void {
    let a: Point = Point{x: 0.0, y: 0.0}
    let b: Point = Point{x: 3.0, y: 4.0}
    
    let d: float = distance(a, b)
    io.print("Distance: " + d)
}

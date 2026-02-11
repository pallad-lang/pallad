# Pallad Non-Goals (Consolidated)

This document defines what Pallad **does not aim to provide**, helping maintain clarity, simplicity, and focus.

---

1. **High-Performance System-Level Behavior:**  
   Pallad is not intended to compete with low-level languages like C or C++ in raw performance.

2. **Replacement or Imitation of Other Languages:**  
   Pallad does not aim to replicate Python, Rust, or any other existing language.

3. **Complex Type / Meta-Programming System:**  
   Advanced or overly complex type systems and metaprogramming are not goals.  
   (Basic hybrid typing remains.)

4. **DSL-Focused Design:**  
   Pallad is not a domain-specific language; it targets general-purpose scripting.

5. **Low-Level System Control & Concurrency:**  
   The core will not include threads, coroutines, direct memory manipulation, manual garbage collection, or OS-level system calls; concurrency and system control can be provided via libraries.

6. **Built-in Frameworks / External Dependencies:**  
   The core language will not ship with GUI, web, or other domain-specific frameworks, nor require embedding in another language to function; these are handled by libraries.

7. **Simplicity & Clarity:**  
   Pallad avoids unnecessary syntax sugar, complex code-generation, and reflection mechanisms that reduce clarity.  
   All important behavior must be explicit; nothing happens “magically”.

8. **Optional Static Typing:**  
   Static typing is optional; Pallad does not enforce it everywhere.

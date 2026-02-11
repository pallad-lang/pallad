# Pallad Language Identity

This document defines the core identity of the Pallad programming language.  
It establishes the fundamental characteristics, priorities, and design principles that guide the language's development and evolution.

---

## 1. Core Nature

- **Language Type:** Scripting language with broad applicability (general-purpose scripting)  
- **Primary Use Cases:** Education, safe experimentation/research, tool development  
- **System-Level Orientation:** Low (not system-level)  
- **DSL Focus:** Not DSL-oriented  
- **Execution Model:** Compiled to bytecode and then interpreted  
- **Type System:** Hybrid — optional static typing  
- **Primary Paradigm:** Object-Oriented Programming (OOP)  

---

## 2. Target Audience

- **Beginners / Students:**  
  Simple, predictable syntax and semantics; dynamic typing makes it easy to start learning.  

- **Professional Developers:**  
  Not the primary target; usage may be limited for general professional development.  

- **Language Designers / Researchers:**  
  Supported due to precise core documentation and design records.  

- **Tool Builders / DevOps Engineers:**  
  Well-suited due to simplified tool construction and ease of learning.  

---

## 3. Core Priorities

1. **Simplicity and Learnability:** Clear, predictable, and simple constructs for easy adoption.  
2. **Flexibility and General-Purpose Use:** Useful in a wide range of scenarios without unnecessary complexity.  
3. **Acceptable Performance:** Reasonable execution speed while prioritizing clarity and pure OOP design.  
4. **Clarity:** All features and constructs should behave in a clear, understandable way.  
5. **Safety and Predictability:** Prevent common mistakes; advanced operations require explicit intent.  
6. **Extensibility / Tooling Support:** Support libraries, tools, and extensions without compromising simplicity.  

---

## 4. Design Principles

1. **Explicitness (Advanced Operations Require Acknowledgment):**  
   Programmers must consciously acknowledge advanced or potentially unsafe operations; nothing happens magically.

2. **General-Purpose Flexibility:**  
   Language supports broad use-cases without forcing unnecessary complexity.

3. **First-Class Object-Oriented Paradigm:**  
   OOP is the primary paradigm, fully supported as a first-class concept.

4. **Specialized Flexibility / Power:**  
   Libraries and extensions enable advanced tasks seamlessly while keeping the core simple.

5. **Simplicity:**  
   Core constructs are clear, predictable, and easy to use.

6. **Safety by Default:**  
   Default behavior is safe and predictable; boundary operations require explicit intent.

7. **Learnability:**  
   Easy for beginners to pick up, with intuitive syntax and semantics.

8. **Acceptable Performance:**  
   Performance is reasonable without sacrificing clarity or OOP purity.

9. **Powerful Core with Extensible Libraries:**  
   Core is sufficient for general and introductory specialized use; internal structure supports advanced library development without friction.

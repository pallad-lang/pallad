# Pallad Stability & Versioning Policy

This document defines how Pallad handles versions, breaking changes, and release stability.  
It ensures predictability while allowing language evolution without unnecessary friction.

---

## 1. Versioning Model

Pallad uses a **semantic-like versioning scheme**:

- **Major version:** Introduces potentially breaking changes in the language or core libraries.  
- **Minor version:** Adds new features, improvements, or optimizations in a backward-compatible way.  
- **Patch version:** Bug fixes, hotfixes, and minor adjustments without any feature changes.

> Example: `1.0.0` → first stable release, `1.5.0` → minor feature improvements, `1.5.1` → patch/hotfix.

---

## 2. Major Versions

- Version `1.0` marks the **first stable release**.  
- Breaking changes are only allowed in new **major versions** (e.g., 1.x → 2.0).  
- Breaking changes may only occur if:
  - The previous behavior is **definitely incorrect or unsafe**, or  
  - A fundamental rewrite is required.

- Backward compatibility should generally be maintained within a major version:  
  - Code written for 1.0 should run in all subsequent 1.x releases, unless it relied on behavior that was clearly a mistake.

---

## 3. Minor Versions

- Minor versions (`1.x`) are the **main development channel**.  
- They **must maintain compatibility** with previous minor releases of the same major version.  
- Minor versions may introduce:
  - New features  
  - Improvements or optimizations  
  - Additions to libraries or tooling  

---

## 4. Patch Versions

- Patch versions (`x.y.Z`) are for **bug fixes, hotfixes, and urgent corrections**.  
- They **do not add new features or change behavior**.  
- All patch releases must maintain full backward compatibility with their minor version.

---

## 5. Stability Principles

1. **Clarity over strict backward compatibility:**  
   - Breaking changes can occur only if leaving them would continue a clearly incorrect or unsafe behavior.  
   - Otherwise, backward compatibility is maintained even if it slightly limits new features.

2. **Predictability:**  
   - Developers should know exactly what to expect when upgrading to a new patch or minor version.

3. **Developer-Friendly Evolution:**  
   - Policy aims to **prevent accidental breakages** while allowing the language to evolve.  
   - Avoid changes that force major rewrites unless absolutely necessary.

4. **Safety:**  
   - Any change that may break code should be **safe by default** and clearly documented in release notes.

# Cathedral Arkhe — Lean 4 Formalization

**Reference:** CATHEDRAL-ARKHE-WHITEPAPER-v1.1, Appendix C
**Status:** 1 PROOF CLOSED, 16 BLOCKED (CI gate §55.1 — proof holes)
**Last Updated:** 2026-07-29

---

## Epistemic Status (Read This First)

This is a **speculative research programme**, not an established theory.

- None of the 17 theorems is proved (except T1.1, which is definitional)
- The central operator `C` is **undefined** (§11.1)
- The heartbeat frequency `ω_hb` has **no independent physical identification** (§11.4)
- One concept (photon as Nambu-Goldstone mode) **appears to violate** the standard Goldstone theorem (§9.4)

See whitepaper Part X for the honest self-assessment.

---

## Project Structure

```
CathedralArkhe/
├── Preliminaries.lean      # Core type definitions (MobiusBand, CasimirOperator)
├── T1/                     # Tier 1: Manifold Structure
│   ├── Mobius.lean         #   T1.1 — PROOF CLOSED (definitional)
│   ├── NonOrientable.lean  #   T1.2 — BLOCKED
│   ├── Connected.lean      #   T1.3 — BLOCKED
│   ├── FundamentalGroup.lean # T1.4 — BLOCKED
│   └── DoubleCover.lean    #   T1.5 — BLOCKED
├── T2/                     # Tier 2: Metric and Connection
│   ├── InducedMetric.lean  #   T2.1 — BLOCKED
│   ├── LeviCivita.lean     #   T2.2 — BLOCKED
│   ├── Weitzenbock.lean    #   T2.3 — BLOCKED
│   ├── Contortion.lean     #   T2.4 — BLOCKED
│   └── AbelianLimit.lean   #   T2.5 — BLOCKED
├── T3/                     # Tier 3: Wave Equation and Boundary
│   ├── WaveOperator.lean   #   T3.1 — BLOCKED
│   ├── Antiperiodic.lean   #   T3.2 — BLOCKED
│   ├── Spectrum.lean       #   T3.3 — BLOCKED
│   └── BerryPhase.lean     #   T3.4 — BLOCKED
├── T4/                     # Tier 4: Controlled Limits
│   ├── WidthZeroLimit.lean #   T4.1 — BLOCKED
│   ├── TorsionZeroMaxwell.lean # T4.2 — BLOCKED
│   └── HyperbolicCone.lean #   T4.3 — BLOCKED
└── T5/                     # Tier 5: Casimir Constraints
    ├── Kantorovich.lean    #   T5.1 — BLOCKED
    ├── ProductFixed.lean   #   T5.2 — BLOCKED (tautology)
    └── FiniteSpeed.lean    #   T5.3 — BLOCKED (contains orphan axiom)
```

---

## Theorem Inventory

| ID | Statement | Tier | LOC Est. | Status | Blocking Reason |
|:---|:---------|:-----|:--------:|:------:|:----------------|
| T1.1 | Möbius quotient well-defined | 1 | 200 | **CLOSED** | Definitional |
| T1.2 | No global orientation | 1 | 150 | BLOCKED | Needs manifold structure |
| T1.3 | Path-connectedness | 1 | 50 | BLOCKED | Needs quotient topology |
| T1.4 | π₁(M) ≅ ℤ | 1 | 400 | BLOCKED | Needs T1.3 + deformation retract |
| T1.5 | Double cover exists | 1 | 300 | BLOCKED | Needs T1.4 |
| T2.1 | Induced metric | 2 | 250 | BLOCKED | Needs T1.1 manifold extension |
| T2.2 | Levi-Civita connection | 2 | 200 | BLOCKED | Needs T2.1 |
| T2.3 | Weitzenböck torsion ≠ 0 | 2 | 350 | BLOCKED | Needs T2.2 |
| T2.4 | Contortion tensor | 2 | 150 | BLOCKED | Needs T2.3 |
| T2.5 | T_μν → F_μν abelian limit | 2 | 500 | BLOCKED | Needs T2.3, T2.4 |
| T3.1 | Wave operator well-posed | 3 | 250 | BLOCKED | Needs T2.1 |
| T3.2 | Antiperiodicity from single-valuedness | 3 | 200 | BLOCKED | Needs T1.1, T3.1 |
| T3.3 | Half-integer spectrum | 3 | 400 | BLOCKED | Needs T3.2 |
| T3.4 | Berry phase = π | 3 | 300 | BLOCKED | Needs T1.4, T3.1 |
| T4.1 | w→0 recovers antiperiodic circle | 4 | 250 | BLOCKED | Needs T1.1, T3.2 |
| T4.2 | κ→0 recovers Maxwell | 4 | 350 | BLOCKED | Needs T2.5 |
| T4.3 | Light-cone structure | 4 | 300 | BLOCKED | Needs T3.1, T4.1 |
| T5.1 | Kantorovich inequality | 5 | 100 | BLOCKED | Mathlib lookup |
| T5.2 | ε₀μ₀ω²R² = 1 (tautology) | 5 | 150 | BLOCKED | By definition |
| T5.3 | c < ∞ ⟺ C ≠ λI | 5 | 300 | BLOCKED | **Contains orphan axiom** |

**Total estimated LOC:** ~4,700 (matches whitepaper §14.2)

---

## Orphan Axioms

This repository contains one explicit orphan axiom:

| Name | Location | Statement | Risk |
|:-----|:---------|:----------|:-----|
| `finite_speed_bridging` | T5/FiniteSpeed.lean | Non-degenerate spectrum ⇒ finite c | **HIGH** |

This axiom is required for T5.3's reverse direction. It cannot be proved in L1
because it bridges mathematics to physics. See whitepaper §17.2–§17.3.

---

## Dependencies

**Single dependency:** [Mathlib4](https://github.com/leanprover-community/mathlib4)

No external physics libraries are used. As of July 2026, no Lean 4 formalization
of superconductivity (BCS), teleparallel gravity, or quantum field theory exists
in public repositories (whitepaper §49.4, §14.1). All 17 theorems are
formalizable using Mathlib's topology, manifold, and analysis libraries alone.

---

## Build Instructions

```bash
# Clone
git clone <repo-url> && cd cathedral-arkhe-lean

# Initialize Mathlib (takes 30-60 minutes first time)
lake update

# Build (will succeed — sorry is syntactically valid)
lake build

# Build specific tier
lake build CathedralArkhe.T1

# Check for proof holes (what CI does)
grep -rn "sorry" CathedralArkhe/ --include="*.lean" | grep -v "^.*:.*--.*sorry"
```

---

## CI Gates (§55.1)

| Gate | Blocks merge on | Whitepaper Reference |
|:-----|:----------------|:---------------------|
| `lake build` | Any failure | §55.1 — baseline compilation |
| No `sorry` | Any occurrence | §2.1 — claims require refutation mechanisms |
| Axioms in designated files | Undesignated axiom | §17.2 — orphan axiom visibility |
| Registry ↔ filesystem | Divergence | §48.3 — theorem-file coherence |

---

## Contributing

1. **No sorry.** If you cannot prove it, do not commit it.
2. **No hidden axioms.** State assumptions explicitly as `axiom` in `*Axiom*.lean`.
3. **No physical content in L1 theorems.** If your statement references
   experiment, measurement, or physical interpretation, it belongs in L3,
   not here.
4. **No interpretive vocabulary in theorem statements.** Terms like
   "recognition," "cogito," "cathedral" are commentary, not mathematics.
   They go in doc comments, not theorem names.

---

## License

This formalization is released for open review. Code artefacts carry their
own licenses as specified in the repository.
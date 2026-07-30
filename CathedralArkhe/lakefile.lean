import Lake
open Lake DSL

/-!
  Cathedral Arkhe — Lean 4 Formalization
  Reference: CATHEDRAL-ARKHE-WHITEPAPER-v1.1, Appendix C

  Dependencies:
    - Mathlib4: the only verified formal mathematics library required.
      No external physics libraries exist in Lean 4 as of July 2026
      (whitepaper §49.4, §14.1).

  CI gate §55.1: `lake build` must pass with zero `sorry`.
  All 17 theorem targets are currently BLOCKED by proof holes.
-/

package cathedralArkhe where
  leanOptions := #[⟨`pp.unicode.fun, true⟩]

@[default_target]
lean_lib CathedralArkhe where
  -- All theorems are sorry-blocked; CI gate §55.1 enforces proof completion
  -- before any target can be marked PROOF CLOSED

require mathlib from git
  "https://github.com/leanprover-community/mathlib4.git" @ "master"

meta if get_config? env = some "dev" then
-- dev overrides for local testing
end
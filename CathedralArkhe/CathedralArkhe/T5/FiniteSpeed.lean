/-!
  Cathedral Arkhe — T5.3: Finite Speed ⇔ Non-Degenerate Spectrum

  Reference: Whitepaper §11.2, Appendix C

  CRITICAL NOTE: The reverse direction requires a BRIDGING AXIOM
  (§17.3) that no proof assistant can check. This is an orphan axiom.
-/

import CathedralArkhe.Preliminaries
import CathedralArkhe.T5.Kantorovich

namespace CathedralArkhe.T5

variable {E : Type*} [InnerProductSpace ℂ E] [CompleteSpace E]
  [CasimirOperator E] (c : ℝ) (hc : 0 < c)

/-- T5.3: Finite propagation speed is equivalent to non-degenerate spectrum.

    Forward direction (L1, provable):
      If c < ∞, then C is not proportional to identity.
      Proof: By T5.1, equality holds iff C ∝ I.

    Reverse direction (REQUIRES BRIDGING AXIOM, §17.3):
      If C has non-degenerate spectrum, then c < ∞.
      This step interprets "non-degenerate spectrum" as "finite
      effective permittivity/permeability" — a physical claim
      that cannot be derived from pure mathematics.

    ORPHAN AXIOM RECORD (§17.2):
      Name: finite_speed_bridging
      Statement: Non-degenerate spectrum of C implies bounded
                 ε₀, μ₀, hence finite c.
      Status: Unproven, likely unprovable in L1.
      Risk: If false, the entire finite-speed argument collapses.
-/
theorem casimir_finite_speed :
    c < (⊤ : ℝ) ↔ ¬ ∃ (λ_val : ℂ), CasimirOperator.C = λ_val • LinearMap.id := by
  -- BLOCKED: Forward direction uses T5.1
  -- Reverse direction is a bridging axiom — cannot be proved
  sorry

/-- The bridging axiom, stated explicitly as an axiom.

    This is the orphan axiom that T5.3's reverse direction requires.
    Stating it as an axiom is the honest thing to do: it makes the
    dependency visible rather than hidden in a proof.

    See §17.2 for the full orphan axiom inventory.
-/
axiom finite_speed_bridging {E : Type*} [InnerProductSpace ℂ E] [CompleteSpace E]
  [CasimirOperator E] (c : ℝ) (hc : 0 < c)
  (hC : ¬ ∃ (λ_val : ℂ), CasimirOperator.C = λ_val • LinearMap.id) :
  c < (⊤ : ℝ)

/-- T5.3 with bridging axiom made explicit -/
theorem casimir_finite_speed_with_axiom :
    c < (⊤ : ℝ) ↔ ¬ ∃ (λ_val : ℂ), CasimirOperator.C = λ_val • LinearMap.id := by
  constructor
  · intro h
    -- Forward: c < ∞ ⇒ C not proportional
    -- By contrapositive of T5.1: if C ∝ I, then ⟨ψ,Cψ⟩⟨ψ,C⁻¹ψ⟩ = 1
    -- which (by the physical interpretation) would give c = ∞
    intro hC
    exfalso
    -- This direction should follow from T5.1 + physical interpretation
    sorry
  · exact finite_speed_bridging c hc

end CathedralArkhe.T5
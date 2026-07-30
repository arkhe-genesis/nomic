import Mathlib

set_option linter.style.header false

namespace CathedralArkhe.T1

variable (L : ℝ) (w : ℝ) (hw : w > 0)

abbrev Rect : Type :=
  { p : ℝ × ℝ // p.fst ∈ Set.Icc 0 L ∧ p.snd ∈ Set.Icc (-(w / 2)) (w / 2) }

instance : TopologicalSpace (Rect L w) := instTopologicalSpaceSubtype

def MobiusRel (p q : Rect L w) : Prop :=
  p.val = q.val ∨
  (p.val.fst = 0 ∧ q.val.fst = L ∧ p.val.snd = -q.val.snd) ∨
  (q.val.fst = 0 ∧ p.val.fst = L ∧ q.val.snd = -p.val.snd)

def mobiusSetoid (hL : L > 0) : Setoid (Rect L w) where
  r := MobiusRel L w
  iseqv := by
    refine ⟨?refl, ?symm, ?trans⟩
    · intro x
      exact Or.inl rfl
    · rintro x y (h_eq | ⟨a, b, c⟩ | ⟨a, b, c⟩)
      · exact Or.inl h_eq.symm
      · exact Or.inr (Or.inr ⟨a, b, c⟩)
      · exact Or.inr (Or.inl ⟨a, b, c⟩)
    · rintro x y z hxy hyz
      rcases hxy with hxy | hxy | hxy
      · rcases hyz with hyz | hyz | hyz
        · exact Or.inl (hxy.trans hyz)
        · exact Or.inr (Or.inl (hxy.symm ▸ hyz))
        · exact Or.inr (Or.inr (hxy.symm ▸ hyz))
      · rcases hyz with hyz | hyz | hyz
        · exact Or.inr (Or.inl (hyz ▸ hxy))
        · exfalso
          have h1 : y.val.fst = L := hxy.2.1
          have h2 : y.val.fst = 0 := hyz.1
          linarith
        · have h1 : x.val.1 = z.val.1 := by linarith [hxy.1, hyz.1]
          have h2 : x.val.2 = z.val.2 := by linarith [hxy.2.2, hyz.2.2]
          have h_eq : x.val = z.val := Prod.ext h1 h2
          exact Or.inl h_eq
      · rcases hyz with hyz | hyz | hyz
        · exact Or.inr (Or.inr (hyz ▸ hxy))
        · have h1 : x.val.1 = z.val.1 := by linarith [hxy.2.1, hyz.2.1]
          have h2 : x.val.2 = z.val.2 := by linarith [hxy.2.2, hyz.2.2]
          have h_eq : x.val = z.val := Prod.ext h1 h2
          exact Or.inl h_eq
        · exfalso
          have h1 : y.val.fst = 0 := hxy.1
          have h2 : y.val.fst = L := hyz.2.1
          linarith

def MobiusBand (hL : L > 0) : Type := Quotient (mobiusSetoid L w hL)

instance (hL : L > 0) : TopologicalSpace (MobiusBand L w hL) :=
  instTopologicalSpaceQuotient

end CathedralArkhe.T1

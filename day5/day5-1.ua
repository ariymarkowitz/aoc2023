# put each block of numbers into a list
Parse ← ⊜(□⊜parse¬∊:" \n". ⊔⊢⇌⊜□≠@:.)¬⌕"\n\n". &fras

Process ← (
  ⊃(↘1)(⊔⊢) # split maps and seeds
  ≡⍜⊔(↯¯1_3) # group into rows of 3
)

NextValue ← (
  ≡(↧⊃(≤/+⊏1_2)(≥⊏1)),,⊔ # ranges containing value
  (+/-⊏1_0⊏⊢|⋅⋅∘) =0⧻.⊚ # new value, or identity if not in any range
)

PartI ← /↧≡∧NextValue ¤ Process Input

PartI "input.txt"
Input ← &fras "input.txt"

Lines ← ⊜□≠@\n.
Parse ← ∩(⊜parse≠@ .⊔) ⍘⊟ ↘1⊢regex ".*:(.*)\\|(.*)"

PartI ← /+ ≡(⌊ⁿ:2-1/+∊ Parse) Lines

PartII ← (
  ≡(/+∊ Parse) Lines # the number of wins of each card type
  ⊃(∘|⧻|⇡⧻)
  ≡(↻¯+1:⬚0↙:↯⊙1)       # the cards gained from individual cards of each type
  ∧(⍜⊡×⊃⊙∘(+1/+⊏⊙⍉))⇡⧻. # get the total cards gained from each type
  /++1/+                # total number of cards
)

PartI Input
PartII Input

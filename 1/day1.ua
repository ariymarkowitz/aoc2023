Split! ← ⊜^1≠⊙.

Words ← Split!□@ $ zero one two three four five six seven eight nine
Digits ← "0123456789"

Result ← (
  ↥∩⊐≡⌕ ⊃(Digits¤|Words¤) # get matches for digits/words
  ∩(⊢⍖)⊃∘≡⇌               # get last and first matches
  +×10                    # combine
)

/+ Split!Result @\n &fras "input.txt"

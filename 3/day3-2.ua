RawInput ← &fras "input.txt"

IsDigit ← ∊:"0123456789"
Pad ← ⍥(⍉≡(↻1⊂⊂@.@.))2
Input ← Pad ⊜∘≠@\n. RawInput

⊜parse ∵IsDigit.♭ Input     # list of numbers in the schematic
⊜□ ⊃(♭∵IsDigit|☇1⇡△) Input  # indices for each number
≡(≡+ ☇1-1⇡3_3 ¤) ⊚=@* Input # indices adjacent to each asterisk
⊠(/↥∊⊙⊔)                    # boolean mask of numbers adjacent to each asterisk
▽≡(=2⧻⊚).                   # keep asterisks adjacent to each mine
/+ ≡(/×▽)⊙¤                 # get gear ratios and sum
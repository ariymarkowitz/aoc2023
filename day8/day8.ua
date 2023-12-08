&fras "input.txt"

⊜□¬⌕"\n\n".
=@R°□°⊟ # L -> 0, R -> 1
Inst ←
⊜(⊃(⊏|⊏+7|⊏+12)⇡3)≠@\n.°□ # split columns
Names ←
≡(⊗:) ¤Names ⊟ # convert nodes to indices
Paths ←
Starts ← ⊚=@A⊢⇌⍉ Names

Next ← ⊏⊙(⊏⊏⊃(◿⧻Inst∘|Inst Paths)) # node, instruction
Done ← =@Z⊢⇌⊏:Names

;≡⍢(⊃Next⋅(+1)|¬Done) Starts 0
# then find LCM with some other tool
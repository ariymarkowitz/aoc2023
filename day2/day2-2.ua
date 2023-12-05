Colors ← {"red" "green" "blue"}
Max ← [12 13 14]

FindInt ← parse⊔⊢regex"\\d+"
InitSplit ← ⊜□↧ ∩≠ ⊃(@:∘|@;∘).
ID ← FindInt ⊔⊢
ProcessColor ← ×⊐≡(/↥⌕)Colors ¤ ⊃∘FindInt
ProcessColorList ← /+⊜(ProcessColor)≠@,.

ColorResults ← ≡(ProcessColorList⊔) ↘1
GameState ← ⊃ColorResults ID InitSplit
IsValidState ← ≡≤¤[12 13 14]
MinimumState ← /↥

SingleSolutionI ← (⋅0|∘) /↧♭IsValidState GameState
SolutionI ← /+ ⊜SingleSolutionI≠@\n.

SingleSolutionII ← /×MinimumState ⊙; GameState
SolutionII ← /+⊜SingleSolutionII≠@\n.

SolutionII &fras "input.txt"
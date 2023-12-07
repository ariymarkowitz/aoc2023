# list of hands and bids, with hands converted to numbers
Parse ← |1.2 ⊗:"J23456789TQKA" ⊓(≡°□|≡⋕) °⊟⍉⊜(⊜□≠@ .)≠@\n. &fras

# determine the type based on the counts of cards
HandType ← ⊢⇌⊚ [⊃(⋅1|/↥=2|=3⧻|/↥=3|=2⧻|/↥=4|=1⧻)] °⊚⊛

Replace ← |2 ⍜▽≡⋅∘=0.: # card, hand
BestType ← /↥≡(HandType Replace)⇡13 ¤

Parse "input.txt"
≡⊃(⊂BestType.|⊂HandType.) # scores as a list of rank and hands
∩(/+×+1⍏⍏)⊙,              # get ranks and total winnings

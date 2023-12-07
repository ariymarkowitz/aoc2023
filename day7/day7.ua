# list of hands and bids
Parse ← |1.2 ⊓(≡°□|≡⋕) °⊟⍉⊜(⊜□≠@ .)≠@\n. &fras

# convert hands to numbers
ToNumI ← ⊗:"23456789TJQKA"
ToNumII ← ⊗:"J23456789TQKA"

# determine the type based on the counts of cards
HandType ← ⊢⇌⊚ [⊃(⋅1|/↥=2|=3⧻|/↥=3|=2⧻|/↥=4|=1⧻)] °⊚⊛

Replace ← ⍜▽≡⋅∘=0.: # card, hand
BestType ← /↥≡(HandType Replace)⇡13 ¤

# scores as a list of rank and hands
≡⊃(⊂BestType.ToNumII|⊂HandType.ToNumI) Parse "input.txt"
∩(/+×+1⍏⍏)⊙, # get ranks and total winnings


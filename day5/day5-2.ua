# put each block of numbers into a list
Parse ← ⊜(□⊜parse¬∊:" \n". ⊔⊢⇌⊜□≠@:.)¬⌕"\n\n". &fras

NoRanges ← ↯0_2 0

# make start/end/displacement map and fill in gaps
ProcessMap ← (
  ≡⍜⍘[⊙⊙∘]⊃(⋅∘|⋅+|-:) ↯¯1_3 # make rows [start, end, displacement]
  ⊏⍏.
  ⊂[¯∞ ⊡0_0 ⊙0].   # add range (-infinity, min)
  ⊂:[⊡¯1_1 ⊙∞ ⊙0]. # add range (max, infinity)
)
ProcessSeeds ← ≡\+ ↯¯1_2 # [start, end] ranges of seeds
Process ← ⊃(≡⍜⊔ProcessMap↘1|ProcessSeeds⊔⊢)

Ends ← ⊃∩(⊢⇌)∩⊢                                     # end1 end2 start1 start2
Intersects ← ↧⊓≥<Ends⇌                              # do the ranges intersect 
Intersection ← (NoRanges|[:⊓↧↥] Ends) Intersects ,, # intersection of ranges

# the image of a range (as a list of ranges) under a map
MapRange ← ∧(⊂ +⊃(⊢⇌|Intersection ↙2)) ⊙¤ ⊙⊙NoRanges
MapRanges ← ⊏⍏. ∧(⊂MapRange) ¤ ⊙⊙NoRanges # map a list of ranges

∧(MapRanges ⊔) Process Parse "input.txt" # get all of the location ranges
/↧⊢⍉                                     # minimum start value of a range

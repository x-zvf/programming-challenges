#!/usr/bin/env tclsh

while {[gets stdin line] >= 0} {
  if {$line eq ""} continue
  if {[scan $line {%[^(] (contains %[^)])} left right] != 2} continue
  # Treat left as a list (dangerous with real input, safe here).
  # right needs to be split correctly.
  set left [list {*}$left]
  set right [split [string map {{ } {}} $right] ,]
  lappend all {*}$left
  foreach i $right {
    lappend sources($i) $left
  }
}

# This whole problem is poorly specified.  Allergens aren't always
# marked, so the number of ingredients that "can't possibly contain an
# allergen" is zero.

# Near as I can tell from the example problem's answer, the steps are:
# 1) Identify which ingredient is the source of each LISTED allergen.
# 2) Iterate over the full list of ingredients and count the number of
#    them that are not identified as an allergen source, without removing
#    duplicates.

proc ldelete {list item} {
  set tmp [list]
  foreach i $list {
    if {$i ne $item} {lappend tmp $i}
  }
  return $tmp
}

set unknowns [array names sources]
while {[llength $unknowns]} {
  # Look for an allergen with only one possible ingredient.
  # puts "unknowns: $unknowns"
  foreach a $unknowns {
    # puts "consider $a"
    if {[llength $sources($a)] == 1} {
      set list {}
      foreach i [lindex $sources($a) 0] {
        if {! [info exists contains($i)]} {lappend list $i}
      }
      # puts " one food; possible sources: $list"
      if {[llength $list] == 1} {
        set contains($i) $a
	# puts " $i contains $a"
	set unknowns [ldelete $unknowns $a]
      }
      continue
    }

    # puts " [llength $sources($a)] foods"
    # Find the ingredients that appear in ALL foods known to be a source
    # of this allergen.  Do this by first counting the number of times
    # each ingredient appears across all such foods.
    array unset count
    foreach list $sources($a) {
      foreach i $list {
        if {! [info exists contains($i)]} {incr count($i)}
      }
    }
    set list {}
    foreach {i n} [array get count] {
      if {$n == [llength $sources($a)]} {lappend list $i}
    }
    # puts " possible sources: $list"
    if {[llength $list] == 1} {
      set i [lindex $list 0]
      set contains($i) $a
      # puts " $i contains $a"
      set unknowns [ldelete $unknowns $a]
    }
  }
}

set n 0
foreach i $all {
  if {! [info exists contains($i)]} {incr n}
}
puts $n

set list {}
foreach {i a} [lsort -stride 2 -index 1 [array get contains]] {
  lappend list $i
}
puts [join $list ,]

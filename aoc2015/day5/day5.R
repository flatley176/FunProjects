rm(list=ls())
vowelstring <- "aeiou"
#string <- "abcdefgh"


#####################
###### part 1 #######
#####################

bad_combos_present <- function(s) {
    bad_combos <- c("ab", "cd", "pq", "xy")
    sum(sapply(bad_combos, function(b) {
        length(grep(b, s))
    }))    
}

get_substrings <- function(s, n) {
    if (nchar(s) >= n) {
        strsplit(sapply(seq(1, nchar(s)-n+1), function(x) {
            substr(s, x, x+n-1)
        }), "")
    }
}

get_unique_doublet_count <- function(l) {
    sum(sapply(l, function(x) {
        length(unique(x))
    }))/2
}

vowel_count <- function(s) {
    sum(sapply(unlist(strsplit(s, "")), function(x) {
        length(grep(x, vowelstring)) 
    }))
}


is_nice_one <- function(s) {
   ifelse(bad_combos_present(s)==0 & get_unique_doublet_count(get_substrings(s, 2)) < nchar(s)-1 & vowel_count(s) >= 3, 1, 0) 
}

part_one_is_nice <- function(input) {
    input <- as.character(read.table(input, header=F)$V1)
    sum(sapply(input, function(i) {
        is_nice_one(i)
    }))
} 

part_one_is_nice("input")


#####################
###### part 2 #######
#####################

has_overlapping_doublet <- function(l) {
    sum(sapply(seq(1, length(l)-1), function(i) {
        ifelse(all(l[[i]]==l[[i+1]]), 1, 0)
    }))
}

nice_alternating_count <- function(s) {
    subs <- get_substrings(s, 3)
    sum(sapply(subs, function(x) {
        ifelse(x[1]==x[3], 1, 0)
    }))
}

is_nice_two <- function(s) {
    doublets <- get_substrings(s, 2)
    ifelse(nice_alternating_count(s)>=1 & has_overlapping_doublet(doublets)==0 & length(doublets)>length(unique(doublets)), 1, 0)
}

## 50 isn't right, so I might be reading something wrong here
## maybe I shouldn't invalidate a string if it contains an "aaa" - as long as it has two or more non-overlapping "aa"s

s <- "aabcdaaabgggggaa"
twice_but_not_overlapping <- function(s) {
    doublets <- get_substrings(s, 2)
    indexes <- sapply(seq(1, length(doublets)-1), function(i) {
        ifelse(all(doublets[[i]]==doublets[[i+1]]), i, 0)
    })
    indexes <- indexes[indexes!=0]
    for (i in seq(2, length(indexes))) {
        if(indexes[i]-indexes[i-1]==1) {
            indexes <- indexes[-i]
        }
    }
}

part_two_is_nice <- function(input) {
    input_list <- as.character(read.table(input, header=F)$V1)
    sum(sapply(input_list, function(i) {
#        is_nice_two(i)
    }))
}

part_two_is_nice("input")




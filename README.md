# fallout-terminal-hacker
This is a quick and dirty program to assist in terminal hacking in fallout games.  I've used it successfully in Fallout 3, Fallout New Vegas and Fallout 4.  It should work anywhere the same game mechanic is used for terminal hacking.

* enter words, one per line
* empty line when done
* script will suggest optimal guess
* provide feedback from the game (number of characters matched)
* word list is filtered based on the feedback
* rinse/repeat

the algorithm picks a candidate word such that if wrong it will eliminate at many other words from the list as possible.

## sample run, using words I pulled out of /usr/share/dict/words

```
egrep "^[a-z][aei][a-z][a-z][aeiou][a-z]$" /usr/share/dict/words |less
```

the word is "denial"

```
$ cargo run
   Compiling fallout-terminal-hacker v0.1.0 (/home/bracher/git/fallout-terminal-hacker)
    Finished dev [unoptimized + debuginfo] target(s) in 0.28s
     Running `target/debug/fallout-terminal-hacker`
Enter words, one per-line, empty line when done.
aerial
babied
bagged
backer
ballad
bemoan
calves
canyon
citrus
decked
denial
dimmer
faucet
fasten
ferret
gambit
garden
genial
healer
heaven
harden
jagged
killer

bagged is optimal
What was the reported count?
0
{"citrus", "aerial", "denial", "genial"}
genial is optimal
What was the reported count?
5
{"denial"}
$
```

`denial` and `genial` actually have an equal chance of being the second guess,
but it worked out this time for the example.

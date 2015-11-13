# fallout-terminal-hacker
quick and dirty perl script to assist in terminal hacking in fallout games.  I've used it successfully in fallout 3, fallout new vegas and fallout 4.  should work anywhere the same game mechanic is used for terminal hacking.

* enter words, one per line
* empty line when done
* script will make a suggestion, prefixed by '+'
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
$ ./fallout-terminal-hacker
enter words, one per line, empty word when done:
> aerial
> babied
> bagged
> backer
> ballad
> bemoan
> calves
> canyon
> citrus
> decked
> denial
> dimmer
> faucet
> fasten
> ferret
> gambit
> garden
> genial
> healer
> heaven
> harden
> jagged
> killer
>
. aerial
. babied
+ bagged
. backer
. ballad
. bemoan
. calves
. canyon
. citrus
. decked
. denial
. dimmer
. faucet
. fasten
. ferret
. gambit
. garden
. genial
. healer
. heaven
. harden
. jagged
. killer
which word was guessed?
> bagged
what was the count?
> 0
. aerial
. citrus
. denial
+ genial
which word was guessed?
> genial
what was the count?
> 5
+ denial
$
```

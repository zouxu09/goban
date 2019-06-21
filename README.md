# Goban

**Library to play with a rusty goban** 

**Use the version >4.3 because in a bug detecting dead stones**

*Channel : stable*


Only contains move generation, and rules there is no IA, neither
front-end.

Exemple :

```{rust}
let mut g = Game::new(GobanSizes::Nine, Rule::Chinese);
        let mut i = 35;
        while !g.legals().count() != 0 && i != 0 {
            g.play(
                &g.legals().map(|coord| Move::Play(coord.0, coord.1))
                    .choose(&mut rand::thread_rng())
                    .unwrap());
            i -= 1;
            println!("{}", g.goban().pretty_string());
        }
```

```{bash}
.........
.........
.........
.........
.........
.........
.........
⚪........
.........


etc...
```


## What works
- Capturing stones
- Playing
- Passing
- Resigning
- Implementation to count points
- Printing an *ugly* ascii goban
- Generate legals moves
- Japanese Rules
- Chinese Rules

## What is not in point:
- Handling dead stones at the end of the game.

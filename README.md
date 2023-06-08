# Dice roller CLI

In-terminal dice roller. Can roll any set of any dices.

## How to use?

There is no options, so just run it and start typing your rolls.

### Format

1. Dice roll can be: `2d10`, `d20`, `10`. 
1. You can sum dice rolls: `2d10 + d20 + 10`.
1. You can add advantage/disadvantage to dice roll: `g2d10 + bd20`(good and bad rolls). Good/bad roll will add one more dice to the throw and remove min/max dice from the throw.

### Output

```
>> <your query>
<your query, normalized> => <all dices shown> => <sum>
```

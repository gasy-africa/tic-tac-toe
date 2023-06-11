# :abacus: Tic Tac Toe

---
**NOTE**

This project is a study of the [Minimax](https://en.wikipedia.org/wiki/Minimax) Algorithm, written in [Rust](https://www.rust-lang.org/), through game boards like the [Tic Tac Toe](https://en.wikipedia.org/wiki/Tic-tac-toe) or the [Fanorona](https://en.wikipedia.org/wiki/Fanorona). To facilitate the study, the project will generate and compare different source codes using a [Generative AI](https://en.wikipedia.org/wiki/Generative_artificial_intelligence) tools like [ChatGPT](https://chat.openai.com/)

The aproach is to :
1. generate a `Minimax Algorithm template`, 
2. generate several Tic-Tac-Toe :scroll: source code `examples` used as :newspaper_roll: source code `templates`
3. gather all the `Responses`
4. mix all the `reponses` by refactoringg the source code based on the first `Minimax Algorithm template`

**Summary**

Generate multiple source codes using Generative AI and refactor to the best source code based on the state of the art `Refactorization` rules.

---



| :hash:                         | Acivities                               |
|--------------------------------|:----------------------------------------|
| [:a:](#a-algorithm-template)   | Algorithm Template :heavy_check_mark:   |
| :b:                            | Tic-Tac-Toe Source Code Generation      |
| [:one:st](#onest-response)     | TicTactoe :heavy_check_mark:            |
| [:two:nd](#twond-response)     | TicTactoe :heavy_check_mark:            |
| [:three:rd](#threerd-response) | TicTactoe :heavy_check_mark:            |
| [:four:th](#fourth-response)   | TicTactoe :heavy_check_mark:            |
| [:ab:](#ab-mixing-solution)    | Mixing :heavy_check_mark:               |
| :cl:                           | Algorithm Enhancement                   |
| [:five:th](#fiveth-response)   | Alpha-Beta :heavy_check_mark:           |

## :a: Algorithm Template 


```
generate minimax algorithm in rust
```
> Certainly! Here's an example implementation of the Minimax algorithm in Rust:

[:point_right: Click to view the generated source code :scroll: ](.docs/a/CODE.md)

Please note that this is a basic framework for the Minimax algorithm, and you'll need to replace the placeholders (...) with your actual game state type and implement the missing parts such as the evaluation function, generating possible moves, and checking for terminal game states.

Additionally, this implementation assumes a two-player, alternating-turn game. If you're working with a different game setup, you might need to modify the algorithm accordingly.

- [ ] Execute

```
cargo run
```
> Returns

[:point_right: Click to view the output when executing the source code :chart: ](.docs/a/OUTPUT.md)

## :b: TicTacToe

:bulb: First question asked to the Generative AI

> generate tic tac toe game using minimax algorithm in rust

### :one:st Response

Sure! Here's an example implementation of Tic-Tac-Toe game using the Minimax algorithm in Rust:

[:point_right: Click to view the generated source code :scroll: ](.docs/1/CODE.md)

Please note that this is a basic implementation of the Tic Tac Toe game using the Minimax algorithm. There may be further improvements and optimizations that can be made.

- [ ] Execute

```
cargo run
```
> Returns

[:point_right: Click to view the output when executing the source code :chart: ](.docs/1/OUTPUT.md)

### :two:nd Response

:recycle: `regenerate response`

[:point_right: Click to view the generated source code :scroll: ](.docs/2/CODE.md)

This implementation demonstrates the Minimax algorithm for the game of Tic Tac Toe. The AI player (O) uses the `find_best_move` function to determine the best move it should make given the current state of the board.

- [ ] Execute

```
cargo run
```
> Returns

[:point_right: Click to view the output when executing the source code :chart: ](.docs/2/OUTPUT.md)

### :three:rd Response

:recycle: `regenerate response`

[:point_right: Click to view the generated source code :scroll: ](.docs/3/CODE.md)

This code represents a simple command-line version of Tic-Tac-Toe where the player ('X') plays against the computer ('O') using the Minimax algorithm. The game prompts the player for their moves and displays the board after each move. The game continues until there is a winner or a draw.

Please note that this implementation does not include any advanced techniques like alpha-beta pruning, and it assumes that the player always enters valid moves. Feel free to enhance the code further according to your requirements.

- [ ] Execute

```
cargo run
```
> Returns :

[:point_right: Click to view the output when executing the source code :chart: ](.docs/3/OUTPUT.md)


### :four:th Response

:recycle: `regenerate response`

[:point_right: Click to view the generated source code :scroll: ](.docs/4/CODE.md)

This implementation allows the human player to play against the computer player using the Minimax algorithm to determine the computer's moves. The game loop continues until there is a winner or a draw, and the final result is displayed at the end. The human player is represented by 'X', and the computer player is represented by 'O'. The human player can enter their move as a row and column coordinate, such as "0,2" for the top-right cell.

Please note that this implementation assumes a standard 3x3 Tic Tac Toe board. If you want to modify the board size, you'll need to adjust the Board type and update the relevant parts of the code accordingly.

- [ ] Execute

```
cargo run
```
> Returns :

[:point_right: Click to view the output when executing the source code :chart: ](.docs/4/OUTPUT.md)

## :ab: Mixing Solution

---
**NOTE**

The `mixing solution` is to take the `2nd` solution and add the `3rd` solution :keyboard: `input`, :desktop_computer: `output` and :loop: `loop` 

---


[:point_right: Click to view the generated source code :scroll: ](.docs/ab/CODE.md)


## :cl: TicTacToe with enhancced algorithm like `alpha-beta pruning`

### [:five:th](#fiveth-response) Response

:bulb: Second question asked to the Generative AI

> generate tic tac toe game solved with minimax algorithm using alpha beta pruning in Rust 

Certainly! Here's an example implementation of Tic Tac Toe solved with the Minimax algorithm using Alpha-Beta pruning in Rust:

[:point_right: Click to view the generated source code :scroll: ](.docs/5/CODE.md)

This implementation extends the previous Tic Tac Toe game implementation with the Minimax algorithm by adding Alpha-Beta pruning to improve efficiency. Alpha-Beta pruning helps to reduce the number of unnecessary evaluations by cutting off branches in the game tree that are guaranteed to be worse than previously explored options.

Please note that the implementation assumes a standard 3x3 Tic Tac Toe board. If you want to modify the board size, you'll need to adjust the Board type and update the relevant parts of the code accordingly.

- [ ] Execute

```
cargo run
```
> Returns :

[:point_right: Click to view the output when executing the source code :chart: ](.docs/5/OUTPUT.md)

```rust
convert the below code to Vec<(usize, usize)>
 let cells: [[Symbol; 3];3] = self.board.cells.clone();
        let empty_cells = cells
            .iter().enumerate()
            .map(|(row,cols)| 
                    cols.iter().enumerate().map(|(col,_)| (row,col))
                );
```

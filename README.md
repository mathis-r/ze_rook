# ze_rook
A python chess engine strongly inspired by [Sunfish](https://github.com/thomasahle/sunfish)
You can play against it [here](https://lichess.org/@/ZE_ROOK), or import the engine in a [UCI](http://wbec-ridderkerk.nl/html/UCIProtocol.html) GUI such as [PyChess](pychess.org).

I made this engine because I wanted to know how they work, if you want to do the same, here are a few resources I used :
- [Sunfish](https://github.com/thomasahle/sunfish), a chess engine written in Python by Thomas Dybdahl Ahle
- [Carnatus](https://github.com/zserge/carnatus), a sunfish port in Go, by Serge Zaitsev. He gives a nice explaination of how it works in his [blog post](https://zserge.com/posts/carnatus)
- [The Chess Programming Wiki](https://www.chessprogramming.org/Main_Page), where basically every concept is explained

## Features
- Uses the [AlphaBeta](https://www.chessprogramming.org/Alpha-Beta#Negamax_Framework) algorithm, in a negamax framework
- Evaluate a position with [pieces value](https://www.chessprogramming.org/Material), and a [Piece Square Table](https://www.chessprogramming.org/Piece-Square_Tables)
- Written in Python

## Limitations
- Fully relies on the GUI through [UCI](http://wbec-ridderkerk.nl/html/UCIProtocol.html) to stop the game when there is a mat or if the game results in a draw.
- No 3 moves draw rule, or 50 moves draw rule

## Currently, it doesn't have :
- [Quiescence Search](https://www.chessprogramming.org/Quiescence_Search), so it doesn't lose pieces randomly
- [Iterative Deepening](https://www.chessprogramming.org/Iterative_Deepening), to make it search faster and to make it easier to implement a time limit to the search (currently it does a 4 [ply](https://www.chessprogramming.org/Ply) search because it's fast and it results in "correct" moves)

## If you want a small, but powerfull engine :
I can only recommand you to see [Sunfish](https://github.com/thomasahle/sunfish), and its ports in [Rust](https://github.com/Recursing/sunfish_rs) or [Go](https://github.com/zserge/carnatus). This engine is more a proof a concept than anything, but at least it "works"

The name ZE_ROOK comes from a meme : "*...and the idea is to sacrifice* ***THE ROOOOOOK!***"
<audio controls>
  <source src="the-rook.mp3" type="audio/mp3">
Your browser does not support the audio element.
</audio>

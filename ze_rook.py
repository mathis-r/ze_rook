#!/usr/bin/pypy3
# !/usr/bin/python3
from itertools import count

# DOCUMENTATION IS A WIP, YOU MIGHT WANT TO SEE THE SUNFISH ENGINE FOR CLEANER, SIMPLER AND FASTER CODE WITH DOCUMENTATION.
# IF YOU FIND ANY BUG, PLEASE TELL ME, SO I CAN FIX IT :D

# These constants will stay the same, they are used to initialize the board when we enter a new position
a1, h1, a8, h8 = 91, 98, 21, 28
N, E, W, S = -10, 1, -1, 10

initial_position = (
    '         \n'
    '         \n'
    ' rnbqkbnr\n'
    ' pppppppp\n'
    ' ........\n'
    ' ........\n'
    ' ........\n'
    ' ........\n'
    ' PPPPPPPP\n'
    ' RNBQKBNR\n'
    '         \n'
    '         \n'
 )

iwc = [0,0]
ibc = [0,0]
iep = 0
ikp = 0

# Variables (unlike constants above, they will change as the game progress)
global tt # The transposition table, used to keep the best move of each position we've been through
tt = {}

position = (
    '         \n'
    '         \n'
    ' rnbqkbnr\n'
    ' pppppppp\n'
    ' ........\n'
    ' ........\n'
    ' ........\n'
    ' ........\n'
    ' PPPPPPPP\n'
    ' RNBQKBNR\n'
    '         \n'
    '         \n'
 )

wc = [0,0]
bc = [0,0]
ep = 0
kp = 0
color = 'w'

directions = {
    'P': (N, N+N, N+W, N+E),
    'N': (N+N+E, E+N+E, E+S+E, S+S+E, S+S+W, W+S+W, W+N+W, N+N+W),
    'B': (N+E, S+E, S+W, N+W),
    'R': (N, E, S, W),
    'Q': (N, E, S, W, N+E, S+E, S+W, N+W),
    'K': (N, E, S, W, N+E, S+E, S+W, N+W)
}

# Piece square table
pst = {
    'P': (  0,   0,   0,   0,   0,   0,   0,   0,   0,   0, 
            0,   0,   0,   0,   0,   0,   0,   0,   0,   0,
            0,   0,   0,   0,   0,   0,   0,   0,   0,   0,
            0,  78,  83,  86,  73, 102,  82,  85,  90,   0,
            0,   7,  29,  21,  44,  40,  31,  44,   7,   0,
            0, -17,  16,  -2,  15,  14,   0,  15, -13,   0,
            0, -26,   3,  10,   9,   6,   1,   0, -23,   0,
            0, -22,   9,   5, -11, -10,  -2,   3, -19,   0,
            0, -31,   8,  -7, -37, -36, -14,   3, -31,   0,
            0,   0,   0,   0,   0,   0,   0,   0,   0,   0,
            0,   0,   0,   0,   0,   0,   0,   0,   0,   0,
            0,   0,   0,   0,   0,   0,   0,   0,   0,   0),

    'N': (  0,   0,   0,   0,   0,   0,   0,   0,   0,   0, 
            0,   0,   0,   0,   0,   0,   0,   0,   0,   0,
            0, -66, -53, -75, -75, -10, -55, -58, -70,   0,
            0,  -3,  -6, 100, -36,   4,  62,  -4, -14,   0,
            0,  10,  67,   1,  74,  73,  27,  62,  -2,   0,
            0,  24,  24,  45,  37,  33,  41,  25,  17,   0,
            0,  -1,   5,  31,  21,  22,  35,   2,   0,   0,
            0, -18,  10,  13,  22,  18,  15,  11, -14,   0,
            0, -23, -15,   2,   0,   2,   0, -23, -20,   0,
            0, -74, -23, -26, -24, -19, -35, -22, -69,   0,
            0,   0,   0,   0,   0,   0,   0,   0,   0,   0,
            0,   0,   0,   0,   0,   0,   0,   0,   0,   0),

    'B': (  0,    0,   0,   0,   0,   0,   0,   0,   0,  0,
            0,    0,   0,   0,   0,   0,   0,   0,   0,  0,
            0,  -59, -78, -82, -76, -23,-107, -37, -50,  0,
            0,  -11,  20,  35, -42, -39,  31,   2, -22,  0,
            0,   -9,  39, -32,  41,  52, -10,  28, -14,  0,
            0,   25,  17,  20,  34,  26,  25,  15,  10,  0,
            0,   13,  10,  17,  23,  17,  16,   0,   7,  0,
            0,   14,  25,  24,  15,   8,  25,  20,  15,  0,
            0,   19,  20,  11,   6,   7,   6,  20,  16,  0,
            0,   -7,   2, -15, -12, -14, -15, -10, -10,  0,
            0,    0,   0,   0,   0,   0,   0,   0,   0,  0,
            0,    0,   0,   0,   0,   0,   0,   0,   0,  0),

    'R': (  0,    0,   0,   0,   0,   0,   0,   0,   0,  0,
            0,    0,   0,   0,   0,   0,   0,   0,   0,  0,
            0,   35,  29,  33,   4,  37,  33,  56,  50,  0,
            0,   55,  29,  56,  67,  55,  62,  34,  60,  0,
            0,   19,  35,  28,  33,  45,  27,  25,  15,  0,
            0,    0,   5,  16,  13,  18,  -4,  -9,  -6,  0,
            0,  -28, -35, -16, -21, -13, -29, -46, -30,  0,
            0,  -42, -28, -42, -25, -25, -35, -26, -46,  0,
            0,  -53, -38, -31, -26, -29, -43, -44, -53,  0,
            0,  -30, -24, -18,   5,  -2, -18, -31, -32,  0,
            0,    0,   0,   0,   0,   0,   0,   0,   0,  0,
            0,    0,   0,   0,   0,   0,   0,   0,   0,  0),

    'Q': (  0,    0,   0,   0,   0,   0,   0,   0,   0,  0,
            0,    0,   0,   0,   0,   0,   0,   0,   0,  0,
            0,    6,   1,  -8,-104,  69,  24,  88,  26,  0,
            0,   14,  32,  60, -10,  20,  76,  57,  24,  0,
            0,   -2,  43,  32,  60,  72,  63,  43,   2,  0,
            0,    1, -16,  22,  17,  25,  20, -13,  -6,  0,
            0,  -14, -15,  -2,  -5,  -1, -10, -20, -22,  0,
            0,  -30,  -6, -13, -11, -16, -11, -16, -27,  0,
            0,  -36, -18,   0, -19, -15, -15, -21, -38,  0,
            0,  -39, -30, -31, -13, -31, -36, -34, -42,  0,
            0,    0,   0,   0,   0,   0,   0,   0,   0,  0,
            0,    0,   0,   0,   0,   0,   0,   0,   0,  0),

    'K': (  0,    4,  54,  47, -99, -99,  60,  83, -62,  0,
            0,  -32,  10,  55,  56,  56,  55,  10,   3,  0,
            0,  -62,  12, -57,  44, -67,  28,  37, -31,  0,
            0,  -55,  50,  11,  -4, -19,  13,   0, -49,  0,
            0,  -55, -43, -52, -28, -51, -47,  -8, -50,  0,
            0,  -47, -42, -43, -79, -64, -32, -29, -32,  0,
            0,   -4,   3, -14, -50, -57, -18,  13,   4,  0,
            0,   17,  30,  -3, -14,   6,  -1,  40,  18,  0,
            0,    0,   0,   0,   0,   0,   0,   0,   0,  0,
            0,    0,   0,   0,   0,   0,   0,   0,   0,  0)
}

# Board representation
def value(piece):
    """A quick and dirty way to get the value of a piece"""
    if piece == 'P':
        return 100
    elif piece == 'N':
        return 280
    elif piece == 'B':
        return 320
    elif piece == 'R':
        return 479
    elif piece == 'Q':
        return 929
    elif piece == 'K':
        return 60000
    elif piece == 'p':
        return -100
    elif piece == 'n':
        return -280
    elif piece == 'b':
        return -320
    elif piece == 'r':
        return -479
    elif piece == 'q':
        return -929
    elif piece == 'k':
        return -60000
    else:
        return 0

def move_generation(position, wc, ep):
    """Generate an array with all possible moves of the board (pseudo legal moves)"""
    move_list = []
    for i in range(len(position)):
        if position[i].isupper() == True:
            piece = position[i]
            for d in directions[piece]:
                for j in count(i+d, d):
                    destination = position[j]
                    if destination.isspace() or destination.isupper():
                        break
                    if piece == "P":
                        if d in (N, N+N) and destination != ".":
                            break
                        if d == N+N and (i < a1+N or position[i+N] != "."):
                            break
                        if d in (N+W,N+E) and destination == "." and j+S != ep:
                            break
                        if a8 <= j <= h8:
                            for p in "QRBN":
                                move_list.append([i, j, p, destination])
                            break
                    move_list.append([i, j, "", destination])
                    if piece in "PNK" or destination.islower():
                        break
                    if i == a1 and position[j + E] == "K" and wc[0] == 0:
                        move_list.append([j + E, j + W, "", destination])
                    if i == h1 and position[j + W] == "K" and wc[1] == 0:
                        move_list.append([j + W, j + E, "", destination])
    return move_list

def move(position, move, wc, bc, ep, kp):
    """Apply a move on a position"""
    listpos = list(position)
    i, j = move[0], move[1]
    prom = move[2]
    piece = listpos[i]
    kp = 0
    if i == a1:
        wc[0] = 1
    if i == h1:
        wc[1] = 1
    if j == a8:
        bc[1] = 1
    if j == h8:
        bc[0] = 1
    if piece == "K":
        kp = 0
        wc[0], wc[1] = 1, 1
        if i-j == 2:
            kp = (j+i)//2
            listpos[a1], listpos[kp] = listpos[kp], listpos[a1]
        if j-i == 2:
            kp = (j+i)//2
            listpos[h1], listpos[kp] = listpos[kp], listpos[h1]
    if piece == "P":
        if j+S == ep:
            listpos[j+S] = "."
            ep = 0
        ep = 0 # else if only "standard" pawn moves are played, ep stay the same, and it could result in illegal moves. So we set it to 0 here.
        if a8 <= j <= h8:
            listpos[i] = prom
            ep = 0
        if j-i == N+N:
            ep = j
    else:
        ep = 0
    listpos[i], listpos[j] = ".", listpos[i]
    position = ''.join(listpos)
    return position, wc, bc, ep, kp

def rotate(position, wc, bc, ep, kp):
    """Rotate the board"""
    ep = 119 - ep
    kp = 119 - kp
    wc, bc = bc, wc
    listpos = list(position)
    for i in range(0,60):
        if listpos[i] != '\n' and listpos[i] != ' ':
            listpos[119-i], listpos[i] = listpos[i].swapcase(), listpos[119-i].swapcase()
    position = ''.join(listpos)
    return position, wc, bc, ep, kp


def search_check(position, kp):
    king = 1
    for i in range(len(position)):
        if position[i] == "k":
            king = i
    if king == 1:
        return 1
    castling = False
    king_ori = 0
    if kp in [23,25]:
        king_ori = 24
        castling = True
    if kp in [24,26]:
        king_ori = 25
        castling = True
    if kp in [93,95]:
        king_ori = 94
        castling = True
    if kp in [94,96]:
        king_ori = 95
        castling = True
    move_list = move_generation(position, [1,1], 0)
    for i in range(len(move_list)):
        if move_list[i][1] == king or move_list[i][1] == kp:
            return 1
        if castling:
            if move_list[i][1] == king_ori:
                return 1
    return 0

def evaluate_mv(position, move, ep):
    i, j, prom, destination = move
    piece = position[i]
    score = pst[piece][j] - pst[piece][i]
    if destination.islower() == True:
        score += pst[destination.swapcase()][119-j]
    if piece == "K" and (i-j == 2 or j-i == 2):
        score += pst["R"][(i+j)//2]
        score -= pst["R"][a1 if j < i else h1]
    if piece == "P":
        if a8 <= j <= h8:
            score += pst[prom][j] - pst["P"][j]
        if j+S == ep:
            score += pst["P"][119- (j+S)]
    return score

def evaluate_pos(position):
    score = 0
    for i in range(len(position)):
        piece = position[i]
        if piece.isupper() == True:
            score += value(piece) + pst[piece][i]
        elif piece.islower() == True:
            score += value(piece)
            piece = position[i].swapcase()
            score = score - pst[piece][119-i]
    return score

# Search
def alphabeta(alpha, beta, depth, position, wc, bc, ep, kp):
    """AlphaBeta algorithm, in a negamax framework, without quiescence search, iterative deepening"""
    """Will try to find the best move on the board"""
    if depth == 0:
        return evaluate_pos(position), None #In the future, I might add quiescence search to the engine, so that it doesn't blunder to much.
    killer = tt.get(str(position))
    move_list = move_generation(position, wc[:], ep)
    if killer:
        alpha = evaluate_pos(position) + evaluate_mv(position, killer, ep)
        bestmove = killer
    else:
        moves = []
        score_m = []
        move_list = move_generation(position, wc[:], ep)
        for i in range(len(move_list)):
            testmove = move(position, move_list[i], wc[:], bc[:], ep, kp)
            newpos = rotate(testmove[0], testmove[1], testmove[2], testmove[3], testmove[4])
            if search_check(newpos[0], newpos[4]) == 0:
                score = - alphabeta(-beta, -alpha, depth - 1, newpos[0], newpos[1], newpos[2], newpos[3], newpos[4])[0]
                moves.append(move_list[i])
                score_m.append(score)
                if score >= beta:
                    return beta, None
                if score > alpha:
                    alpha = score
        if len(score_m) == 0:
            newpos = rotate(position, wc[:], bc[:], ep, kp)
            if search_check(newpos[0], 0) == 1: # if the position is mate
                alpha = -MATELOWER + 4 - depth # mate - nb of moves to get there => get to the mate as fast as possible
                bestmove = None
            else:                           # if the position is stalemate
                alpha = 0
                bestmove = None
        else:
            maxscore = max(score_m)
            indexmax = score_m.index(maxscore)
            if depth > 2:
                tt[str(position)] = moves[indexmax]
            bestmove = moves[indexmax]
    return alpha, bestmove

# UCI implementation
MATEUPPER = value("K") + 10 * value("Q")
MATELOWER = value("K") - 10 * value("Q")

def fen(fen_str):
    """Generate the board and the variables with a FEN string"""
    position = 120*[' ']
    wc = []
    bc = []
    ep = 0
    kp = 0
    listpos = list(position)
    parts = fen_str.split()
    rows = parts[0].split("/")
    if len(rows) != 8:
        return position, wc, bc, ep, kp, "FEN shuld have 8 rows"
    for r in range(8):
        index = r*10 + 21
        for p in rows[r]:
            if p in "KQRBNPkqrbnp":
                listpos[index] = p
                index +=1
            else:
                try:
                    p = int(p)
                except:
                    return position, wc, bc, ep, kp, "invalid FEN string"
                if 1<=p<=8:
                    for i in range(p):
                        listpos[index] = '.'
                        index += 1
    for i in range(120):
        if i % 10 == 9:
            listpos[i] = "\n"
    position = ''.join(listpos)
    color = parts[1]
    castling = parts[2]
    wc = [0 if "Q" in castling else 1, 0 if "K" in castling else 1]
    bc = [0 if "k" in castling else 1, 0 if "q" in castling else 1]
    ep = parse(parts[3]) if parts[3] != "-"  else 0
    if color == 'b':
        position, wc, bc, ep, kp = rotate(position, wc, bc, ep, kp)
    return position, wc, bc, ep, kp, color

def parse(c):
    fil, rank = ord(c[0]) - ord("a"), int(c[1]) - 1
    return a1 + fil - 10 * rank

def render(i):
    rank, fil = divmod(i - a1, 10)
    return chr(fil + ord("a")) + str(-rank + 1)

while True:
    args = input().split()
    if args[0] == "uci":
        print("id name ZE_ROOK v0.1")
        print("uciok")

    elif args[0] == "isready":
        print("readyok")

    elif args[0] == "quit":
        break

    elif args[:2] == ["position", "startpos"]:
        position, wc, bc, ep, kp = initial_position, iwc, ibc, iep, ikp
        color = 'w'
        if len(args) > 3:
            if args[2] == "moves":
                ply = 0
                for ply, move_str in enumerate(args[3:]):
                    i, j, prom = parse(move_str[:2]), parse(move_str[2:4]), move_str[4:].upper()
                    if color == 'b':
                        i, j = 119 - i, 119 - j
                    if ply % 2 == 1:
                        i, j = 119-i, 119-j
                        position, wc, bc, ep, kp = rotate(position, wc, bc, ep, kp)
                        position, wc, bc, ep, kp = move(position, [i, j, prom, "."], wc, bc, ep, kp)
                        position, wc, bc, ep, kp = rotate(position, wc, bc, ep, kp)
                    else:
                        position, wc, bc, ep, kp = move(position, [i, j, prom, "."], wc, bc, ep, kp)
                if ply % 2 == 0 and color == 'w':
                    position, wc, bc, ep, kp = rotate(position, wc, bc, ep, kp)
                    color = 'b'
                elif ply % 2 == 0 and color == 'b':
                    position, wc, bc, ep, kp = rotate(position, wc, bc, ep, kp)
                    color = 'w'
            kp = 0

    elif args[:2] == ["position", "fen"]:
        fen_str = ''
        for i in range(len(args[2:8])):
            fen_str = fen_str + args[2:8][i] +' '
        position, wc, bc, ep, kp, color = fen(fen_str)
        if len(args) >= 9:
            if args[8] == "moves":
                ply = 0
                for ply, move_str in enumerate(args[9:]):
                    i, j, prom = parse(move_str[:2]), parse(move_str[2:4]), move_str[4:].upper()
                    if color == 'b':
                        i, j = 119 - i, 119 - j
                    if ply % 2 == 1:
                        i, j = 119-i, 119-j
                        position, wc, bc, ep, kp = rotate(position, wc, bc, ep, kp)
                        position, wc, bc, ep, kp = move(position, [i, j, prom, "."], wc, bc, ep, kp)
                        position, wc, bc, ep, kp = rotate(position, wc, bc, ep, kp)
                    else:
                        position, wc, bc, ep, kp = move(position, [i, j, prom, "."], wc, bc, ep, kp)
                if ply % 2 == 0 and color == 'w':
                    position, wc, bc, ep, kp = rotate(position, wc, bc, ep, kp)
                    color = 'b'
                elif ply % 2 == 0 and color == 'b':
                    position, wc, bc, ep, kp = rotate(position, wc, bc, ep, kp)
                    color = 'w'
        kp = 0

    elif args[0] == "go":
        move_str = None
        scmv = alphabeta(-MATEUPPER, MATEUPPER, 4, position, wc[:], bc[:], ep, kp)
        move_bfr_str = scmv[1]
        if move_bfr_str == None:
            break
        i, j = move_bfr_str[0], move_bfr_str[1]
        if color  == 'b':
            i, j = 119 - i, 119 - j
        move_str = render(i) + render(j) + move_bfr_str[2].lower()

        print("bestmove", move_str or '(none)')

# Perft algorithm
def perft(depth, position, wc, bc, ep, kp):
    n_moves = 0
    nodes = 0
    move_list = move_generation(position, wc[:], ep)
    legalmove = []
    for i in range(len(move_list)):
        testmove = move(position, move_list[i], wc[:], bc[:], ep, kp)
        newpos = rotate(testmove[0], testmove[1], testmove[2], testmove[3], testmove[4])
        if search_check(newpos[0], newpos[4]) == 0:
            legalmove.append(testmove)
            print(testmove[0])
    n_moves = len(legalmove)
    if depth == 1:
        return n_moves
    i=0
    for i in range(n_moves):
        pos = rotate(legalmove[i][0], legalmove[i][1], legalmove[i][2], legalmove[i][3], legalmove[i][4])
        nodes += perft(depth-1, pos[0], pos[1], pos[2], pos[3], pos[4])
    return nodes

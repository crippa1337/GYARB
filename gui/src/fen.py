from ursina import Button, color
from utils import create_tile


def parse_fen(fen: str) -> tuple[list[list[Button]], str, int, int]:
    board: list[list[None | Button]] = [[None for _ in range(7)] for _ in range(7)]
    x = 0
    y = 6
    pos, player, half, full, *_ = fen.split()
    print(pos)
    for ch in pos:
        if ch == "x":
            b = create_tile(x, y)
            b.text, b.text_color = "X", color.orange
            board[x][y] = b
            x += 1
        elif ch == "o":
            b = create_tile(x, y)
            b.text, b.text_color = "O", color.azure
            board[x][y] = b
            x += 1
        elif ch == "-":
            b = create_tile(x, y, True)
            board[x][y] = b
            x += 1
        elif ch.isnumeric():
            for _ in range(int(ch)):
                b = create_tile(x, y)
                board[x][y] = b
                x += 1
        elif ch == "/":
            x = 0
            y -= 1
    return board, player, int(half), int(full)


def generate_fen(board: list[list[Button]], player: str, half: int) -> str:
    fen = ""

    def get_char(tile) -> str:
        if tile.text == "X":
            return "x"
        elif tile.text == "O":
            return "o"
        elif tile.color == color.black:
            return "-"
        return ""

    x = 0
    y = 6
    for y in range(6, -1, -1):
        i = 0
        for x in range(0, 7):
            tile = board[x][y]
            ch = get_char(tile)
            if ch == "":
                i += 1
                continue
            elif i > 0:
                fen += str(i)
                i = 0
            fen += ch
        if i > 0:
            fen += str(i)
        fen += "/"
        y -= 1
    return f"{fen} {player.lower()} {half} {(int(half) // 2) + 2}"

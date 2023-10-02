from re import S
from ursina import Button, Entity, color, Text

from uai import Handler
from fen import parse_fen


class Game:
    selected: Button | None
    handler: Handler | None
    move_stack: list[str]

    def __init__(self, board, handler=None) -> None:
        self.player = Entity(name="X", color=color.orange)
        self.turn_text = t = Text(
            text=f"Turn: {self.player.name}",
            scale=2,
            position=(-0.85, 0.45),
            color=self.player.color,
        )
        self.board = board
        self.selected = None
        self.handler = handler
        self.half_moves = 0
        self.moves = 1

    def load_fen(self, fen):
        board, player, half, full = parse_fen(fen)
        if player == "o":
            self.swap_turn()
        self.board = board
        self.half_moves = half
        self.moves = full

    def move_dup(self, new):
        pos = new.position
        x = int(pos.x)
        y = int(pos.y)
        # Update surrounding tiles
        for i in range(-1, 2):
            for j in range(-1, 2):
                x_rel = x + i
                y_rel = y + j
                if 6 < x_rel or x_rel < 0:
                    continue
                if 6 < y_rel or y_rel < 0:
                    continue
                t = self.board[x + i][y + j]
                if t.text is not None:
                    t.text = self.player.name
                    t.text_color = self.player.color

    def move_jmp(self, new):
        if self.selected is None:
            return
        self.selected.text = ""
        self.selected.text_color = None
        self.move_dup(new)

    def click_tile(self, tile):
        if self.selected is not None and (tile.text is None or tile.text == ""):
            # Get the Chebyshev Distance
            p_1 = self.selected.position
            p_2 = tile.position
            chebyshev_distance = max(abs(p_1.x - p_2.x), abs(p_1.y - p_2.y))

            if chebyshev_distance == 2.0:
                self.move_jmp(tile)
            elif chebyshev_distance == 1.0:
                self.move_dup(tile)
            else:
                return

            # Set the new squares properties
            tile.text = self.player.name
            tile.text_color = self.player.color

            # Reset for next turn
            self.swap_turn()
        # Select a tile
        elif tile.text == self.player.name:
            self.selected = tile

    def swap_turn(self):
        if self.player.name == "X":
            self.player.name = "O"
            self.player.color = color.azure
        else:
            self.player.name = "X"
            self.player.color = color.orange

            self.moves += 1

        self.half_moves += 1

        self.turn_text.text = f"Turn: {self.player.name}"
        self.turn_text.color = self.player.color
        self.selected = None

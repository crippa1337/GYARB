from ursina import *
from fen import generate_fen, parse_fen

if __name__ == '__main__':
    app = Ursina()

camera.orthographic = True
camera.fov = 8
camera.position = (3, 3)
Text.default_resolution *= 2





class Game:
    selected: Button | None
    def __init__(self, board) -> None:
        self.player = Entity(name='X', color=color.orange)
        self.turn_text = t = Text(text=f'Turn: {self.player.name}',
                                  scale=2, position=(-0.85, 0.45), color=self.player.color)
        self.board = board
        self.selected = None
    
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
        self.selected.text = ''
        self.selected.text_color = None
        self.move_dup(new)

    def click_tile(self, tile):
        if self.selected is not None and (tile.text is None or tile.text == ''):
            # Get the Chebyshev Distance
            p_1 = self.selected.position
            p_2 = tile.position
            chebyshev_distance = max(abs(p_1.x - p_2.x), abs(p_1.y - p_2.y))

            if chebyshev_distance == 2.:
                self.move_jmp(tile)
            elif chebyshev_distance == 1.:
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
        if self.player.name == 'X':
            self.player.name = 'O'
            self.player.color = color.azure
        else:
            self.player.name = 'X'
            self.player.color = color.orange

        self.turn_text.text = f'Turn: {self.player.name}'
        self.turn_text.color = self.player.color
        self.selected = None








def main():
    game = Game([])
    game.board = parse_fen("x6/-6/7/7/7/7/o5x")
    print(generate_fen(game.board))
    scene.game = game

    

if __name__ == '__main__':
    main()
    app.run()

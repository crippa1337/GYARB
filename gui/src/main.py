from ursina import *

if __name__ == '__main__':
    app = Ursina()

camera.orthographic = True
camera.fov = 8
camera.position = (3, 3)
Text.default_resolution *= 2


class Game:
    def __init__(self) -> None:
        self.player = Entity(name='X', color=color.orange)
        self.turn_text = t = Text(text=f'Turn: {self.player.name}',
                                  scale=2, position=(-0.85, 0.45), color=self.player.color)
        self.selected = None

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

game = Game()
player = game.player
board = [[None for _ in range(7)] for _ in range(7)]
selected = None

def create_new(tile):
    pos = tile.position
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
            t = board[x + i][y + j]
            if t.text != None:
                t.text = player.name
                t.text_color = player.color

def jump(old, new):
    old.text = ''
    old.text = None
    old.text_color = None
    create_new(new)

# Initial board state
for y in range(7):
    for x in range(7):
        b = Button(parent=scene, position=(x, y), scale=1, radius=0)
        board[x][y] = b

        def on_click(b=b):
            selected = game.selected
            # Make a move
            if selected != None and b.text == None:
                # Get the Chebyshev Distance
                p1 = selected.position
                p2 = b.position
                distance = max(abs(p1.x - p2.x), abs(p1.y - p2.y))

                if distance == 2.:
                    jump(selected, b)
                elif distance == 1.:
                    create_new(b)
                else:
                    return
                
                # Set the new squares properties
                b.text = player.name
                b.text_color = player.color
                
                # Reset for next turn
                game.swap_turn()
            # Select a tile
            elif b.text == player.name:
                game.selected = b


        b.on_click = on_click
        
        

board[6][0].text, board[6][0].text_color = 'X', color.orange
board[0][6].text, board[0][6].text_color = 'X', color.orange
board[0][0].text, board[0][0].text_color = 'O', color.azure
board[6][6].text, board[6][6].text_color = 'O', color.azure

if __name__ == '__main__':
    app.run()

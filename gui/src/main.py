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

    def swap_turn(self):
        if self.player.name == 'X':
            self.player.name = 'O'
            self.player.color = color.azure
        else:
            self.player.name = 'X'
            self.player.color = color.orange

        self.turn_text.text = f'Turn: {self.player.name}'
        self.turn_text.color = self.player.color


game = Game()
player = game.player
board = [[None for _ in range(7)] for _ in range(7)]

# Initial board state
for y in range(7):
    for x in range(7):
        b = Button(parent=scene, position=(x, y), scale=1, radius=0)
        board[x][y] = b

        def on_click(b=b):
            b.text = player.name
            b.text_color = player.color
            game.swap_turn()

        b.on_click = on_click

board[6][0].text, board[6][0].text_color = 'X', color.orange
board[0][6].text, board[0][6].text_color = 'X', color.orange
board[0][0].text, board[0][0].text_color = 'O', color.azure
board[6][6].text, board[6][6].text_color = 'O', color.azure

if __name__ == '__main__':
    app.run()

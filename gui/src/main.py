from ursina import *
from fen import generate_fen, parse_fen
from game import Game

if __name__ == "__main__":
    app = Ursina()

camera.orthographic = True
camera.fov = 8
camera.position = (3, 3)
Text.default_resolution *= 2


def main():
    game = Game([])
    game.board = parse_fen("x6/-6/7/7/7/7/o5x")
    print(generate_fen(game.board))
    scene.game = game


if __name__ == "__main__":
    main()
    app.run()

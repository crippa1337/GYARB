from ursina import camera, Text, scene, Ursina
from fen import generate_fen, parse_fen
from game import Game
from uai import Handler

camera.orthographic = True
camera.fov = 8
camera.position = (3, 3)
Text.default_resolution *= 2

def ursina_setup():
    camera.orthographic = True
    camera.fov = 8
    camera.position = (3, 3)
    Text.default_resolution *= 2


def main():
    game = Game([])
    scene.game = game
    game.load_fen("x6/-6/7/7/7/7/o5x o 1 1")
    pass


if __name__ == "__main__":
    app = Ursina()
    ursina_setup()
    main()
    app.run()

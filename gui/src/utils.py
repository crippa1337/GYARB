from ursina import Button, color, scene

def create_tile(x, y, blocked=False) -> Button:
    clr = color.gray
    if blocked:
        clr = color.black
    b = Button(parent=scene, position=(x, y), scale=1, radius=0, color=clr)

    # Blocked tiles cannot be interacted with, they do not need an on click listener
    if blocked:
        return b

    def on_click(b=b):
        b.parent.game.click_tile(b)

    b.on_click = on_click
    return b
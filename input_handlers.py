import libtcodpy as libtcod


def handle_keys(key):
    # Movement keys
    if key.vk == libtcod.KEY_UP:
        return {'move': (0, -1)}
    elif key.vk == libtcod.KEY_DOWN:
        return {'move': (0, 1)}
    elif key.vk == libtcod.KEY_LEFT:
        return {'move': (-1, 0)}
    elif key.vk == libtcod.KEY_RIGHT:
        return {'move': (1, 0)}

    elif key.vk == libtcod.KEY_KP1: #Down left
        return {'move': (-1, 1)}
    elif key.vk == libtcod.KEY_KP2: #Down
        return {'move': (0, 1)}
    elif key.vk == libtcod.KEY_KP3: #Down right
        return {'move': (1, 1)}
    elif key.vk == libtcod.KEY_KP4: #Left
        return {'move': (-1, 0)}
    elif key.vk == libtcod.KEY_KP5: #Wait
        return {'move': (0, 0)}
    elif key.vk == libtcod.KEY_KP6: #Right
        return {'move': (1, 0)}  
    elif key.vk == libtcod.KEY_KP7: #Up left
        return {'move': (-1, -1)}
    elif key.vk == libtcod.KEY_KP8: #Up
        return {'move': (0, -1)} 
    elif key.vk == libtcod.KEY_KP9: #Up right
        return {'move': (1, -1)}

    

    if key.vk == libtcod.KEY_ENTER and key.lalt:
        # Alt+Enter: toggle full screen
        return {'fullscreen': True}
    elif key.vk == libtcod.KEY_ESCAPE:
        # Exit the game
        return {'exit': True}

    # No key was pressed
    return {}
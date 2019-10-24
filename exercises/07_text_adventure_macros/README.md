# Overview

This week, the focus of the club is to modify the previous weeks exercises to make use of macros.

The thinking here is to allow us to provide a `game_state!` macro, which will allow us to declaratively define a complete world instance, with all the rooms, items, links between rooms and the player, configured appropriately.

In essence, this would allow us to go from creating each item and adding them to their appropriate containers, to something very loosely like:
```
let world = game_state!({
    "room_1": {
        "items": [
            "stick",
            "stone"
        ]
    },
    "room_2": {
        "items": [],
    },
    "player": {
       "name": "John Smith"
   }
})
```

This code would effectively compile to code that would execute to return a world instance with the appropriate rooms and player configured as specified.

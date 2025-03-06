# Game logic

This package is divided in two modules:

## MAP:

Contains a matrix that represents the layout of the map as a 2D grid, you can set values in the matrix from 0 (void space) up to 4 (Door), to see the available textures refer to the `textures` package.

## Player:

Contains the player structure that contains the following values:
* Position in x
* Position in y
* Direction angle

And the methods used to update the player position and direction implementing collision detection algorithms and bounding de value of the angle between 0 and 2π.

TODO
==

# Problem

Rule with suicide enabled cannot be implemented for the moment.

# Optimizing

- Use a string structure and don't compute liberties on the fly. ? 
- Use a better structure for storing stones ?
- Zobrist hashing for super ko detection
- Using a struct more efficient than Vec<bool> (bitvec ?)

# Functionalities

- Detect dead stones at the end of the game.
- SGF Import
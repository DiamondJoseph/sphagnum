# Sphagnum: Seeded Plaything for Heuristic Antagonism of Galactic Nasties (Um, Mahact)

Hobby project for learning Rust, implementing a map creator for Twilight Imperium for the RAW method.

Most map generators online either generate the whole map at once, or are implementing a Milty draft.

When playing Twilight Falls mode recently, I wanted to run RAW map generation which is more unbalanced, and an opportunity to be unfair and cruel to one another, in keeping with the lore of this game mode. However, it requires creating and maintaining multiple piles of cards that are secret to whoever is running the draft if they intend to play, and therefore ate into gameplay time by having to be run IRL.

Therefore, the aim of this project is to get to a point where the executable can be distributed to multiple players and due to a seeded random number generator, the cards they receive will be deterministic and exclusive with the cards other players see.

Initial goals:
- [ ] Presenting CLI options to input names of players
- [ ] Generating and presenting a seed for the creator of the draft
- [ ] Creating 6 "hands" of system tiles: 3 blue backed, 2 red backed
- [ ] Allowing use of an existing seed + partial map to regain "your" hand
- [ ] When using existing seed + partial map, tracking by use of existing state which player's turn to place a tile
- [ ] Allowing placing a tile in RAW method: 1st ring complete before placing in 2nd ring, 2nd ring complete before placing in 3rd ring, anomalies not adjacent unless no other valid placement, green backed system only in corner hexes
- [ ] Interoperability with existing online map generators/viewers (i.e. adopting spiral-like system ordering)

Stretch goals:
- [ ] Player count selection (4-8 inclusive), with hyperlanes automatically added and considered for anomaly placement
- [ ] Twilight Falls drafting support, by drafting the relevant parts of the faction cards
- [ ] Graphical support, either as a TypeScript web GUI or CLI rendering
- [ ] Support for other game modes (Minor Factions?)
- [ ] Support for third party tiles
- [ ] Summarising of "slices"/"sectors", relevant secret objectives to assist in making informed decisions

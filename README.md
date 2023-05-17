# Source Engine Server Searcher
SESS is an Open Source project that aims to fully recreate the Source 
Engine Server Searcher (also named Community Server Browser) from
Valve's Multiplayer Source Games like 
[TF2](https://store.steampowered.com/app/440/Team_Fortress_2/), 
[Half-Life 2: Deathmatch](https://store.steampowered.com/app/320/HalfLife_2_Deathmatch/), 
[CS:GO](https://store.steampowered.com/app/730/CounterStrike_Global_Offensive/) and
many others.

# Backend
This is the backend side of the project, providing a "lighting-fast"
[GraphQL](https://graphql.org/) api made in pure [Rust](https://www.rust-lang.org/) 
(using [Juniper](https://github.com/graphql-rust/juniper)
for the GraphQL server and [GameDig](https://github.com/gamedig/rust-gamedig) 
for querying each server and communicating with the master server).

# Live service
The master branch is publicly available 
[here](http://cosminperram.com:20240/playground), the service redeploys everytime a
commit is pushed, and as such, this is not a reliable endpoint, use just for 
playground purposes.

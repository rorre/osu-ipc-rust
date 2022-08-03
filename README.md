# osu! Difficulty Calculator IPC

<center>
A blazing fast osu! calculator IPC alternative written in Rust.

\<insert blazing fast meme here\>

Made possible with [MaxOhn's rosu-pp](https://github.com/MaxOhn/rosu-pp).

</center>

## Running

1. Download latest binary from [latest release](https://github.com/rorre/osu-ipc-rust/releases/latest)
2. Run it
3. Run osu!

## FAQ

### The Star Rating doesn't automatically update!

osu! will only request for update whenever there are changes to the beatmap, else it will use cached version
available in `osu!.db`.

> You may remove this file to force osu! to rebuild the entire database and use IPC to recalculate,
> but depending on how large your library is, **this could take a long time**.  
> [See here for more information about osu!.db](https://github.com/ppy/osu/wiki/Legacy-database-file-structure#osudb)

### osu! doesn't call IPC at all!

Try running the server first before osu!

### Some maps shows 0 SR!

Other maps is probably in the process of calculating. Usually processing a map is blazingly fast, but there are times such as calculating [fanzhen's XNOR XNOR XNOR](https://osu.ppy.sh/beatmapsets/1236927#osu/2619200) takes a really long time to do and blocks other difficulty calculation updates.

### Why is this single threaded?

osu! only sends the request one at a time and waits until the connection is dropped, so there is no use to make it multi-threaded.

### Where do you base the code from?

From [osu!lazer's LegacyIpc folder](https://github.com/ppy/osu/tree/master/osu.Desktop/LegacyIpc) and [osu!framework's TcpIpcProvider](https://github.com/ppy/osu-framework/blob/master/osu.Framework/Platform/TcpIpcProvider.cs)

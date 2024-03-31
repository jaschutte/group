# Group
Group strings together and filter on each group.

# Examples
## Basic
From a small list of 3 elements, keep all elements which contain 'bar':
`$ echo 'foo $ bar $ foobar' | group -s ' $ ' -f 'bar'`
Output:
```
bar
foobar
```

## Hyprland
Retrieves all Hyprland clients and filters based on the name (in this case, Spotify):
`$ hyprctl clients | group -f 'Spotify'`
Output:
```
Window 4d8bcd0 -> Spotify:
	mapped: 1
	hidden: 0
	at: 0,0
	size: 500,500
	workspace: 1 (1)
	floating: 0
	monitor: 0
	class:
	title: Spotify
	initialClass:
	initialTitle: Spotify
	pid: 1
	xwayland: 0
	pinned: 0
	fullscreen: 0
	fullscreenmode: 0
	fakefullscreen: 0
	grouped: 0
	swallowing: 0
	focusHistoryID: 1
```

# Note
All filtering is done using literal matching, no special character manipulation is done.

This means regex is not supported.


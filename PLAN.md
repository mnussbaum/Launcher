### GUI

- Initially show search box
- User can also type keywords to narrow down queries
- After search query entered expand to show results
- Eventually allow hovered results to show a preview window
- Preview window should be able to display text and images
- Maybe preview window should have some interactive abilities eventually

### Query lang / plugin system

- All plugins receive text + keywords for every query
- Plugins should check the keyword set and quickly respond with an empty set to
  queries that use keywords that the plugin doesn't accept
- Possible alternative would be to have plugins explicitly register the
  keywords they accept
- Plugins will accept input over STDIN
- Plugins can take STDIN, run, print response to STDOUT and exit
- Or plugins can stay up, keep streaming inputs and outputs and not get
  restarted with each query/command

- Have some way to explicitly limit queries to a single plugin, maybe a special
  keyword
- Plugins will need to accept query text + keywords and return a result list
- Plugins will need to accept a selected result
- Plugins will eventually need to accept a preview request for a result

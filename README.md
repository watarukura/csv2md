# csv2md

input csv, output markdown table.

## usage

```shell
$ ./csv2md --help
csv2md 1.0.0

USAGE:
    csv2md [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --delimiter <delimiter>    (option) delimiter [default: \t]
    -H, --header <header>          (option) is header exists (true|false) [default: true]
```

## example

```shell
$ head uspop.csv 
City,State,Population,Latitude,Longitude
Davidsons Landing,AK,,65.2419444,-165.2716667
Kenai,AK,7610,60.5544444,-151.2583333
Oakman,AL,,33.7133333,-87.3886111
Richards Crossroads,AL,,31.7369444,-85.2644444
Sandfort,AL,,32.3380556,-85.2233333
Selma,AL,18980,32.4072222,-87.0211111
Shadow Oaks Addition,AR,,34.9555556,-91.9475000
Summerville,AR,,33.5202778,-92.3555556
El Mirage,AZ,32308,33.6130556,-112.3238889
```

```shell
$ ./csv2md --delimiter=, <(cat uspop.csv | head)
City|State|Population|Latitude|Longitude
 -- | -- | -- | -- | -- 
Davidsons Landing|AK||65.2419444|-165.2716667
Kenai|AK|7610|60.5544444|-151.2583333
Oakman|AL||33.7133333|-87.3886111
Richards Crossroads|AL||31.7369444|-85.2644444
Sandfort|AL||32.3380556|-85.2233333
Selma|AL|18980|32.4072222|-87.0211111
Shadow Oaks Addition|AR||34.9555556|-91.9475000
Summerville|AR||33.5202778|-92.3555556
```

City|State|Population|Latitude|Longitude
-- | -- | -- | -- | --
Davidsons Landing|AK||65.2419444|-165.2716667
Kenai|AK|7610|60.5544444|-151.2583333
Oakman|AL||33.7133333|-87.3886111
Richards Crossroads|AL||31.7369444|-85.2644444
Sandfort|AL||32.3380556|-85.2233333
Selma|AL|18980|32.4072222|-87.0211111
Shadow Oaks Addition|AR||34.9555556|-91.9475000
Summerville|AR||33.5202778|-92.3555556

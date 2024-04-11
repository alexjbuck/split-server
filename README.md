# Split Server

[![Release](https://github.com/alexjbuck/split-server/actions/workflows/release.yml/badge.svg)](https://github.com/alexjbuck/split-server/actions/workflows/release.yml)
[![Security audit](https://github.com/alexjbuck/split-server/actions/workflows/security-audit.yml/badge.svg)](https://github.com/alexjbuck/split-server/actions/workflows/security-audit.yml)
[![Build & Test](https://github.com/alexjbuck/split-server/actions/workflows/check-and-test.yml/badge.svg)](https://github.com/alexjbuck/split-server/actions/workflows/check-and-test.yml)

## API

This server has the following API:
- `/` => List all missions and their split in a JSON format.
The format is:
```
{
  [
    {
      "name":"Mission Name",
      "split":"Test"
    },
    {...},
    ...
  ]
}
```

- `/split/<name>` => List all splits corresponding to records which match on
  `name`.
The format is:
```
# Single matching split
["Test"]
# Multiple matching splits (this is probably bad)
["Train","Validation"]
```

- `/list/<split>` => List all the missions corresponding to records which match
  on `split`. This API only supports split definitions defined in
  [Splits](#splits).
The format is:
```
# No matching missions
[]
# Single matching mission
["Mission 1"]
# Multiple matching splits (this is probably bad)
["Mission 1","Mission 2"]
```

## Splits

There are 4 splits definitions this API implements:
1. `train` => `DataSplit::Train`
1. `validation` => `DataSplit::Validation`
1. `test` => `DataSplit::Test`
1. `_` => `DataSplit::Unknown`

The _ case represents a catchall case.
Example:
```
# Get request to /list/random-string returns
["list of missions","with unknown","as their datasplit"]
# If there are no "unknown's in the dataset
[]
```
## Getting started

Download the release archive and extract it. In the folder named `static` edit
the csv file named `data.csv` to reflect your datasets/splits. The webserver will
read this file when handling queries.

Folder structure
```
- split-server(.exe)
- static\
  - data.csv
```

Then simply run `split-server`.

## Updating list

Replace the `data.csv` file with an updated one, following the same format.

## CSV Format
The required CSV format is 
```
name,split
Mission 1, test
mission_2, validation
mission-3!!~! (who made this formatting??), train
```
Strictly speaking, the header row is required but the actual text is not used.
For clarity the header row should remain `name,split` to describe the data
format.

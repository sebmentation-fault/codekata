# Kata 04: Data Munging

> See [Kata 04: Data Munging](http://codekata.com/kata/kata04-data-munging/)

Martin Fowler gave me a hard time for Kata02, complaining that it was yet another
single-function, academic exercise. Which, or course, it was. So this week let’s
mix things up a bit.

Here’s an exercise in three parts to do with real world data. Try hard not to read
ahead—do each part in turn.

## Part One: Weather Data

In `weather.dat` you’ll find daily weather data for Morristown, NJ for June 2002.
Download this text file, then write a program to output the day number (column one)
with the smallest temperature spread (the maximum temperature is the second column,
the minimum the third column).

## Part Two: Soccer League Table

The file `football.dat` contains the results from the English Premier League for
2001/2. The columns labeled ‘F’ and ‘A’ contain the total number of goals scored
for and against each team in that season (so Arsenal scored 79 goals against opponents,
and had 36 goals scored against them). Write a program to print the name of the
team with the smallest difference in ‘for’ and ‘against’ goals.

## Part Three: DRY Fusion

Take the two programs written previously and factor out as much common code as possible,
leaving you with two smaller programs and some kind of shared functionality.

## Kata Questions

To what extent did the design decisions you made when writing the original programs
make it easier or harder to factor out common code?

Was the way you wrote the second program influenced by writing the first?

Is factoring out as much common code as possible always a good thing? Did the readability
of the programs suffer because of this requirement? How about the maintainability?

## Solutions/Working-out

1. When writing the original programs, individually, I was understanding their
   individual needs -- for example, the weather file has 2-row header, and a footer
   too, both which are irrelevant when parsing the spreads. I also saw from the
   football file that some tables have rows in the middle which should be ignored.
   If I had tried to preemptively factor out common code, I may not have spotted
   these until later, and then had to refactor the library code (and hence also
   refactor the dependent binaries).
2. Yes -- when writing the football file, I copied the structure from the weather
   file and only changed a few lines and variables names.
3. As explained before, it might be best not to preemptively factor out common code
   until you can show that two functions share common functionality. This is because
   abstracting early can cause problems when the function interface (e.g. for `get_min_spread`)
   changes. That said, the readability of the weather and football files becomes
   easier when you can hide complexity behind a library file, which likely comes
   at a cost for maintainability.

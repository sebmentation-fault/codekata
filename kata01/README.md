# Kata01: Supermarket Pricing

> See [Kata 01: Supermarket Pricing](http://codekata.com/kata/kata01-supermarket-pricing/)

This kata arose from some discussions we’ve been having at the DFW Practioners meetings.
The problem domain is something seemingly simple: pricing goods at supermarkets.

Some things in supermarkets have simple prices: this can of beans costs $0.65. Other
things have more complex prices. For example:

* three for a dollar (so what’s the price if I buy 4, or 5?)
* $1.99/pound (so what does 4 ounces cost?)
* buy two, get one free (so does the third item have a price?)

This kata involves no coding. The exercise is to experiment with various models
for representing money and prices that are flexible enough to deal with these (and
other) pricing schemes, and at the same time are generally usable (at the checkout,
for stock management, order entry, and so on). Spend time considering issues such
as:

* does fractional money exist?
* when (if ever) does rounding take place?
* how do you keep an audit trail of pricing decisions (and do you need to)?
* are costs and prices the same class of thing?
* if a shelf of 100 cans is priced using “buy two, get one free”, how do you value
  the stock?

This is an ideal shower-time kata, but be careful. Some of the problems are more
subtle than they first appear. I suggest that it might take a couple of weeks
worth of showers to exhaust the main alternatives.

## Goal

The goal of this kata is to practice a looser style of experimental modelling.
Look for as many different ways of handling the issues as possible. Consider the
various tradeoffs of each. What techniques are best for exploring these models?
For recording them? How can you validate a model is reasonable?

## Solution/Working-out

Storing simple price:

* could use a float -- might getting rounding errors from floating point addition
  being not exact in some cases as it may round down/up unexpectedly (the fact that
  this the differnce will be so minimal could be excluded for small shops)
* otherwise, create a prices class such that any time the cents/pennies exceeds
  99, it adds 1 to the dollar/pound, albeit adding to the codebase, items to
  maintain, increases complexity when currency evolves (e.g. how to handle if the
  shop wants to accept cryptocurrency or another currency -- create a new class?)
  and will likely slow the process down slightly due to more operations needed.

Dealing with complex prices:

* could armotize the deal across all items (e.g. 3 for 1 dollar would become 1 for
  0.33 only when 3 are bought simultaneously) but then we would lose accounting
  of where the final cent/penny was from.
* alternitively could create a new object/item that is called "3 [item]", and when
  the cashier scans the third, the previous 2 items are deleted and a 3-item one
  is added to the list. This has the added benefit of maintaining a list of where
  deals were used in databases/reciepts.
* for buying 2, get one free, it makes the most sense to simply add a the third
  item but with a listing that shows it was given for free.

Fractional money should arguably not exist. It complicates things from an accounting
perspective -- the total value of the prices might not equal the total that gets
charged by the cashier at the till.

We want to minimise the chance of rounding from floating point arithmetic.

Cost and prices should be treated as different things.

Valuing some stock of 100 cans priced at one dollar/pound with a buy two get one
free deal should be valued as two thirds the value of the stock without the deal.
We know that only 99 cans can be sold with the deal, and there will be one left
dangling, but seeing as inventory is volatile (some people do not want to buy
more than one, some might break in transit, more may arrive before they run out),
it is impracticle to value it differently.

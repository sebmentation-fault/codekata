# Kata03: How Big? How Fast?

> See [Kata 03: How Big? How Fast?](http://codekata.com/kata/kata03-how-big-how-fast/)

Rough estimation is a useful talent to possess. As you’re coding away, you may suddenly
need to work out approximately how big a data structure will be, or how fast some
loop will run. The faster you can do this, the less the coding flow will be disturbed.

So this is a simple kata: a series of questions, each asking for a rough answer.
Try to work each out in your head. I’ll post my answers (and how I got them) in
a week or so.

## How Big?

* roughly how many binary digits (bit) are required for the unsigned representation
of:
  * 1,000
  * 1,000,000
  * 1,000,000,000
  * 1,000,000,000,000
  * 8,000,000,000,000
* My town has approximately 20,000 residences. How much space is required to store
    the names, addresses, and a phone number for all of these (if we store them
    as characters)?
* I’m storing 1,000,000 integers in a binary tree. Roughly how many nodes and levels
  can I expect the tree to have? Roughly how much space will it occupy on a 32-bit
  architecture?

### Solutions/Working-out 1

1. For every thousand we go up, we only need 10 more bits:
    1. 1,000 => can be represented in log_2(1,000) bits => around **10** because
       we need to round up to the next whole bit
    2. 1,000,000 => can be represented in log_2(1,000,000) => **20** bits
    3. 1,000,000,000 => can be represented in log_2(1,000,000,000) => **30** bits
    4. 8,000,000,000 => can be represented in log_2(8,000,000,000) => **33** bits
2. We will assume the following:
    * we need 32 characters/bytes for the name (16 for first and last name respectively)
    * the address is within a town, so we do not need to store county/country/etc
      as this is the same for all addresses. Taking house/flat number (which could
      use letters as well), as well as road and postcode, we can assume the address
      might also need 32 bytes.
    * phone numbers might need up to 14 characters/bytes to be represented -- they
      need to be stored as strings as some residents might have phone numbers from
      different country codes and therefore the '+XX' needs space.
    * this means roughly 78 bytes per resident. If we assume that the town hall
      wants information such as age, then we could round this to 80 bytes.
    * in total, for 20,000 residents, we will need at least 160,000 bytes, or
      160 kilobytes
3. Answer:
    * The tree will have as many nodes as elements to store - so 1,000,000. Assuming
      that the tree is balanced (or as close to balanced as it could be), the tree
      will have a depth of log_2(1,000,000) which is just under 10 (where the 9th
      row is almost completely filled).
    * On a 32-bit machine, the left and right pointers will be 32 bits, as will
      the value on the node. 32 times 1,000,000 is 32,000,000 bits or 4,000,000
      bytes or 4 gigabytes.

## How Fast?

My copy of Meyer’s Object Oriented Software Construction has about 1,200 body pages.
Assuming no flow control or protocol overhead, about how long would it take to send
it over an async 56k baud modem line?

My binary search algorithm takes about 4.5mS to search a 10,000 entry array, and
about 6mS to search 100,000 elements. How long would I expect it to take to search
10,000,000 elements (assuming I have sufficient memory to prevent paging).

Unix passwords are stored using a one-way hash function: the original string is
converted to the ‘encrypted’ password string, which cannot be converted back to
the original string. One way to attack the password file is to generate all possible
cleartext passwords, applying the password hash to each in turn and checking to
see if the result matches the password you’re trying to crack. If the hashes match,
then the string you used to generate the hash is the original password (or at least,
it’s as good as the original password as far as logging in is concerned). In our
particular system, passwords can be up to 16 characters long, and there are 96 possible
characters at each position. If it takes 1mS to generate the password hash, is this
a viable approach to attacking a password?

### Solutions/Working-out 2

1. Assuming each page has 50 lines, and each line has space for 80 characters, we
   could be dealing with 4,000 characters, with each character represented as a
   single ASCII byte. Over all the pages, we are dealing with 4,800,000 characters/bytes.
   Assuming that a 56k baud modem line can tramsmit 56 kilobits a second, then we
   could consider the line to have a transmission speed of 7,000 characters/bytes
   every second. Hence, the book would take 685.7142857143 seconds or just under
   12 minutes.
2. Binary search is logarithmic in nature -- each marginal increase in depth takes
   respectively 'less' time to solve. We can assume that for every 10 entries, it
   takes the algorithm 1mS, hence, by the time we get to 10,000,000 entries, it
   will take the algorithm 7mS to search for an element.
3. I know this!! The list of all the generated cleartext passwords paired with their
   encrypted ciphertext is called a rainbow table. If we have 16 characters, and
   there are 96 combinations for each character (and assuming characters have to
   have at least 8 characters), we have `(8 * 96) + (9 * 96) + ... + (16 * 96)`
   or 96 * (8 + ... + 16). Sum of all the numbers between 8 and 16 is
   `(9)/2 * (8 + 16)` or 108, and then multiplied by 96 gives 10,368 different combinations.
   That means it may take 10,368 mS to create the rainbow table, which is just
   over 10 seconds. The password look up afterwards (perhaps using the binary search
   algorithm) means the whole password cracking process could be complete within
   11 seconds.

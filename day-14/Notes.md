# Chinese Remainder Theorem

We can search by sieving

https://en.wikipedia.org/wiki/Chinese_remainder_theorem#Search_by_sieving

We want...

n == 0 mod 7
n == 1 mod 13
n == 4 mod 59
n == 6 mod 31
n == 7 mod 19

Order by biggest

59
31
19
13

4 mod 31
4 + 59 mod 31
4 + 118 mod 31, etc until mod == 6

result + (59 * 31) mod 19, until mod == 7

result + (59 * 31 * 19) etc
find the lowest by subtracting out the result
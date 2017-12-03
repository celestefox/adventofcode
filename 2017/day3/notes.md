17  16  15  14  13
18   5   4   3  12
19   6   1   2  11
20   7   8   9  10
21  22  23---> ...

4 3 2 3 4
3 2 1 2 3
2 1 0 1 2
3 2 1 2 3
4 3 2 3 4

etc...
is there a mathematical pattern?

01 02 03 04 05 06 07 08 09 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 ...
00 01 02 01 02 01 02 01 02 03 02 03 04 03 02 03 04 03 02 03 04 03 02 03 04 05 04 03...

Hrrm. There is an obvious pattern there.
It's simple but hard to describe

00
01 02 x4
03 02 03 04 x4
05 04 03 04 05 06 x4 ...

pattern matching feels like a useful tool here, but how?

have to keep in mind I don't know part 2
though if I have to rewrite for it there's not much of a way around it...

... Okay. Thankfully I had it pointed out somewhere that there's really obvious perfect squares there. I don't know if I want to do that or try the ways I was thinking before.

1   9   25  49  ...
1^2 3^2 5^2 7^2

So the odd numbers squared are the bottom left corners... and you can calculate the distance easier with them?

So, find the closest one... but what if, say, you have 17, which is opposite 25? how is knowing the square useful?

Okay. So, each corner is from the list of odd numbers. if you have corner n, the number is n^2 and the distance is n-1.
The sides of the square of n are... n. And you start in an obvious place. Hmm.

############################################

each corner has a "index", "cindex", and "number", as I'm calling them. Everything has an "index" and "number". The "number" is what the problem gives - the number written on a cell. the "index" is the distance to it - what we are trying to solve. These corners are special because there is a number, "cindex", such that cindex-1 is index and cindex^2 is number. These are the squares of odd numbers - the center is cindex 1, the first bottom right corner is cindex 3, the next is 5, etc.

We are trying to find the index of some number, which I will call `n`. First, we find the cindex of the "closest" (highest number that is still less than `n`) corner. To do this, `floor(sqrt(i))`, and `dec` if `even?`. Call this `ci`.

Next, we want the "distance" around the spiral from that corner. We will find that with `n-ci^2` and call that `dist`.

Then, we want the "side position" of `n` - "where" it is on the side. This is `pos` = `(mod dist (inc ci))`. This is because the corner is the last number of each square. Corner cindex 3 continues on at 10 to the left one more, then spiraling around more, etc. The sides of the square our corner is on are `ci-1`, but the sides of the next are `ci+2-1`. (I'm specifically talking about how I add that many squares to get to what is the next corner here. In the next part I'm talking about the total dimension, one more).

So. We know we are position `pos` on the side of the square with sides `ci+2` overall. That is 0 indexed because mod. The center of each side is `(ci+1)/2` index. Add `abs(pos-(ci+1)/2)` to that, because that's how much further we are from the center. 

A concrete example. `n=20`. `ci=3`. `dist=11`. `side=2`. `pos=3`. `(ci+1)/2=2`. `abs(pos-prev)=1`. And checking with the table above, `2+1`, our answer, is indeed right!



stuff you need to know to implement bsp trees
by husky

contact: nikocs@voremicrocomputers.com


# prerequisites for reading this text file

you should know basic mathematic operations, such as addition, subtraction,
multiplication, and division.

you don't necessarily need to know how to calculate these manually, you
just need to know what they do.


# vectors

a vector is a group of numbers that represents a point in space.

(technically this is called a "point" in mathematics and a "vector"
is something else, but here we will use them interchangably)

vectors can contain any amount of numbers, and the amount of numbers they
contain corresponds to the dimension they represent.


the most common vector you will see is a three-dimensional vector,
which i will refer to as a "vector3".

this will be written here as [x, y, z]

if you don't fully understand vectors, here's a quick way you can
play around with them:

download blender, add a cube to the scene, and play around with the
position of the cube. the position is a vector.


notably, vectors can represent a point in any dimension,
a two-dimensional vector, or a "vector2", would look like this:
[x, y]

if you make sure that the cube in blender always stays at a z-value of
0, then it will act the same as if it were in a two-dimensional space.


you can also have a one-dimensional vector, or a vector1. in practice
this wouldn't really be used as it's no different from just a single number,
but it'll be useful to think of in a later section.


# vector magnitude / length

if you ever learned the pythagorean theorem in school, it will come in handy
now.

i'll give a refresher though.

if we have a right-triangle, i.e. a triangle where one of the angles within
it is 90 degrees, then we can use the following equation to take the width
and the height of the triangle's straight sides, and get the length of the
angled side.

a^2 + b^2 = c^2

where a = width
and b = height
and c = angled side length

(you can swap the order of a and b without anything changing)


there are two important things we can use this for.

first of all, if we have a vector2 describing the point
[2, 4] (two on the x-axis, four on the y-axis)

then we could imagine that the point represents the top of a right-triangle
wherein the width of the triangle is its x-axis distance from 0
and the height of the triangle is its y-axis distance from 0

this means that the width of the triangle is 2, and the height is 4; notice
how those values are the numbers in the vector

if we plug those numbers into the pythagorean theorem, like

2^2 + 4^2 = c^2
which is the same as
4 + 16 = c^2
which is the same as
20 = c^2
which is the same as
sqrt(20) = c
which is the same as
4.47... = c

we get the length of the angled side of the triangle, which is also the
exact distance of the point from [0, 0]

this means that we can use the pythagorean theorem to calculate the distance
of a vector2 from [0, 0]

the distance of a vector from [0, 0] is known as its "length", or "magnitude".


now the other main thing to note about this is that we can apply this
technique to every dimension.

if we rename the variables in the pythagorean theorem to be more specific to
our specific case, it would look like this:

x^2 + y^2 = magnitude^2

and all we need to do to apply this to other dimensions is to add or remove
the axis

to apply this to the third dimension with a vector3, it would look like this:

x^2 + y^2 + z^2 = magnitude^2

and to apply this to the pretty much useless vector1, it would look like this:

x^2 = magnitude^2



# vector normalization

a normalized vector is a vector where its magnitude is always exactly 1

this could be a vector3 like [1, 0, 0]
or a vector3 like [0, 1, 0]
or even a vector3 like [0.43643576, 0.8728715, 0.21821788] (approximation)

normalized vectors are used to represent directions, and you'll realize in
the next section why they're so useful, however for now i'll just tell you
how to make any vector become a normalized vector.

if we take the vector3 [2, 2, 1] and get its magnitude by doing the following:

x^2 + y^2 + z^2 = magnitude^2

2^2 + 2^2 + 1^2 = magnitude^2

4 + 4 + 1 = magnitude^2

9 = magnitude^2

sqrt(9) = magnitude

magnitude = 3

and then, we divide each of the components of the vector by its magnitude

[2/3, 2/3, 1/3] = [0.66, 0.66, 0.33] (approximation)

this is a valid normal vector (i.e. a vector that is normalized)

do note that due to the usage of division, it is not possible to normalize a
vector whose magnitude is 0, and the only vector whose magnitude is 0 is a
vector where all of its components are 0.

this means that while you can normalize a vector3 such as [1, 1, 0], you
cannot normalize the vector3 [0, 0, 0]

you also cannot normalize the vector2 [0, 0] or the vector1 [0]

if you try this in a programming language using floating point values, you
will either get a crash or have a floating point NaN value, which you should
avoid.

this makes sense as there is no direction in a vector without any components


# vector dot products

i think the best way to learn how a dot product works is to first understand
what it can do.

a dot product takes two vectors, where one of the vectors is usually a
normalized vector, and returns:

the distance of a point in the direction of the given normal.
(notably, the order in which you provide the point and the normal in the order
of operations does not matter)

a good way to visualize this is the following,

if we have the normal vector3 [1, 0, 0] and the point vector3 [2, 1, 3],
the dot product would be 2.

this is because the normal vector points exactly toward 1 on the x axis, and 0
everywhere else. so our normal points directly towards positive x.
the point [2, 1, 3] is 2 on the positive x, so that's its distance in that
direction!

if we take the same point of [2, 1, 3], but this time we dot it with normal
vector [0, -1, 0], we get a dot product of -1

using relative terms wherein y represents up/down, this means that our normal
vector points directly downwards, and since our point is pointing upwards from
[0, 0, 0], the distance is negative because it is "behind" [0, 0, 0] when the
normal faces downwards.


this is where our "useless" vector1 will come in handy for an explanation.

first of all, there are only two possible normalized vector1s:
[1]
and
[-1]

this is because any number divided by absolute itself will be either 1 or
negative 1.
(x^2 = c^2 will always result in a positive number,
regardless of the sign of x)

now because there are only two possible directions, how do you think we would
figure out the distance of a point on this one-dimensional line from the
origin (or rather, the vector1 [0]) given a vector1 direction?

we simply multiply our point vector1 by the normal vector1.

if we have the normal vector1 [1] and the point vector1 [70], the dot product
is 70

if we have the normal vector1 [-1] and the point vector1 [70], the dot product
is -70

this isn't very useful in one-dimensional space though, so how do we expand
this to other dimensions?


we just simply do this for each component.


if we go to the second dimension, with a normal vector2 of [1, 0] and a point
vector2 of [10, 30], to get the dot product we do the following

(1 * 10) + (0 * 30) = 10

specifically, the equation we use is:

(x1 * x2) + (y1 * y2) = dot product in two-dimensional space


this works in every dimension, so in 3d space its defined as

(x1 * x2) + (y1 * y2) + (z1 * z2) = dot product in three-dimensional space


and you can even do this in four-dimensional space!

(x1 * x2) + (y1 * y2) + (z1 * z2) + (w1 * w2) = 4d dot product

(this won't be needed here, it's just interesting!)


now you may be wondering, what happens if both vectors in the dot product
aren't normal vectors?

the answer is very simple, the result will just be scaled by each of the
un-normalized components!


this is a bit hard to visualize, but luckily there's really only one use
for it that you'll necessarily want to use

if we had the vector2 [20, 10] and called it A,
and then got the dot of A with A

what do you think would happen?

let's calculate it,

the 2d dot product is the following:
(x1 * x2) + (y1 * y2) = 2d dot product

so if we plug in our vector2 of [20, 10] into both slots...
(20 * 20) + (10 * 10)
which is the same as
400 + 100
which is
500

now if we first imagine what this would equate to if one of these was a
normal vector, it would be the normal vector pointing in the exact direction
of the vector2 of [20, 10]. the dot product would then just be the magnitude
of the vector!

but now think about the fact that we're scaling it by itself, this
means that if we think about the output, the magnitude is also scaled by
itself.

another word for "scaled by itself" is "multiplied by itself", or "squared".

which means that the dot product of something with itself is the
square magnitude of the vector.

which means that the square root of the dot product of something with itself
is just the magnitude of the vector!

sqrt(500) == 22.36 (approx.)
magnitude of [20, 10] == 22.36 (approx.)

which can be rewritten as
sqrt(dot(A, A)) = magnitude of A

if we expand the dot product into its full form in 2d space...

sqrt((Ax * Ax) + (Ay * Ay)) = magnitude of A

you may notice something...


this is just the pythagorean theorem, but written in a different form!

(Ax * Ax) can be rewritten as Ax^2
and
(Ay * Ay) can be rewritten as Ay^2

which means that this is just
sqrt(Ax^2 + Ay^2) = magnitude of A

and if we move the sqrt to the other side...
Ax^2 + Ay^2 = (magnitude of A)^2

this is literally just the pythagorean theorem!

so there's my very very loose proof for why dot(A, A) == magnitude of A


theoretically this is all the core math concepts you need to know to implement
a bsp tree, but i'll add onto this file with more helpful stuff eventually

i also want to provide an actual overview of the bsp tree creation algorithm,
as well as how bsp trees work, but this took long enough to write and
hopefully will get your brain excited enough to be able to study more :D

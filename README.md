## Problem

If you choose 4 points on a sphere and consider the tetrahedron with these points as it's vertices,
what is the probability that the center of that sphere is inside of that tetrahedron?
(Each point is independently chosen relative to a uniform distribution on the sphere.)

## Bouquets
<<<<<<< HEAD
This project was inspired by the video [The hardest question on the hardest test](https://www.youtube.com/watch?v=OkmNXy7er84) by 3Blue1Brown:
=======
This project was inspired by this video by 3Blue1Brown:
>>>>>>> a45e7124b969fbeada19bf6c5b49eab941e9fc0b

<a href="http://www.youtube.com/watch?feature=player_embedded&v=OkmNXy7er84
" target="_blank"><img src="http://img.youtube.com/vi/OkmNXy7er84/0.jpg" 
alt="The hardest problem on the hardest test" width="480" height="360" border="10" /></a>

<<<<<<< HEAD

## Description

The proof discussed in the video is very elegant, but I wanted to see how it worked in practice.
=======
https://www.youtube.com/watch?v=OkmNXy7er84

## Description

The proof discussed in the video is very elegant, but I wanted to see how it worked in theory.
>>>>>>> a45e7124b969fbeada19bf6c5b49eab941e9fc0b

For the random distribution, I create a starting point P(0 | 0 | 1), then rotate it by a random amount around the x-, y- and z-axis for a uniform distribution.

To test whether a point is inside or outside of the tetrahedron, I convert it's vertices into a barycentric coordinate system.
For more information, check out [this](https://en.wikipedia.org/wiki/Barycentric_coordinate_system) Wikipedia article.

## Examples
You can find more examples in `/examples` or generate your own by running `main.rs`.

![alt text](https://github.com/eliavaux/putnam-1992/blob/main/examples/example-1.gif)
![alt text](https://github.com/eliavaux/putnam-1992/blob/main/examples/example-2.gif)

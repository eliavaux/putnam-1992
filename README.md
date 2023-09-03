# putnam-1992
Putting the 1992 Putnam Test question A-6 into code.

## Credit
This project was inspired by this video by 3Blue1Brown:

<a href="http://www.youtube.com/watch?feature=player_embedded&v=OkmNXy7er84
" target="_blank"><img src="http://img.youtube.com/vi/OkmNXy7er84/0.jpg" 
alt="The hardest problem on the hardest test" width="480" height="360" border="10" /></a>

## Problem

If you chose 4 points on a sphere and consider the tetrahedron with these points as it's vertices,
what is the probability that the center of that sphere is inside of that tetrahedron?
(Each point is independently chosen relative to a uniform distribution on the sphere.)

## Description

The proof discussed in the video is very elegant, but I wanted to know whether it was actually true or not.

For the random distribution, I took a starting point P(0 | 0 | 1), then rotated it by a random amount around the x-, y- and z-axis for a uniform distribution.

To test whether a point is inside or outside of the tetrahedron, I converted it's vertices into a barycentric coordinate system.
For more information, check out [this](https://en.wikipedia.org/wiki/Barycentric_coordinate_system) Wikipedia article.

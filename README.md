## Question
If you choose 4 points on a sphere and consider the tetrahedron with these points as it's vertices,
what is the probability that the center of that sphere is inside of that tetrahedron?
(Each point is independently chosen relative to a uniform distribution on the sphere.)


## Thank You
3Blue1Brown for his video [_The hardest question on the hardest test_](https://www.youtube.com/watch?v=OkmNXy7er84) for inspiring me to create this project.


## Description
For a uniform distribution of the points, a starting point P(0 | 0 | 1) is created, then rotated by a random degree around the x-, y- and z-axis.

To test whether a point is inside or outside of the tetrahedron, it's vertices are converted into a barycentric coordinate system. For more
information, check out the [Wikipedia article](https://en.wikipedia.org/wiki/Barycentric_coordinate_system) on the topic.

I have added a plot function the the code to help visualize the problem as well as see if my code was working correctly.


## Thoughts
While the solution to this problem by 3Blue1Brown is very elegant, I wanted to implement a heuristic approach to the problem. This project
was more of a practice for my knowledge of linear algebra than meant as a serious analysis of the question.


## Examples
You can find more examples in the `examples/` directory or generate your own by running the `src/main.rs` file.

![example-1](https://github.com/eliavaux/putnam-1992/blob/main/examples/example-1.gif)

Center of sphere outside of the tetrahedron

![example-2](https://github.com/eliavaux/putnam-1992/blob/main/examples/example-2.gif)

Center of sphere inside of the tetrahedron

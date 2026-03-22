# Applied Computer Graphics 4860-1084

[![CI_Linux](https://github.com/ACG-2024S/acg/actions/workflows/ubuntu.yml/badge.svg)](https://github.com/ACG-2024S/acg/actions/workflows/ubuntu.yml)
[![CI_Win](https://github.com/ACG-2024S/acg/actions/workflows/windows.yml/badge.svg)](https://github.com/ACG-2024S/acg/actions/workflows/windows.yml)

![teaser](doc/rep_image.png)

Lecture at graduate school of information science and technology in the university of Tokyo, spring semester, 2026

UTOL (UTokyo-LMS) (for Zoom URL, Slack and GitHub Classroom invitations): 

- https://utol.ecc.u-tokyo.ac.jp/lms/course?idnumber=2026_4886_4860-1084_01


## Instructors

Dr. Nobuyuki Umetani 
- email: n.umetani@gmail.com
- url: http://www.nobuyuki-umetani.com/
- lab's web page: https://cgenglab.github.io/labpage/en/


Sei Imai (Lecture on May. 18th about game development)
- CEO of I. Meisters(https://i-meisters.com/)


Dr. Anran Qi (Guest lecturer for Apr.27th, parametric curve)
- Project assistant professor at Takeo Igarashi's lab
- https://anranqi.github.io/



## Time

Monday 2nd period, 10:25am - 12:10pm


## Course Description

Computer graphics is a technology to computationally represent objects' geometry, appearance and movement. This course is an introduction to the techniques generally seen in computer graphics. The aim of the course is to get familiar with applied mathematics such as linear algebra, vector analysis, partial differential equations, numerical analysis and optimization through the topics in computer graphics. There are C++ programming assignments to acquire research-oriented graphics programming skills such as OpenGL, shader programming, Eigen matrix library, Git and cmake. 

Topics:
- affine transformation & homography
- visualization (rasterization / ray casting)
- continuous optimization
- parametric curves & surfaces
- variational mesh deformation


## Lecture Schedule

| Day | Topic | Assignment | Slide |
|:----|:---|:---|:---|
|(1)<br>Apr. 20| **Introduction**<br>Digital image, Rasterization in 2D |  | |
|(2)<br>Apr. 27| **Parametric curve/surface** <br/>Bézier curve, Splines, polynomial |  |  |
|(3)<br>May. 7| **Coordinate transformation**<br>Affine, Homography transformation |  |  |
|(4)<br>May 11| **Rasterization1**<br>Graphics pipeline, Depth buffer method, Simple-shading |  |  |
|(5)<br>May 18| **Guest Lecture by Sei Imai (I.Meisters)**<br> Game development |  |   |
|(6)<br>May 22| **Spatial data structure**<br> Bounding volume hierarchy, Kd-tree, Octree  |  |  |
|(7)<br>May 25| **Volume Representation**<br>Implicit modeling, Ray marching, Volume rendering |  |  |
|(8)<br>Jun. 8| **Rasterization2**<br> Monte-Carlo integration for anti-aliasing, Gaussian splatting |  |  |
|(9)<br>Jun. 15| **Ray Casting1**<br>Material model, Rendering equation |  | |
|(10)<br>Jun. 22| **Ray Casting2**<br> Advanced Monte Carlo integration |  |  |
|(11)<br>June 29| **Optimization**<br> Gradient descent, Back propagation, Newton's method | | |
|(12)<br>July 6| **Laplacian mesh deformation**<br> Sparse linear system |  |  |
|(13)<br>July 13| **Differentiable Rendering**<br>Reynolds transport theorem, Edge sampling, Nvdiffrast | |  |


## Grading

- 20% lecture attendance
  - Attendance is counted based on writing a secret keyword on LMS. The keyword is announced for each lecture.  
- 80% small assignments
  - see below

## Assignments

There are many small programming assignments. 
To do the assignments, you need to create your own copy of this repository through **GitHub Classroom**.  
These assignments need to be submitted using **pull request** functionality of the GitHub. 
Look at the following document. 

[How to Submit the Assignments](doc/submit.md)

| Task ID | Title | Thumbnail |
|:---|:---|:---|
| [task01](task01) | **Rasterization of lines and polygons**<br>DDA, winding number | <img src="task01/preview.png" width=100px> |
| [task02](task02) | **Rasterization of parametric curves**<br> Quadratic Bézier curve, root of polynominal | <img src="task02/preview.png" width=100px> |
| [task03](task03) | **Perspectively-correct texture mapping**<br>rasterization of triangle, barycentric coordinate | <img src="task03/preview.png" width=100px> |
| [task04](task04) | **Vertex shader practice** <br>Rendering pipeline, mirror reflection, OpenGL | <img src="task04/preview.png" width=100px> |
| [task05](task05) | **Fragment shader practice**<br>Ray marching method, CSG modeling, implicit modeling | <img src="task05/preview.png" width=100px> |
| [task06](task06) | **Monte Carlo integration1**<br>Ambient occlusion, importance sampling, BVH | <img src="task06/preview.png" width=100px> |
| [task07](task07) | **Monte Carlo integration2**<br/>Multiple importance sampling | <img src="task07/preview.png" width=100px> |
| [task08](task08) | **Skeletal Character Animation**<br>Linear blend skinning, articulated rigid body | <img src="task08/preview.png" width=100px> |
| [task09](task09) | **Laplacian Mesh Deformation**<br> Quadratic programming, sparse linear system | <img src="task09/preview.png" width=100px> |

### Policy

- Do the assignment by yourself. Do not share the answers of the assignments.
- Late submission of an assignment is subject to grade deduction.
- Score each assignment will not be open soon (instructor needs to adjust weight of the score later).
- The assignments might not be graded soon.


## Slides

- [[1] Introduction](http://nobuyuki-umetani.com/acg2024s/introduction.pdf)

- [[2] C++ programming](http://nobuyuki-umetani.com/acg2024s/cpp.pdf)

- [[3] Git+GitHub](http://nobuyuki-umetani.com/acg2024s/git.pdf)

- [[4] Digital Image](http://nobuyuki-umetani.com/acg2024s/digital_image.pdf)

- [[5] Rasterization in 2D](http://nobuyuki-umetani.com/acg2024s/rasterization_2d.pdf)

- [[6] Barycentric Coordinates](http://nobuyuki-umetani.com/acg2024s/barycentric_coordinates.pdf)

- [[7] Parametric Curve](http://nobuyuki-umetani.com/acg2024s/parametric_curve.pdf)

- [[8] Polynominal Root finding](http://nobuyuki-umetani.com/acg2024s/polynominal.pdf)

- [[9] Coordinate Transformation](http://nobuyuki-umetani.com/acg2024s/transformation.pdf)

- [[10] 2D Homogeneous Transformation](http://nobuyuki-umetani.com/acg2024s/transformation_homogeneous_2d.pdf)

- [[11] 3D Homogeneous Transformation](http://nobuyuki-umetani.com/acg2024s/transformation_homogeneous_3d.pdf)

- [[12] 3D Rasterization](http://nobuyuki-umetani.com/acg2024s/rasterization_3d.pdf)

- [[13] Graphics Pipeline](http://nobuyuki-umetani.com/acg2024s/graphics_pipeline.pdf)

- [[14] Shading](http://nobuyuki-umetani.com/acg2024s/shading.pdf)

- [[15] Subpixel Effect](http://nobuyuki-umetani.com/acg2024s/rasterization_subpixel.pdf)

- [[16] Implicit Modeling](http://nobuyuki-umetani.com/acg2024s/implicit_modeling.pdf)

- [[17]Ray Casting](http://nobuyuki-umetani.com/acg2024s/ray_casting.pdf)

- [[18]Monte Carlo Integration](http://nobuyuki-umetani.com/acg2024s/monte_carlo_integration.pdf)

- [[19]Ray Triangle Collision](http://nobuyuki-umetani.com/acg2024s/ray_triangle_collision.pdf)
- [[20]Character deformation](http://nobuyuki-umetani.com/acg2024s/character_deformation.pdf)
- [[21] Jacobian&Hessian](http://nobuyuki-umetani.com/acg2024s/jacobian.pdf)
- [[22] Optimization](http://nobuyuki-umetani.com/acg2024s/optimization.pdf)
- [[23] Mesh laplacian](http://nobuyuki-umetani.com/acg2024s/mesh_laplacian.pdf)
- [[24] Sparse linear system](http://nobuyuki-umetani.com/acg2024s/sparse_linear_system.pdf)
- [[25] Fluid simulation](http://nobuyuki-umetani.com/acg2024s/fluid_simulation.pdf)



## Reading Material
- Introduction to Computer Graphics by Cem Yuksel](https://www.youtube.com/watch?v=vLSphLtKQ0o&list=PLplnkTzzqsZTfYh4UbhLGpI5kGd5oW_Hh)
- [Scratchpixel 2.0](https://www.scratchapixel.com/)
- [Awesome Computer Graphics (GitHub)](https://github.com/luisnts/awesome-computer-graphics)
- [Skinning: Real-time Shape Deformation SIGGRAPH 2014 Course](https://skinning.org/)
- [Physics-based Animation2023S (Another course by the instructor) ](https://github.com/nobuyuki83/Physics-based_Animation_2023S)
- [Physics-based Animation2021S (Another course by the instructor) ](https://github.com/nobuyuki83/Physics-based_Animation_2021S)
- [Applied Computer Graphics 2022S (The same course two years ago)](https://github.com/nobuyuki83/Applied_Computer_Graphics_2022S)


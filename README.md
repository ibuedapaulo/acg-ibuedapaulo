# Applied Computer Graphics 4860-1084

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

Computer graphics is a technology to computationally represent objects' geometry, appearance and movement. This course is an introduction to the techniques generally seen in computer graphics. The aim of the course is to get familiar with applied mathematics such as linear algebra, vector analysis, partial differential equations, numerical analysis and optimization through the topics in computer graphics. There are Rust/Unity programming assignments to acquire research-oriented graphics programming skills such as shader programming and Git. 

Topics:
- Visualization of lines/triangles using rasterization/ray casting
- Parametric curves/surfaces
- Affine/homography transformation
- Volume representation and visualization
- Monte Carlo integration
- Continuous optimization (gradient/hessian based)
- Variational mesh deformation



## Lecture Schedule

| Day | Topic | 
|:----|:---|
|(1)<br>Apr. 20| **Introduction**<br>Introduction [1] Digital image[2], Rasterization in 2D (Brezenham's algorithm)[3] |  
|(2)<br>Apr. 27| **Parametric curve/surface** <br/>Polynomial root finding [4], Parametric Curve (Bézier curves/B-Splines)[5], Git [6] |  
|(3)<br>May. 7| **Coordinate transformation**<br> 2D transformation (Homography transformation) [7], 3D transformation (Pinhole camera model) [8] |  
|(4)<br>May 11| **Rasterization1**<br>Rasterization in 3D (Depth buffer method) [9], Graphics pipeline [10], Shading [11] | 
|(5)<br>May 18| **Guest Lecture by Sei Imai (I.Meisters)**<br> Game development | 
|(6)<br>May 22| **Spatial data structure**<br> Ray casting [12], Jagged array [13], Spatial data structure (BVH, Kd-tree, Octree) [14]  | 
|(7)<br>May 25| **Volume Representation**<br>Implicit modeling [15], Volume visualization (ray marching, volume rendering)[16] | 
|(8)<br>Jun. 8| **Rasterization2**<br> Anti-aliasing and transparency( Monte-carlo integration, Gaussian splatting) [17] | 
|(9)<br>Jun. 15| **Ray Casting1**<br> Material modeling (Rendering equation) [18], Jacobian [19] | 
|(10)<br>Jun. 22| **Ray Casting2**<br> Path tracing (Importance sampling)[20] |
|(11)<br>June 29| **Optimization**<br> Optimization (Gradient descent, Back propagation, Newton's method) [21] | 
|(12)<br>July 6| **Laplacian mesh deformation**<br> Sparse linear system [22], Mesh laplacian [23] | 
|(13)<br>July 13| **Differentiable Rendering**<br> Differential rendering (Reynolds transport theorem, Edge sampling, Nvdiffrast) [24] | 


## Slides

*A link to the slides will be added to the title right before each class*

- [1] Introduction
- [2] Digital image
- [3] Rasterization in 2D
- [4] Polynomial root finding
- [5] Parametric curve
- [6] Git
- [7] Coordinate transformation in 2D
- [8] Coordinate transformation in 3D
- [9] Rasterization in 3D
- [10] Graphics pipeline
- [11] Shading
- [12] Ray casting
- [13] Jagged array
- [14] Spatial data structure
- [15] Implicit modeling
- [16] Volume visualization
- [17] Antialiasing & transparency
- [18] Material modeling
- [19] Jacobian
- [20] Path tracing
- [21] Optimization
- [22] Sparse linear system
- [23] Mesh Laplacian
- [24] Differential Rendering
- [25] Rust language
- [26] Unity

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

**Below is a tentative schedule. Each assignment will be open and in a class one-by-one**

| Task ID | Title | Thumbnail | Assigned | Due |
|:---|:---|:---|:---|:---|
| task01 | **Rasterization of lines and polygons**<br>Winding number |  |Apr. 27| May 1|
| task02 | **Rasterization of parametric curves**<br> Parametric curve, Polynomial root finding | |May 7| May 11|
| task03 | **Perspectively-correct texture mapping**<br>rasterization of triangle,Barycentric coordinates |  |May 11| May 15|
| task04 | **Shader practice1**<br>Unity, Rendering pipeline |  |May 22| May 26|
| task05 | **Acceleration of geometry computing**<br> Bounding-volume hierarchy |  |May 25| May 29|
| task06 | **Shader practice2**<br>Unity, Ray marching method, CSG modeling, implicit modeling | |Jun. 8| Jun. 12 |
| task07 | **Gaussian splatting**<br>Rasterization of Gaussian primitives, Tile-based acceleration, Alpha-blending | |Jun. 15| Jun. 19|
| task08 | **Monte Carlo integration**<br/>Importance sampling |  |Jun. 29| Jul. 3|
| task09 | **Laplacian Mesh Deformation**<br> Quadratic programming, Sparse linear system | |Jul. 6| Jul. 10|

### Policy

- Do the assignment by yourself. Do not share the answers of the assignments.
- Late submission of an assignment is subject to grade deduction.
- Score each assignment will not be open soon (instructor needs to adjust weight of the score later).
- The assignments might not be graded soon.





## Reading Material

### General Computer Graphics

- 🎥 [Introduction to Computer Graphics by Cem Yuksel](https://www.youtube.com/watch?v=vLSphLtKQ0o&list=PLplnkTzzqsZTfYh4UbhLGpI5kGd5oW_Hh)
- 🌐 [Awesome Computer Graphics (GitHub)](https://github.com/luisnts/awesome-computer-graphics)
- 📖 [Fundamentals of Computer Graphics – Shirley & Marschner](https://www.amazon.co.jp/dp/1482229390)
- 📖 [Real-Time Rendering Resources](https://www.realtimerendering.com/index.html)
- 📖 [Physically Based Rendering](https://pbr-book.org/4ed/contents)
- 🌐 [Scratchpixel 2.0](https://www.scratchapixel.com/)

### Parametric Representation
- 🌐 [Bézier Curve](https://pomax.github.io/bezierinfo/)

### Rendering / Ray Tracing
- 🌐 [Ray Tracing in One Weekend series](https://raytracing.github.io/)

### Spatial Data Structures
- 📖 [PBR Book, Ch. 7](https://pbr-book.org/4ed/Primitives_and_Intersection_Acceleration)

### Implicit Modeling
- 🌐 [2D SDF by Inigo Quilez](https://iquilezles.org/articles/distfunctions2d/)

### Geometry / Mesh Processing
- 📖 [Polygon Mesh Processing – Botsch et al.](https://www.amazon.co.jp/dp/1568814267)
- 🌐 [Geometry Processing course – Alec Jacobson](https://github.com/alecjacobson/geometry-processing-csc2520)

### Gaussian Splatting
- 📄 [3D Gaussian Splatting (original paper & resources)](https://repo-sam.inria.fr/fungraph/3d-gaussian-splatting/)

### Differentiable Rendering
- 🌐 [Nvdiffrast](https://nvlabs.github.io/nvdiffrast/)

### Related Courses by the Instructor
- 🌐 [Physics-based Animation 2023S](https://github.com/nobuyuki83/Physics-based_Animation_2023S)
- 🌐 [Physics-based Animation 2021S](https://github.com/nobuyuki83/Physics-based_Animation_2021S)


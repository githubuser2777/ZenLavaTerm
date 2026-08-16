# LavaTerm Simulation & Physics Model

## 1. Overview

The core simulation of **LavaTerm** models organic, fluid-like lava lamp motion using the **Metaball (Isosurface)** algorithm coupled with classical buoyancy and viscous drag physics.

The simulation space is normalized to $[0.0, 1.0] \times [0.0, 1.0]$ where:
- $X \in [0.0, 1.0]$ (left to right)
- $Y \in [0.0, 1.0]$ (bottom to top, where $Y=0.0$ is the heat source bottom and $Y=1.0$ is the cooling top)

---

## 2. Mathematical Model

### 2.1. Scalar Field Evaluation

For a collection of $N$ metaballs, each with center position $\mathbf{p}_i = (x_i, y_i)$ and radius $R_i$, the scalar field intensity $F(x, y)$ at any test point $(x, y)$ is given by the superposition of individual radial potential functions:

$$F(x, y) = \sum_{i=1}^{N} f_i(d_i)$$

where $d_i = \|\mathbf{p} - \mathbf{p}_i\| = \sqrt{(x - x_i)^2 + (y - y_i)^2}$.

Standard Inverse-Square Potential:
$$f_i(d_i) = \frac{R_i^2}{d_i^2 + \epsilon}$$

or the computationally friendly polynomial smooth cutoff kernel:
$$f_i(d_i) = \begin{cases} \left(1 - \left(\frac{d_i}{R_i}\right)^2\right)^2 & \text{if } d_i < R_i \\ 0 & \text{if } d_i \ge R_i \end{cases}$$

### 2.2. Isosurface Thresholding

A virtual pixel at $(x, y)$ is considered part of the fluid lava if its cumulative field intensity exceeds a threshold $T$:

$$\text{is\_lava}(x, y) = F(x, y) \ge T$$

When two metaballs come close together, their field potentials add up ($F(x, y) = f_1 + f_2$), smoothly bridging the gap between them to form a unified, organic drop before splitting apart.

---

## 3. Physics & Thermodynamics

Each blob $i$ is characterized by state vector:
$$\mathbf{S}_i = \left( \mathbf{p}_i, \mathbf{v}_i, R_i, \Theta_i \right)$$
where $\Theta_i \in [0.0, 1.0]$ represents the normalized internal temperature of the blob.

### 3.1. Thermal Buoyancy & Convection

Blobs exchange heat with the environment:
- Bottom boundary ($y \approx 0.0$): Heating plate. Blob heats up ($\Theta_i \to 1.0$).
- Top boundary ($y \approx 1.0$): Cooling zone. Blob cools down ($\Theta_i \to 0.0$).

The vertical buoyancy force $F_{\text{buoyancy}}$ depends on the difference between the blob's temperature $\Theta_i$ and the neutral equilibrium temperature $\Theta_0 = 0.5$:

$$F_{\text{buoyancy}} = k_b \cdot (\Theta_i - \Theta_0)$$

### 3.2. Gravity & Viscous Drag

Total vertical acceleration $a_y$:
$$a_y = F_{\text{buoyancy}} - g$$

Viscosity / fluid drag applies an opposing force proportional to the blob's velocity:
$$\mathbf{F}_{\text{drag}} = -\mu \cdot \mathbf{v}_i$$

### 3.3. Thermal Noise & Brownian Perturbation

To avoid robotic, linear trajectories, a small pseudo-random noise term $\mathbf{\eta}(t)$ is added to horizontal and vertical velocities:
$$\mathbf{v}_i \leftarrow \mathbf{v}_i + \mathbf{\eta}(t) \cdot \text{noise\_strength}$$

---

## 4. Numerical Integration & Timestep

Positions and velocities are integrated using Euler-Cromer or Velocity Verlet integration with bounded delta time $\Delta t$:

$$\Delta t_{\text{effective}} = \min(\Delta t, \Delta t_{\text{max}})$$

$$\mathbf{v}_i(t + \Delta t) = \mathbf{v}_i(t) \cdot (1 - \mu \cdot \Delta t) + \mathbf{a}_i \cdot \Delta t$$
$$\mathbf{p}_i(t + \Delta t) = \mathbf{p}_i(t) + \mathbf{v}_i(t + \Delta t) \cdot \Delta t$$

Clamping $\Delta t_{\text{effective}}$ prevents numerical explosions if the terminal process is suspended or experiences frame hitches.

---

## 5. Determinism and Testing

Unit tests for simulation math utilize seeded PRNG sources or fixed initial conditions to ensure 100% deterministic test execution across platforms.

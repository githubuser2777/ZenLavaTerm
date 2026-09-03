# Simulation & Metaball Physics

The core simulation subsystem (`src/core/`) models the fluid mechanics, thermal buoyancy, viscous drag, and interaction dynamics of lava lamp blobs in normalized continuous space.

---

## 1. Mathematical Model

### 1.1 Scalar Potential Field Summation
Blobs are modeled as metaballs exerting an inverse-square potential field. For $N$ active blobs at positions $(x_i, y_i)$ with radii $R_i$:

$$F(x, y) = \sum_{i=1}^{N} \frac{R_i^2}{(x - x_i)^2 + (y - y_i)^2 + \epsilon}$$

Where $\epsilon = 10^{-5}$ prevents division by zero when evaluating at a blob's center.

### 1.2 Thermal Field Weighted Interpolation
Each blob carries a temperature $T_i \in [0.0, 1.0]$. The composite temperature $T(x, y)$ at any spatial point is evaluated via inverse-distance weighting:

$$T(x, y) = \frac{\sum_{i=1}^N T_i \cdot \frac{R_i^2}{(x - x_i)^2 + (y - y_i)^2 + \epsilon}}{F(x, y)}$$

This ensures smooth color gradient blending when two blobs merge.

---

## 2. Classical Forces & Numerical Integration

For each blob with velocity $\mathbf{v} = (v_x, v_y)$ and position $\mathbf{p} = (p_x, p_y)$:

1. **Thermal Buoyancy**:
   - Hotter blobs rise against gravity; colder blobs sink:
     $$F_{\text{buoyancy}} = k_b \cdot (T_i - T_{\text{ambient}})$$
2. **Gravity**:
   - Constant downward acceleration:
     $$F_{\text{gravity}} = -g$$
3. **Viscous Drag (Damping)**:
   - Fluid drag opposes velocity proportionally:
     $$\mathbf{v}(t + \Delta t) = \mathbf{v}(t) \cdot (1 - \mu \cdot \Delta t)$$
4. **Thermal Heating & Cooling**:
   - Blobs near the bottom gain heat (simulating the heating filament); blobs near the top cool down.
5. **Elastic Boundary Reflection**:
   - Blobs bouncing against viewport edges $[0.0, 1.0]$ undergo velocity inversion with restitution damping.

---

## 3. Interactive Dynamics (`src/core/interaction.rs`)

User inputs and hardware telemetry perturb the fluid via four interaction primitives:

- **Radial Shockwave (Mouse Left-Click)**:
  - Repels blobs radially with smooth inverse-distance falloff and thermal excitation.
- **Fluid Stirring (Mouse Drag)**:
  - Transfers directional momentum from the cursor vector into nearby blobs, simulating a stirring spoon.
- **Harmonic Ripples (Keyboard Typing)**:
  - Induces micro-oscillations across blob velocities corresponding to keystroke frequency.
- **Convective Pressure (Mouse Scroll)**:
  - Modulates vertical buoyant pressure throughout the column.

# Set ranges, &c.
set terminal pngcairo 1280,960
set yrange [-1:1]
set xrange [0:5]

x1 = sin(0)
x1 = 0
y1 = sin(x1)
x2 = 2
y2 = sin(x2)

m = (y2 - y1) / (x2 - x1)
b = y1 - m * x1
linear_interp(x) = m * x + b

# Plot both functions with given line styles.
plot linear_interp(x) w l lc 15, sin(x) w l lc 22

set terminal pngcairo size 1280,960
set yrange [-1:1]
set xrange [0:5]

x1 = sin(0)
x1 = 0
y1 = sin(x1)
x2 = 2
y2 = sin(x2)

# Define linear function.
m = (y2 - y1) / (x2 - x1)
b = y1 - m * x1
linear_interp(x) = m * x + b

# Define cosine interpolation function.
cos_interp(x) = y1 * (1 - (1 - cos(pi * ((x - x1) / (x2 - x1)))) / 2) + y2 * ((1 - cos(pi * ((x - x1) / (x2 - x1)))) / 2) 

plot linear_interp(x) w l lc 15, cos_interp(x) w l lc 10, sin(x) w l lc 22

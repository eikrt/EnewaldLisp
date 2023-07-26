convert -size 100x32 xc:pink -fill white -pointsize 8 -gravity center -annotate +0+0 "Test Coverage" media/test_coverage.png
convert -size 100x32 xc:orange -fill white -pointsize 8 -gravity center -annotate +0+0 "Benchmark avg: " media/benchmark.png
convert -size 512x128 xc:orange -fill white -pointsize 8 -gravity center -annotate +0+0 "EnewaldLisp" media/logo.png
cat << 'EOF' > README.md
* EnewaldLisp

![logo](media/logo.png)
![logo](media/benchmark.png)
![logo](media/test_coverage.png)

** 
EOF

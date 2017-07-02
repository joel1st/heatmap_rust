#! /bin/bash
NUMBER_OF_POINTS=10000 node generate_points.js > points.txt

./target/release/heatmap --image_width=36000 --image_height=18000 --diameter=100 --color=blues --location_source=file --file_output=intense_large.png
echo "1 done"

./target/release/heatmap --image_width=36000 --image_height=18000 --diameter=200 --color=reds --location_source=file --file_output=medium_intensity.png
echo "1 done"

./target/release/heatmap --image_width=36000 --image_height=18000 --diameter=400 --color=purples --location_source=file --file_output=average_intensity.png
echo "3 done"

gm convert intense_large.png -blur 20x9 -segment 0.000001 -transparent black output_large.png
echo "1 curves done"
gm convert medium_intensity.png -blur 20x9 -segment 0.000001 -transparent black output_large2.png
echo "2 curves done"
gm convert average_intensity.png -blur 20x9 -segment 0.000001 output_large3.png
echo "3 curves done"
gm convert xc:transparent -compose Over output_large3.png output_large2.png output_large.png -mosaic result_large.png
echo "mosaic done"
gm convert -crop 5000x5000 result_large.png +adjoin tile%04d.png
echo "mosaic done"

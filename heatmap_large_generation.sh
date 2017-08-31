#! /bin/bash
NUMBER_OF_POINTS=50000 node generate_points.js > points.txt

./target/release/heatmap --image_width=3600 --image_height=1800 --diameter=1 --color=blues1 --location_source=mongo --file_output=blues1.png
./target/release/heatmap --image_width=3600 --image_height=1800 --diameter=1 --color=blues2 --location_source=mongo --file_output=blues2.png
./target/release/heatmap --image_width=3600 --image_height=1800 --diameter=1 --color=blues3 --location_source=mongo --file_output=blues3.png
./target/release/heatmap --image_width=3600 --image_height=1800 --diameter=2 --color=blues4 --location_source=mongo --file_output=blues4.png
./target/release/heatmap --image_width=3600 --image_height=1800 --diameter=3 --color=blues5 --location_source=mongo --file_output=blues5.png
./target/release/heatmap --image_width=3600 --image_height=1800 --diameter=5 --color=blues6 --location_source=mongo --file_output=blues6.png
./target/release/heatmap --image_width=3600 --image_height=1800 --diameter=8 --color=blues7 --location_source=mongo --file_output=blues7.png
./target/release/heatmap --image_width=3600 --image_height=1800 --diameter=13 --color=blues8 --location_source=mongo --file_output=blues8.png
./target/release/heatmap --image_width=3600 --image_height=1800 --diameter=21 --color=blues9 --location_source=mongo --file_output=blues9.png
./target/release/heatmap --image_width=3600 --image_height=1800 --diameter=34 --color=blues10 --location_source=mongo --file_output=blues10.png
./target/release/heatmap --image_width=3600 --image_height=1800 --diameter=55 --color=blues11 --location_source=mongo --file_output=blues11.png
./target/release/heatmap --image_width=3600 --image_height=1800 --diameter=89 --color=blues12 --location_source=mongo --file_output=blues12.png
./target/release/heatmap --image_width=3600 --image_height=1800 --diameter=144 --color=blues13 --location_source=mongo --file_output=blues13.png
./target/release/heatmap --image_width=3600 --image_height=1800 --diameter=233 --color=blues14 --location_source=mongo --file_output=blues14.png
./target/release/heatmap --image_width=3600 --image_height=1800 --diameter=377 --color=blues15 --location_source=mongo --file_output=blues15.png
echo "1 done"

echo "first thing"
gm convert xc:transparent -compose Over black.png blues15.png blues14.png blues13.png blues12.png blues11.png blues10.png blues9.png blues8.png blues7.png blues6.png blues5.png blues4.png blues3.png blues2.png blues1.png -mosaic intensity_map.png

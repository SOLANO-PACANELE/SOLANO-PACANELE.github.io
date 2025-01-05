mkdir -p img2
for x in img0/*/*.png; do
    # echo $x;
    t2=$(echo img2/$(echo $x | cut  -d/ -f2-3))
    t1=$(echo img1/$(echo $x | cut  -d/ -f2-3))
    zz=$(echo $t2 | cut -d/ -f1-2)
    echo $t2
    for k in $(seq 6 18); do
        (
            mkdir -p $zz
            magick convert $t1 +dither   -posterize $k   +noise gaussian -unsharp  0.5x0.5 $t2.$k.png
        ) &
    done
    wait
    magick convert $t2.*.png -average  $t2
    rm -f $t2.*.png

    for k in $(seq 6 18); do
        (
            magick convert $t2 +dither   -posterize $k   $t2.$k.png
        ) &
    done
    wait
    magick convert $t2.*.png -average  $t2
    rm -f $t2.*.png

    magick convert $t2 -alpha extract -blur 0x8 +dither -colors 3 -colors 2  -colorspace gray -normalize -blur 0x2 -morphology Erode:2 Diamond  $t2.alpha.png
    magick $t2 $t2.alpha.png -compose CopyOpacity -composite $t2.fin.png
    mv $t2.fin.png $t2
    rm -f $t2.alpha.png

done
wait


echo ok
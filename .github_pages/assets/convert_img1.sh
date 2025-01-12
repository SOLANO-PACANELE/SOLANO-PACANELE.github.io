rm -rf img1 img2 img3
cp -a img0 img1


for i in 1 2 3; do



    mkdir -p img2
    for x in img1/*/*.png; do
        # echo $x;
        t=$(echo img2/$(echo $x | cut  -d/ -f2-3))
        zz=$(echo $t | cut -d/ -f1-2)
        echo $t
        (
            mkdir -p $zz
            magick convert $x -channel  A  -blur  0x3 -channel RGB -blur  0x3  -auto-level  -morphology Erode Octagon:5  -morphology Dilate Octagon:5 -unsharp 0x5 -channel RGB +noise gaussian $t
        ) &
    done
    wait

    for x in img1/*/*.png; do
        # echo $x;
        t2=$(echo img2/$(echo $x | cut  -d/ -f2-3))
        t3=$(echo img3/$(echo $x | cut  -d/ -f2-3))
        zz=$(echo $t3 | cut -d/ -f1-2)
        echo $t3
        (
            mkdir -p $zz
            magick convert $x $t2 -average  $t3
        ) &
    done
    wait
    rm -rf img1 img2
    mv img3 img1

done

echo ok
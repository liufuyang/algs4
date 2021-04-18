#!/bin/bash
set -e

alg4_dir=$PWD
lift_dir=/usr/local/lift
bin_dir=~/bin

mkdir -p $alg4_dir/.lift
cd $alg4_dir/.lift
curl -O "https://lift.cs.princeton.edu/java/linux/lift-cli.zip"
unzip lift-cli.zip
rm -f lift-cli.zip
cd $alg4_dir

sudo ln -sf $alg4_dir/.lift/lift           $lift_dir
ln -sf $alg4_dir/.lift/bin/checkstyle      $bin_dir/checkstyle
ln -sf $alg4_dir/.lift/bin/spotbugs        $bin_dir/spotbugs
ln -sf $alg4_dir/.lift/bin/java-algs4      $bin_dir/java-algs4
ln -sf $alg4_dir/.lift/bin/java-introcs    $bin_dir/java-introcs
ln -sf $alg4_dir/.lift/bin/javac-algs4     $bin_dir/javac-algs4

echo Java lib at:
echo $lift_dir/lib
ls   $lift_dir/lib
echo Extra binary at:
echo $bin_dir
echo You should hear an A-scale sound now, meaning install lation success

echo 'IMPRTANT: edit java-algs4 script to include stdlib.jar! (I will try do it here for you...)'
patch $bin_dir/java-algs4  algs4_setup.patch
patch $bin_dir/javac-algs4 algs4_setup.patch

echo Otherwise visit https://lift.cs.princeton.edu/java/linux/

java-introcs StdAudio

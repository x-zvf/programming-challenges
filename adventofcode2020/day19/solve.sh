#!/bin/sh
perl -ne "print if /`python solve.py $1`/" input | wc -l

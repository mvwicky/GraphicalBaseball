#!/bin/sh

cd ..
sass --watch scss:css --style compressed &
hhvm -m server -p 8080 &

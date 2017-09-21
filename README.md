RayTracer
===

[![Build Status](https://travis-ci.org/ranveeraggarwal/rust-raytracer.svg?branch=master)](https://travis-ci.org/ranveeraggarwal/rust-raytracer)

A ray tracer in rust. Based on Peter Shirley's Ray Tracing in One Weekend.

## Changelog

### September 21, 2017

Multi-threaded implementation using Rayon. 480x320 image at 100 samples took around 46 minutes to render.

### September 10, 2017

Single-threaded implementation complete. 1200x800 image at 10 samples took nearly 6 hours to render.

## Status

Currently, it has been run on 504 random spheres at 100 samples.    
CPU: Imtel Xeon E5-1650 v2: 6 cores/12 threads.

![One Weekend](outputs/one_weekend.jpg "One Weekend")
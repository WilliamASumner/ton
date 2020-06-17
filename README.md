# Ton
![Uh oh, the example picture didn't load](https://raw.githubusercontent.com/WilliamASumner/ton/example-images/specular-and-diffuse.png)
## Running it
Running Ton should be as easy as `git clone`ing the repo and running `cargo run`. Running with the `--release` flag will probably make it run faster. All output is directed to the `output/test.png` file, though this will probably change as I make this more complex.

## Description
Ton is my personal exploration project on raytracing and hopefully later path tracing. The main goal is to write *everything* I can from scratch, so that I understand how it all works from the ground up, to learn Rust in the process, and so that I can practice managing a larger project. I'd also like to use it to render some fluid-solver output eventually but that is a project for another day. Though I'm trying my best to keep things effecient and clear, I'm absolutely sure I will miss some optimizations and idiomatic Rust patterns, so if you're annoyed by that or just want to help, feel free to shoot me a pull request.

## Further Reading
A lot of what I've written so far is found in a few great books:
1. Physically Based Rendering by Pharr, Humphreys and Jakob (I would highly recommend this one, it covers pretty much everything, though it can get technical)
2. Fundamentals of Computers Graphics by Steve Marschner and Peter Shirley
3. More to come later...

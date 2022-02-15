# Flooting

## Preview
![Game Preview](readme/preview.gif)

## Setup
First you need to install vcpkg with cargo, to do that run: `cargo install cargo-vcpkg`

## Developing
Before you generate the development executable you need to compile the dependencies, if you don't know how to, you can go to the "Compiling the Dependencies" later on in this file.
Once you have the dependencies compiled you can run: `cargo run` to automatically build and run your app(this is going to generate an unoptimized version, so DO NOT use this as the deploy version, later on in this file you'll learn how to generate the optimized deploy version)

## Deploying
First you need to compile the dependencies, if you don't know how to, you can go to the "Compiling the Dependencies" later on in this file. 
Then you need to build the executable, you can do that with: `cargo build --release`

## Compiling the Dependencies
You need to compile the dependencies every time you change the dependencies, but as long as you don't change the dependencies you'll only need to compile them once, you can compile the dependencies running: `cargo vcpkg build`

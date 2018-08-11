# WebGL + Rust

because why not, i guess

* [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for Rust <-->
  JavaScript interop
* [`nalgebra`](http://nalgebra.org/) for linear algebra nonsense
* [`nphysics`](http://nphysics.org/) as a physics engine (this uses a modified
  version to fit `wasm-bindgen`, so you have to `git clone --recursive` and
  whatnot)
* [`lazy-static.rs`](https://github.com/rust-lang-nursery/lazy-static.rs) for
  keeping global state
* [`actix-web`](https://actix.rs/) for the server side
* [`bincode`](https://github.com/TyOverby/bincode) for
  serialization/deserialization over the network
* [`pcg_rand`](https://github.com/robojeb/pcg_rand) for generating very
  high-quality random numbers very quickly

## try

```bash
git clone --recursive https://github.com/AugmentedFifth/webgl_test.git
cd webgl_test
./build.sh fresh
cd server
make
cd ../out/dist
../../server/target/release/webgl_test_server

firefox localhost:11484
```

## license

![GNU Affero General Public License version 3+](https://www.gnu.org/graphics/agplv3-155x51.png)

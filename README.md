# description
just a simple calculator
it supports 5 operations: 
  addition          +
  substraction      -
  multiplication    *
  division          /
  power             ^

# Building

- clone repository if not already cloned
  ```
  git clone httos://github.com/11mushroom/simple_rusty_calc.git
  ```

- to build binary
  
  go to the root directory of the repository
  ```
  cd simple_rusty_calc
  ```

  and build it with cargo
  ```
  cargo build --release
  ```

  the `simple_rusty_calc` binary will be in `target/release` directory

# usage of simple_rusty_calc script
  ```bash
  ./simple_rusty_calc "(2+2)*2"
  ```
  it will print the result of expression

  example:
  ```bash
  $ ./simple_rusty_calc "(2+2)*2"
  8
  ```


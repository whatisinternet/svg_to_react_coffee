# SVG to React Coffee

Converts SVG images into valid(ish) React SVG components that can be easily
embeded into react render functions.

-----

## TODO:
- Write to something other than std::out
- Fix issue with style attribute not being correctly parsed.

## Installation:

### Compile yourself:

1. Install [Rust and cargo](http://doc.crates.io/)
2. git clone git@github.com:whatisinternet/svg_to_react_coffee.git
3. cd svg_to_react_coffee && cargo build --release
4. You can find the executable in target/release

### Ruby gem:
  Coming soon!

## Usage
  - Print to screen:

    ./svg_to_react_coffee file.svg

  - Write to file:

    ./svg_to_react_coffee file.svg > output.coffee

## Contributing

1. Fork it ( https://github.com/whatisinternet/svg_to_react_coffee/fork )
2. Create your feature branch (`git checkout -b my-new-feature`)
3. Commit your changes (`git commit -am 'Add some feature'`)
4. Push to the branch (`git push origin my-new-feature`)
5. Create a new Pull Request

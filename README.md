# SVG to React Coffee

Converts SVG images into valid React SVG components that can be easily
embeded into react render functions.

-----

# Demo:
[Demo app](https://github.com/whatisinternet/react-svg-demo)

## TODO:
- Only write out if within a valid attribute
- Write to something other than std::out
- Return warnings on failed tags
- Config options for passing in supported tags, and attributes
- Possibly split off HTML renderer as related project

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

## Limitations:

This cargo is limited by [ReactJS's implementation of the SVG spec](https://facebook.github.io/react/docs/tags-and-attributes.html).

The currently supported list of tags are as follows:
```code
  circle clipPath defs ellipse g line linearGradient 
  mask path pattern polygon polyline radialGradient 
  rect stop svg text tspan
```

Currently any failed tag gets dropped and the parser will continue onwards
without warning.

Further, some tags while supported do not fully support all attributes.

The currently supported list of attributes are as follows:
```code
  style clipPath cx cy d dx dy fill fillOpacity fontFamily fontSize fx fy
  gradientTransform gradientUnits markerEnd markerMid markerStart offset opacity
  patternContentUnits patternUnits points preserveAspectRatio r rx ry
  spreadMethod stopColor stopOpacity stroke strokeDasharray strokeLinecap
  strokeOpacity strokeWidth textAnchor transform version viewBox x1 x2 x y1 y2 y
```

[Open SVG Pull requests for React](https://github.com/facebook/react/pulls?&q=is%3Apr+svg)


## Contributing

1. Fork it ( https://github.com/whatisinternet/svg_to_react_coffee/fork )
2. Create your feature branch (`git checkout -b my-new-feature`)
3. Commit your changes (`git commit -am 'Add some feature'`)
4. Push to the branch (`git push origin my-new-feature`)
5. Create a new Pull Request

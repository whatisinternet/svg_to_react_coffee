# SVG to React Coffee

[![Build
Status](https://travis-ci.org/whatisinternet/svg_to_react_coffee.svg)](https://travis-ci.org/whatisinternet/svg_to_react_coffee)

Converts SVG images into valid React SVG components that can be easily
embedded into react render functions.

-----

# Demo:
[Demo app](https://github.com/whatisinternet/react-svg-demo)

## Installation:

### Compile yourself:

1. Install [Rust and cargo](http://doc.crates.io/)
2. git clone git@github.com:whatisinternet/svg_to_react_coffee.git
3. Library: cd svg_to_react_coffee && cargo build --release --lib
3. Executable: cd svg_to_react_coffee && cargo build --release --lib && cargo build --release --bin svg_to_react_bin
4. You can find the executable in target/release

## Usage:

### As an executable:
  - Print to screen:

```shell
./svg_to_react_bin file.svg
```

  - Write to file:

```shell
./svg_to_react_bin file.svg > output.coffee
```

### As a library:

```rust
...
extern crate svg_to_react;

use svg_to_react::convert;
...
fn main() {
...
  convert(file_path);
...
}

```

NB: The library has been set to export as a C non-clobbered method to inter-op
with Python, Ruby, C, etc.

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
  width height style clipPath cx cy d dx dy fill fillOpacity fontFamily fontSize fx fy
  gradientTransform gradientUnits markerEnd markerMid markerStart offset opacity
  patternContentUnits patternUnits points preserveAspectRatio r rx ry
  spreadMethod stopColor stopOpacity stroke strokeDasharray strokeLinecap
  strokeOpacity strokeWidth textAnchor transform version viewBox x1 x2 x y1 y2 y
```

and 

```code
accept acceptCharset accessKey action allowFullScreen allowTransparency alt
async autoComplete autoFocus autoPlay cellPadding cellSpacing charSet checked
classID className colSpan cols content contentEditable contextMenu controls
coords crossOrigin data dateTime defer dir disabled download draggable encType
form formAction formEncType formMethod formNoValidate formTarget frameBorder
headers height hidden high href hrefLang htmlFor httpEquiv icon id label lang
list loop low manifest marginHeight marginWidth max maxLength media mediaGroup
method min multiple muted name noValidate open optimum pattern placeholder
poster preload radioGroup readOnly rel required role rowSpan rows sandbox scope
scoped scrolling seamless selected shape size sizes span spellCheck src srcDoc
srcSet start step style tabIndex target title type useMap value width wmode
```

Also any attribute that uses data-* or aria-*

[Open SVG Pull requests for React](https://github.com/facebook/react/pulls?&q=is%3Apr+svg)


## Contributing

1. Fork it ( https://github.com/whatisinternet/svg_to_react_coffee/fork )
2. Create your feature branch (`git checkout -b my-new-feature`)
3. Commit your changes (`git commit -am 'Add some feature'`)
4. Push to the branch (`git push origin my-new-feature`)
5. Create a new Pull Request

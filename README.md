![Rust](https://github.com/nickmachnik/fastatools/workflows/Rust/badge.svg?branch=master)
![Build Status](https://travis-ci.org/nickmachnik/fastatools.svg?branch=master)

# fastatools

`fastatools` is a command line application written in Rust that provides methods for fast parsing, indexing and manipulating FASTA files.

## Installing

### Latest release (Linux only)

Download the latest [release](https://github.com/nickmachnik/fastatools/releases/latest) to a directory in which you would like to keep the binary.
For example:

```
cd
mkdir ./.fastatools
cd .fastatools
wget https://github.com/nickmachnik/fastatools/releases/download/v0.1.0/fastatools-v0.1.0-x86_64-unknown-linux-gnu.tar.gz
tar -xf fastatools-v0.1.0-x86_64-unknown-linux-gnu.tar.gz
```

Add that directory to your path. On Ubuntu you could add this line to your `.bashrc`:

```
export PATH="~/.fastatools:$PATH"
```

### Compile from scratch

To use the latest unreleased version, download this repo and build it with [cargo](https://github.com/rust-lang/cargo):
```
git clone https://github.com/nickmachnik/fastatools.git
cd fastatools
cargo test
cargo build --release
./target/release/fastatools -h
```
<!-- 
### Linux

Download the latest [release](https://github.com/nickmachnik/fastatools/releases/latest) to a directory in which you would like to keep the binary.
For example:

```
cd
mkdir ./.fastatools
cd .fastatools
wget [LINK TO RELEASE]
tar -xf [RELEASEFILE]
```

Add that directory to your path. On Ubuntu you could add this line to your `.bashrc`:

```
export PATH="~/.fastatools:$PATH"
``` -->

## Usage

```
fastatools -h
```

## Contributing & Suggestions

This project grows with my own needs. Feature suggestions and pull requests are welcome!

## License

Double licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  https://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

<!-- 
End with an example of getting some data out of the system or using it for a little demo

## Running the tests

Explain how to run the automated tests for this system

### Break down into end to end tests

Explain what these tests test and why

```
Give an example
```

### And coding style tests

Explain what these tests test and why

```
Give an example
```

## Deployment

Add additional notes about how to deploy this on a live system

## Built With

* [Dropwizard](http://www.dropwizard.io/1.0.2/docs/) - The web framework used
* [Maven](https://maven.apache.org/) - Dependency Management
* [ROME](https://rometools.github.io/rome/) - Used to generate RSS Feeds

## Contributing

Please read [CONTRIBUTING.md](https://gist.github.com/PurpleBooth/b24679402957c63ec426) for details on our code of conduct, and the process for submitting pull requests to us.

## Versioning

We use [SemVer](http://semver.org/) for versioning. For the versions available, see the [tags on this repository](https://github.com/your/project/tags).

## Authors

* **Billie Thompson** - *Initial work* - [PurpleBooth](https://github.com/PurpleBooth)

See also the list of [contributors](https://github.com/your/project/contributors) who participated in this project.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details

## Acknowledgments

* Hat tip to anyone whose code was used
* Inspiration
* etc

 -->
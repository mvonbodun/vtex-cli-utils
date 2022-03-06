# VTEX CLI Utilities

**VTEX CLI Utilities** is a collection of command line programs written in RUST.  There are three main projects:

- **impex** - a CLI that focuses on the loading of catalog, pricing and inventory data
- **algolia** - a CLI that extracts data from VTEX and a JSON format suitable for importing into Algolia
- **vtex** - a RUST library that contains the VTEX models (structs) that the API's expects and utility methods for extracting the category tree and looking up VTEX identifiers

These collection of programs were developed as open source and are not supported by the VTEX Development team.  We welcome help in developing and enhancing the programs.  The utilities are very much in Alpha and could use more polish and documentation.  These programs are supplied as is and come with no warranty.

## impex
**impex** stands for Import / Export.  The idea behind this utility is to have a standard way of loading large quantities of data using CSV files.  This utility is most useful for a new implementation of VTEX.  It is highly performant and uses concurrency, threads and rate limiters where necessary to maximize the speed at which data can be loaded into VTEX.  This project is the furthest along of the utilities.
For more infofrmation, see the [README.md](impex/README.md) for the project.

## algolia
**algolia** is a utility to extract data from VTEX and build a JSON structure that is ingestible by Algolia.  This program will extract all the SKUs from VTEX, get pricing and inventory and build a JSON structure following the recommendations of Algolia to use instant search.  This is very much an Alpha release and many of the specification types (Size / Color) are hardcoded.
For more information, see the algolia folder.

## vtex
**vtex** is a RUST library that contains the reusable code needed by both the **impex** and **algolia** projects.  It has a set of **models** that represent the JSON formats used by the VTEX APIs.  It also contains a **utils** module that has a set of functions useful for extracting data from VTEX APIs.
For mmore information, see the vtex folder.

## Contributing
If you wish to contribute, you can do the following:
- Open issues in Github for bugs or functionality requests
- Install RUST (https://www.rust-lang.org/), checkout the code from Github and do a build using cargo: **cargo build**
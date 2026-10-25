rustfilmar

A desktop movie search application built with Rust.

Overview

rustfilmar is a desktop application that lets you search for movies and explore their information through a simple, neon-inspired graphical interface.

Features

* Search for movies by title.
* View movie posters.
* Display release years and user ratings.
* Read movie overviews.
* Browse search results in a scrollable interface.
* Neon-inspired dark UI.

Screenshots

<!-- Add your screenshots to the screenshots/ directory and uncomment the line below. -->
<!-- ![Movie search interface](screenshots/main.png) -->

Built With

* Rust — Application logic
* Iced — Graphical user interface
* Reqwest — HTTP requests
* Serde — JSON deserialization
* TMDB API — Movie information and poster images

Getting Started

Prerequisites

* Rust and Cargo
* A TMDB API access token

Installation

Clone the repository:
```bash
git clone https://github.com/kiracracker/rustfilmar.git
cd rustfilmar
```
Configure your TMDB API token in the source code, then run:

```bash
cargo run
```
To build an optimized version:
```bash
cargo build --release
```
Project Status

This project is a work in progress.

License

No license has been specified yet.

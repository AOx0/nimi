# nimi

Super-simple 15 min project to create Manim project directories with a basic structure. 

	project_name
	├── cfg
	│  ├── manim.cfg
	│  ├── manim_low.cfg
	│  └── manim_very_low.cfg
	├── justfile
	├── main.py
	└── README.md

A folder with the `project_name`. A _just_ file that renders the animation and a `cfg` subfolder with various dimensions.

## Installation

	cargo install --git https://github.com/AOx0/nimi


## Usage
Create a new project in the current directory with the default name `manim_template`

	nimi

Create a new project in the current directory called `new_animation`

	nimi -n new_animation

Create a new project in the parent directory called `lol`

	nimi ../ -n lol

Etc.


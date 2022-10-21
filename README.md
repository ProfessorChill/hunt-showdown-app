# Hunt App
Currently this is a website built from the inspiration of the [Chaos Loadout](https://richardcqc.com/chaos-loadout.html) website, but with a lot more options and customizability. I did this to test and build on my Rust programming skills.


While I do plan on adding a few other things to this website, for now I feel I will be releasing this project as is for other people to learn from as well.


This would not have been possible without [yew.rs](https://yew.rs/) an amazing framework for building front-end web apps using WebAssembly. I highly recommend looking at that project and contributing to it as well! And [rembg](https://github.com/danielgatis/rembg) for the amazing utility for removing backgrounds from images with AI without paying an arm and a leg to use it (it's free ☺).

## Requirements
- [Rust](https://www.rust-lang.org/)
- [trunk](https://trunkrs.dev/) for testing, building, and releasing the website.
- Python3 (for building images `process_images.py`)
	- [rembg](https://github.com/danielgatis/rembg) (installed as an executable)
	- [wand](https://github.com/emcconville/wand)
- Node (npm) for [bluma](https://bulma.io/) and nothing else.

## Building the images
The SVG files have to be manually made using inkscape (or whatever you want) currently. I don't have it being automatically done in Python yet.


The `process_images.py` script relies on [rembg](https://github.com/danielgatis/rembg) and [wand](https://github.com/emcconville/wand) the paths are not checked and this script sould be considered **UNSAFE** since it accesses the `os.mkdir` and `shutil.rmtree` functions. While I've tried to ensure it does not remove anything else by accident please use this script **AT YOUR OWN RISK** it was made quickly with no intent to be published, I'm only pushing it to the git for my own future use. This script is very unoptimized and is horribly made, this should not be used as an example of my "professional" work and is simply there because I didn't want to convert hundreds of images by hand.

## Building
Local testing.
```
$ npm i
$ trunk serve
```
Building for dist.
```
$ npm i
$ trunk build --release
```
Then copy ./dist to the server.

# Q&A

## Why WebAssembly?
I just think it's a really neat technology and I want to learn how to build utilizing it better. To top it off it allows me to use Rust which is my favorite programming language.

## How are the images referenced without paths?
In `src/content/generic_item.rs` for weapons/tools/consumables there is `GenericItem::to_weapon_path`/`GenericItem::to_tool_path`/`GenericItem::to_consumable_path` which takes the name, removes spaces, dots and brackets, if a variant is provided it takes the `fmt::Display` and removes the spaces, if there is no variant it's not provided. Lastly it checks for postfixes (only for Caldwell Conversion **Pistol** right now) and appends to the end.


This would make `Caldwell Conversion Chain Pistol` become `/images/weapons/CaldwellConversionChainPistol.png`


In `content.rs` for bullets there is `BulletVariant::to_svg_path` which similar to `GenericItem::to_weapon_path` combines name and variant without spaces. There are some caviats where BulletVariant optionally accepts weapon as an input for the case of Special but on a crossbow requiring `CrossbowBoltPoison.svg` instead of `SpecialPoison.svg` and so on so forth. It also requires a `BulletSize` to be provided for clear reasons.


This would make `Long Full Metal Jacket` into `/images/bullets/LongFullMetalJacket.svg`.

## Why JSON instead of RSON when you're using Rust?
This was actually an oversight, when I was initially building the datasets I didn't have a programming language in mind, I was simply making the datasets for future use and figured "if I'm using JS then JSON works, if I'm using Rust then JSON works, if I'm using GO then JSON works, and if I'm using Python (server-side rendered) then JSON still works". I might change from JSON to RSON in the future, but for now it is what it is.

## This json data is incorrect
It is currently updated to 1.10 including the ammo purchase changes (half price on weapons with two ammo slots) with the exception of the issues listed in [TODO](#todo)


It might be, this is because I manually created the `data/*.json` files by launching the game and going through the data since Crytek does not want people data-mining their game (which would be very difficult considering the games data files are encrypted anyway).


If you would like to make a commit to correct the data, I request three things
1. Please do not use data from the [Hunt: Showdown Wiki](https://huntshowdown.fandom.com/wiki/Hunt:_Showdown_Wiki) while I'm sure 99% of their data is correct, I would like my data to be sourced from in-game to be as up to date as possible.
2. If possible please commit with a message containing a link to proof of the change (i.e. A picture uploaded to [imgur](https://imgur.com/) of how much experience is needed to unlock the desired object). This is simply to ensure the data was found in-game or though patch notes.
3. Typos will need the images/raw_images changed to be accepted, this is simply because the image files are a generated name. `images/type/name + variant + postfix.webp` so if a name is changed then the image name needs to be changed as well.

## Why are there two raw_images for status_icons/icons/bullets (.xcf and .svg)
XCF is the picture I took in-game. I then open it in Inkscape and trace the icon to make it an SVG. The icon is then saved as an **INKSCAPE SVG** which I later save as a normal SVG for use on the website.

## Why don't the raw_images have transparent backgrounds?
The `process_images.py` script gets all `.xcf` files in `./raw_images` and converts them to `.png` files then moves them to `./images` using [wand](https://github.com/emcconville/wand), it then uses a subprocess (because the rembg python function has a memory leak) to [rembg](https://github.com/danielgatis/rembg) the backgrounds of all images in that folder, I'm aware that `rembg p <input_folder> <output_folder>` exists however since I have a few images that I manually removed the backgrounds of and I'm making it ignore those certain images, hence using `rembg i <input_file> <output_file>`. It then does a lossy conversion from `.png` to `.webp` to save on data transfer.

## Why does the data look so weird?
Serde Json (as far as I know) currently does not support `Vec<T>` where T is an Enum struct. However it does support it if you provide the following.
```rust
#[derive(Deserialize)]
#[serde(tag = "t", content = "c")]
enum DataSet {
	Thing {
		val: u8,
		magic: Vec<u8>,
	}
}
```
Where `tag = "t"` can be anything even `tag = "banana"` and `content = "c"` can be anything as well.


As a result my data that utilizes enum structs results in looking like:
```json
[
	{
		"data": [
			{"t": "Thing", "c": {"val": 0, "magic": [0, 1]}}
		]
	}
]
```

The `variants` field is only for the base weapon since a variant cannot have variants.

## The SVGs are horrid
Thanks! Please submit a commit with better looking ones, I'm a coder not an artist :)

## TODO
- Limits on how much item type can show up in advanced options.
- Make a new `data/bullets.json` since they're a static calcuation and don't need to be re-defined multiple times in `data/weapons.json`

## TODO Later
- Pick random legendary skin.

## Known things that aren't bugs
These are logic programming that seems odd but makes sense when looking at it, I'll try to make errors more clear in the future.


- If the budget is set TOO LOW to purchase locked in items it will still keep them but won't calculate the cost.
- "Always Dual Wield" and "Always Duplicate Weapons" can be checked but "Always Quartermaster" will uncheck those. This is because dual wield is a medium slot weapon, and we can't duplicate a large weapon, if we duplicate a medium weapon it's not utilizing quartermaster.

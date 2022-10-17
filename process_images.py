"""
This script deletes directories from './images' that contain images with backgrounds that are to be removed.
"""

import os
import subprocess
import shutil
import sys

from wand.image import Image
from PIL import Image as PILImage

# Ignore these folders since they're manually created SVG files.
ignore_folders = [
    './raw_images/icons',
    './raw_images/bullets',
    './raw_images/status_icons',
    './images/icons',
    './images/bullets',
    './images/status_icons',
]

# Delete these folders when checking folders.
del_these = [
    './images/weapons',
    './images/consumables',
    './images/tools',
]

# Do not run rembg on these images, convert last.
# These are images that look horrid with rembg and have manually had their backgrounds removed.
do_not_rembg = [
    './images/weapons/CavalrySaber.png',
    './images/weapons/Martini-HenryIC1Riposte.png',
    './images/weapons/BerthierMle1892Riposte.png',
    './images/weapons/Romero77Alamo.png',
]


def info(msg: str) -> None:
    """Prints a message with the info formatting"""

    print(f'\033[30;42m[INFO]\033[0m {msg}')


def get_files_in_dir(search_dir: str, extensions: list[str]) -> list[str]:
    """Get files in a directory matching extensions.

    Parameters:
        search_dir (str): The directory to depth search by 1 for images.
        extensions (list[str]): The extensions to look for.

    Returns:
        list[str]: A list of paths containing the extension in the img_dir.
    """

    file_paths = []

    for ls_path in os.listdir(search_dir):
        current_dir = search_dir
        path_check = f'{current_dir}/{ls_path}'

        if os.path.isfile(path_check):
            # Get the path extension to check if it's an extension we're looking for.
            path_ext = os.path.splitext(path_check)
            path_ext = path_ext[len(path_ext) - 1]

            if path_ext in extensions:
                file_paths.append(path_check)
        elif path_check not in ignore_folders:
            new_paths = get_files_in_dir(path_check, extensions)
            file_paths = file_paths + new_paths

    return file_paths


def subprocess_remove_backgrounds() -> None:
    """Run a subprocess on rembg to remove the backgrounds of png files in './images'.
    I'm aware 'rembg p path_input path_output' exists however I'm trying to preserve
    certain images and delete others."""

    # This is made as a temporary workaround due to a GPU memory leak in the rembg remove command.
    image_paths = get_files_in_dir('./images', ['.png'])

    # Instead of using "rembg p path_input path_output" we are doing this to prevent running
    # rembg on images that have manual overrides listed in "do_not_rembg".
    for image_path in image_paths:
        if image_path in do_not_rembg:
            continue

        path_splitext = os.path.splitext(image_path)
        new_path = f'{path_splitext[0]}_rembg{path_splitext[1]}'

        info(f'Rembg {image_path}')
        command = subprocess.Popen(['rembg', 'i', image_path, new_path])
        command.wait()

        if os.path.exists(image_path) and os.path.isfile(image_path):
            os.remove(image_path)

        os.rename(new_path, image_path)


def convert_png_to_webp() -> None:
    """Convert PNG files in './images' to WebP"""

    image_paths = get_files_in_dir('./images', ['.png'])

    for image_path in image_paths:
        if os.path.exists(image_path) and os.path.isfile(image_path):
            im = PILImage.open(image_path)
            new_path = image_path.replace('.png', '.webp')
            info(f'Converting {image_path} to {new_path}')
            im.save(new_path, format='WebP', lossless=False)
            os.remove(image_path)


def save_images() -> None:
    """Save XCF files in './raw_images' to PNG in './images'"""

    image_paths = get_files_in_dir('./raw_images', ['.xcf'])

    for image_path in image_paths:
        img = Image(filename=image_path)

        with img.convert('png') as converted:
            # I know I can chain replace, it just looks better like this in my opinion.
            new_path = image_path.replace('./raw_images', './images')
            new_path = new_path.replace('.xcf', '.png')

            info(f'Converting {image_path} to {new_path}')
            converted.save(filename=new_path)


def dir_check() -> None:
    """Make './images' if it doesn't exist, if it does remove all directories flagged in
    'del_these' and all the content in them, then create the missing subdirectories
    in './images'"""

    if not os.path.exists('./images'):
        info('Creating missing directory ./images')
        os.mkdir('./images')
    else:
        # If images does exist we might have existing data, remove them, we want a fresh start.
        # We do however want to keep: bullets,icons,status_icons as those are
        # manually created data.

        for to_del in del_these:
            if os.path.exists(to_del) and os.path.isdir(to_del):
                info(f'Removing directory {to_del}')
                shutil.rmtree(to_del)

    for image_path in os.listdir('./raw_images'):
        current_path = './raw_images'
        check_path = f'{current_path}/{image_path}'
        create_path = f'./images/{image_path}'

        if os.path.isdir(check_path) and not os.path.exists(create_path):
            info(f'Creating missing directory {create_path}')
            os.mkdir(create_path)


def print_help() -> None:
    """Print the help message."""

    print(
        "python3 process_images.py\n\n"\
        "     check_remove       - Checks if the images directory is valid, removes all\n"\
        "                          weapons/consumables/tools images which are in the \"images\"\n"\
        "                          folder then creates missing directories in \"images\"\n"\
        "                          from \"raw_images\".\n\n"\
        "     save_images        - Takes all images not in \"ignore_folders\" and converts the\n"\
        "                          xcf in \"raw_images\" to png and moves them to \"images\"\n"\
        "                          in their respective folders.\n\n"\
        "     remove_backgrounds - Removes the backgrounds from the png images from non-ignored\n"\
        "                          folders in \"images\" and converts them to webp format for\n"\
        "                          web use, removing the original images. This ignores\n"\
        "                          \"ignored\" images (manually created).\n\n"\
        "     all                - This runs all of the above in the order: check_remove,\n"\
        "                          save_images, remove_backgrounds"
    )


if __name__ == '__main__':
    if len(sys.argv) < 2:
        print_help()
        sys.exit()

    for arg in sys.argv[1:]:
        if arg == 'check_remove':
            dir_check()
        elif arg == 'save_images':
            save_images()
        elif arg == 'convert':
            convert_png_to_webp()
        elif arg == 'remove_backgrounds':
            # Rembg in Python currently doesn't work due to a memory leak.
            # This is a temporary workaround to mitigate that.
            subprocess_remove_backgrounds()
            convert_png_to_webp()
        elif arg == 'all':
            dir_check()
            save_images()
            subprocess_remove_backgrounds()
            convert_png_to_webp()

    info('Args completed running!')

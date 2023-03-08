from PIL import Image
import os, sys

input_path = "/mnt/e/SteamLibrary/steamapps/common/Factorio/data/base/graphics/icons/"
output_path = "../icons/"

for file in os.listdir(input_path):
    if file.endswith(".png"):
        im = Image.open(input_path + file)
        im = im.crop((0, 0, 64, 64))
        im.save(output_path + file)

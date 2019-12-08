"""
https://adventofcode.com/2019/day/8
"""

from typing import List, Dict
from collections import Counter

def read_image(data: str, layer_width, layer_height) -> List[str]:
    layer_size = layer_width * layer_height
    n_layers = len(data)//layer_size
    assert n_layers*layer_size == len(data)
    return [data[i*layer_size:(i+1)*layer_size] for i in range(n_layers)]

def count_digits(layer: str) -> Dict[str, int]:
    return Counter([digit for digit in layer])


def get_count(digit: str, counter: Counter) -> int:
    try:
        n = counter[digit]
    except:
        n = -1
    return n


def check_corruption(data: str, layer_width, layer_height) -> int:
    layers = read_image(data, layer_width, layer_height)
    counts = [count_digits(layer) for layer in layers]
    counts_zero = [get_count('0', c) for c in counts]
    fewest_zero = counts_zero.index(min(counts_zero)) 
    target_layer = counts[fewest_zero]
    return target_layer['1'] * target_layer['2']


def decode_image(data:str, layer_width: int, layer_height: int) -> List[str]:
    image_size = layer_width * layer_height
    layers = read_image(data, layer_width, layer_height)
    # 0: black
    # 1: white
    # 2: transparent
    
    pixels = [[layer[i] for layer in layers] for i in range(image_size)]
    final_image = ''
    count = 0
    for pixel in pixels:
        for val in pixel:
            if val != '2': # not transparent
                if val == '0':
                    final_image += ' '
                    break
                elif val == '1':
                    final_image += '#'
                    break
                else:
                    raise ValueError('Wrong')
        count += 1
        if (count != 0) and (count % layer_width == 0):
            final_image += '\n'
    return final_image


    return pixels



LAYER_WIDTH = 25
LAYER_HEIGHT = 6

with open('day08_input.txt') as f:
    data = f.read()


part1 = check_corruption(data, LAYER_WIDTH, LAYER_HEIGHT)
print(f'part1: {part1}')

part2 = decode_image(data, LAYER_WIDTH, LAYER_HEIGHT)
print(f'part2:')
print(part2)
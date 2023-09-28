#!/usr/bin/env python

import numpy as np
from matplotlib._cm_listed import _magma_data, _inferno_data


def write_cmap(f, cmap_array, cmap_name):
    cmap_u8 = np.round(cmap_array * 255).astype(np.uint8)
    f.write('pub const {}_LUT: [[u8; 3]; 256] = [\n'.format(cmap_name.upper()))
    for i in range(256):
        f.write('    [{:>3}, {:>3}, {:>3}],\n'.format(*cmap_u8[i]))
    f.write('];\n\n')

magma_array = np.array(_magma_data)
inferno_array = np.array(_inferno_data)

with open('src/color_map_listed.rs', 'w') as f:
    write_cmap(f, magma_array, 'magma')
    write_cmap(f, inferno_array, 'inferno')


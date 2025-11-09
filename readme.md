# Barcode reader

Read barcodes by turning the pixels from an image of a barcode into bytes, then decoding them

![barcode](barcode.png)

1. Crops a center line out of the image
2. Turns the pixel values into bytes
3. Calculates average bar width to get the scale
4. Turns pixel bytes into correct binary
5. Decodes binary into correct digits

Output for the above image
``[7, 0, 5, 6, 3, 2, 4, 4, 1, 9, 4, 7]``

Current solution only works when the first digit is 0 and validation/repair needs to be improved so that it would work universally for all barcodes.
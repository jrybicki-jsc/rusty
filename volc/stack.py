import os
from PIL import Image

os.chdir('cropped')
images = os.listdir()

red_data =[]
green_data = []
blue_data = []

for image in images:
    with Image.open(image) as img:
         if image ==images[0]:
            img_size = img.size

         red_data.append(list(img.getdata(0)))
         green_data.append(list(img.getdata(1)))
         blue_data.append(list(img.getdata(2)))

avg_red = [round(sum(x) / len(red_data)) for x in zip(*red_data)]
avg_blue = [round(sum(x) / len(blue_data)) for x in zip(*blue_data)]
avg_green = [round(sum(x) / len(green_data)) for x in zip(*green_data)]

merged_data = [(x) for x in zip(avg_red, avg_green, avg_blue)]
stacked = Image.new('RGB', (img_size))
stacked.putdata(merged_data)
stacked.show()

os.chdir('..')
stacked.save('stacked.tiff', 'TIFF')
 

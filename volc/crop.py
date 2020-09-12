import os
import sys
import shutil
from PIL import Image, ImageOps

def main():
    img_dir = 'res/video_frames'
    files = os.listdir(img_dir)
    for file_num, fl in enumerate(files, start=1):
        with Image.open(os.path.join(img_dir, fl)) as img:
            gray = img.convert('L')
            bw = gray.point(lambda x: 0 if x < 90 else 255)
            box = bw.getbbox()
            padded_box = (box[0]-20, box[1]-20, box[2]+20, box[3]+20)
            cropped = img.crop(padded_box)
            scalled = ImageOps.fit(cropped, (860, 860), Image.LANCZOS, 0, (0.5, 0.5))
           
            scalled.save(f"cropped_{file_num}.jpg", 'JPEG')


if __name__=="__main__":
    main()

import os
import math
import random
import pygame as pg

WHITE = (255, 255, 255)
BLACK = (0, 0, 0)
RED = (255, 0, 0)
GREEN = (0, 255, 0)
LT_BLUE = (173, 216, 230)

class Satellite(pg.sprite.Sprite):
   def __init__(self, background):
       super().__init__()
       self.background = background
       self.image_sat = pg.image.load('res/satellite.png').convert()
       self.image_crash = pg.image.load('res/satellite_crash_40x33.png').convert()
       self.image = self.image_sat
       self.rect = self.image.get_rect()
       self.image.set_colorkey(BLACK)
      
       self.x = random.randrange(315, 425)
       self.y = random.randrage(70, 180)
       self.dx = random.choice([-3, 3])
       self.dy = 0
       self.heading = 0
       self.fuel = 100
       self.mass = 1
       self.distance = 0
       self.thrust = pg.mixer.Sound('res/thrust_audio.ogg')
       self.thrust.set_volume(0.07)


   def thruster(self, dx, dy):
       self.dx += dx
       self.dy += dy
       self.fuel -=2 
       self.thrust.play()

   def check_keys(self):
       keys = pg.key.get_pressed()

       if keys[pg.K_RIGHT]:
          self.thruster(dx=0.05, dy=0)
       elif keys[pg.K_LEFT]:
          self.thruster(dx=-0.05, dy=0)
       elif keys[pg.K_UP]:
          self.thruster(dx=0, dy=-0.05)
       elif keys[pg.K_DOWN]:
          self.thruster(dx=0, dy=0.05)

   def locate(self, planet):
       px, py = planet.x, planet.y
       dist_x = self.x - px
       dist_y = self.y - py
       planet_dir_radians = math.atan2(dist_x, dist_y)
       self.heading = planet_dir_radians * 180 / math.pi
       self.heading -= 90
       self.distance = math.hypot(dist_x, dist_y)



 

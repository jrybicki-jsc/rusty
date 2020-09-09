import sys
import math
import random
import pygame as pg

pg.init()

BLACK = (0, 0, 0)
WHITE = (255, 255, 255)
LT_GRAY = (180, 180, 180)
GRAY  = (120, 120, 120)
DK_GRAY = (80, 80, 80)

class Particle(pg.sprite.Sprite):
    gases_colors = {'SO2': LT_GRAY, 'CO2': GRAY, 'H2S': DK_GRAY, 'H2O': WHITE}

    VENT_LOCATION = (320, 300)
    IO_SURFACE_Y = 308
    VELOCITY_SO2 = 8
    GRAVITY = 0.5

    vel_scalar = {'SO2': 1, 'CO2': 1.45, 'H2S': 1.9, 'H2O': 3.6 } 

    def __init__(self, screen, background):
        super().__init__()
        self.screen = screen 
        self.background = background
        self.image = pg.Surface((4, 4))
        self.rect = self.image.get_rect()
        self.gas = random.choice(list(Particle.gases_colors.keys()))
        self.gas = 'H2S'
        self.color = Particle.gases_colors[self.gas]
        self.vel = Particle.VELOCITY_SO2 * Particle.vel_scalar[self.gas]
        self.x, self.y = Particle.VENT_LOCATION
        self.vector()

    def vector(self):
        orient = random.uniform(60, 120)
        radians = math.radians(orient)
        self.dx = self.vel * math.cos(radians)
        self.dy = -self.vel * math.sin(radians)

    def update(self):
        self.dy += Particle.GRAVITY
        pg.draw.line(self.background, self.color, (self.x, self.y), (self.x + self.dx, self.y + self.dy))
        r, g, b = self.color
        if (max(self.color) < 255):
            self.color = (r+5, g+5, b+5)

        self.x += self.dx
        self.y += self.dy
        if self.x < 0 or self.x > self.screen.get_width():
           self.kill()
        if self.y < 0 or self.y > Particle.IO_SURFACE_Y:
            self.kill()



def main():
     screen = pg.display.set_mode((639, 360))
     pg.display.set_caption('Io Volcano Simulator')
     background = pg.image.load('tvashtar_plume.gif')
     screen.blit(background, (0, 0))
     
     legend_font = pg.font.SysFont('None', 24)
     water_label = legend_font.render('---H2O', True, WHITE, BLACK)
     h2s_label = legend_font.render('---H2S', True, DK_GRAY, BLACK)
     co2_label = legend_font.render('---CO2', True, GRAY, BLACK)
     so2_label = legend_font.render('---SO2/S2', True, LT_GRAY, BLACK)

     particles = pg.sprite.Group()
     clock = pg.time.Clock()
     while True:
          clock.tick(25)
          particles.add(Particle(screen, background))
          for event in pg.event.get():
              if event.type == pg.QUIT:
                 pg.quit()
                 sys.exit()
          screen.blit(background, (0, 0))
          screen.blit(water_label, (40, 20))
          screen.blit(h2s_label, (40, 40))
          screen.blit(co2_label, (40, 60))
          screen.blit(so2_label, (40, 80))

          particles.update()
          particles.draw(screen)

          pg.display.flip()

if __name__ == "__main__":
      main()

     







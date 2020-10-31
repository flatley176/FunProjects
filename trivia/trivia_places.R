rm(list=ls())
library(ggplot2)
library(sf)
theme_set(theme_bw())
library("rnaturalearth")
library("rnaturalearthdata")
library(ggmap)
library(glue)
world <- ne_countries(scale = "medium", returnclass = "sf")
class(world)

sites <- data.frame(latitude = c(42.5, 38.0), 
                     longitude = c(23.5, 23.6))
ggplot(data = world) +
  geom_sf() +
  geom_point(data = sites, aes(x = longitude, y = latitude), size = 4, shape = 23, fill = "darkred") +
  coord_sf(xlim = c(10,30), ylim = c(35,44), expand = FALSE)
